use crate::gl_wrapper::*;
use std::collections::HashMap;
use std::path::Path;
use crate::containers::CONTAINER;
use std::ffi::c_void;
use crate::shaders::voxel::VoxelShader;

#[derive(Copy)]
#[derive(Clone)]
pub struct Block {
    pub id: &'static str
}

impl Block {
    fn air() -> Self {
        Block { id: "air" }
    }
}

pub struct BlockCatalog {
    pub blocks_texture_atlas: Texture2D,
    pub block_types: HashMap<String, (u32, u32)>
}

pub struct Chunk {
    blocks: [Block; 256],
    mesh: VAO,
    positions: VBO,
    tex_coords: VBO,
    indices: EBO,
    indices_len: u32,

    lightmap: Texture2D
}

impl Chunk {
    pub fn new() -> Self {
        let positions = VBO::new(vec![
            VertexAttribute {
                index: 0,
                components: 2,
            },
        ]);

        let tex_coords = VBO::new(vec![
            VertexAttribute {
                index: 1,
                components: 2
            },
        ]);

        let indices = EBO::new();

        Chunk {
            blocks: [Block::air(); 256],
            mesh: VAO::new(&[positions.clone(), tex_coords.clone()], Some(&indices)),
            positions,
            tex_coords,
            indices,
            // TODO Put length inside buffer
            indices_len: 0,
            lightmap: {
                let mut lightmap = Texture2D::new();
                lightmap.allocate(TextureFormat::RGB, 16, 16, 1);
                lightmap
            }
        }
    }

    pub fn add_block(&mut self, x: usize, y: usize, block: Block) {
        self.blocks[y * 16 + x] = block;
        self.regen_mesh();
        // TODO Modify lightmap if it's a light source
    }

    pub fn remove_block(&mut self, x: usize, y: usize) {
        self.blocks[y * 16 + x] = Block::air();
        self.regen_mesh();
        // TODO Regen lightmap if it's a light source
    }

    fn regen_mesh(&mut self) {
        let block_catalog = CONTAINER.get_local::<BlockCatalog>();
        let mut vec_positions = Vec::<f32>::new();
        let mut vec_tex_coords = Vec::<f32>::new();
        let mut vec_indices = Vec::<u32>::new();

        let mut i = 0;
        for y in 0..16 {
            for x in 0..16 {
                let block_id = self.blocks[16 * y + x].id;
                if block_id != "air" {
                    let (tex_x, tex_y) = block_catalog.block_types.get(block_id).cloned().unwrap();
                    let (tex_x, tex_y) = (tex_x as f32 / 128.0, tex_y as f32 / 128.0);
                    let (tex_w, tex_h) = (16.0 / 128.0, 16.0 / 128.0);

                    println!("Regen i x y {} {} {}", i, x, y);
                    let (tile, tex_coords, indices) = gen_tile(
                        i as u32,
                        x as f32,
                        y as f32,
                        1.0,
                        1.0,
                        tex_x,
                        tex_y,
                        tex_w,
                        tex_h
                    );
                    i += 1;

                    vec_positions.extend_from_slice(&tile);
                    vec_tex_coords.extend_from_slice(&tex_coords);
                    vec_indices.extend_from_slice(&indices);
                }
            }
        }

        self.indices_len = vec_indices.len() as u32;
        gl_call!(gl::NamedBufferData(self.positions.id, 4 * vec_positions.len() as isize, vec_positions.as_ptr() as *mut c_void, gl::STATIC_DRAW));
        gl_call!(gl::NamedBufferData(self.tex_coords.id, 4 * vec_tex_coords.len() as isize, vec_tex_coords.as_ptr() as *mut c_void, gl::STATIC_DRAW));
        gl_call!(gl::NamedBufferData(self.indices.id, 4 * vec_indices.len() as isize, vec_indices.as_ptr() as *mut c_void, gl::STATIC_DRAW));
    }

    fn add_light_source(&mut self) {
        unimplemented!();
    }

    fn remove_light_source(&mut self) {
        unimplemented!();
    }
}

pub struct ResourceManager;

impl ResourceManager {
    pub fn gen_blocks_texture_atlas(dir: &Path) -> BlockCatalog {
        let mut atlas = Texture2D::new();
        let (atlas_width, atlas_height) = (128, 128);
        atlas.allocate(TextureFormat::RGBA, atlas_width, atlas_height, 1);

        let mut blocks = HashMap::new();

        assert!(dir.is_dir());
        println!("Entries:");
        let mut x = 0;
        let mut y = 0;
        for entry in dir.read_dir().expect("Unable to read textures directory") {
            if let Ok(entry) = entry {
                if !entry.path().is_file() {
                    continue;
                }
                assert!(y < atlas_height, "Texture atlas is full!");

                // Update atlas
                let img = image::open(entry.path());
                let img = match img {
                    Ok(img) => img.flipv(),
                    Err(err) => panic!("Could not open block texture image"),
                };

                atlas.update(x, y, &img);

                // Save name and texture coordinates
                let block_name = entry.path().file_stem().unwrap().to_str().unwrap().to_owned();
                blocks.insert(block_name, (x, y));

                x += 16;
                if x / atlas_width > 0 {
                    x %= atlas_width;
                    y += 16
                }
            }
        }

        for (key, value) in blocks.iter() {
            println!("Name: {}, tex: {},{}", key, value.0, value.1);
        }

        BlockCatalog { blocks_texture_atlas: atlas, block_types: blocks }
    }
}

pub struct VoxelWorld {
    pub chunk: Chunk,
}

impl VoxelWorld {
    pub fn new() -> Self {
        VoxelWorld { chunk: Chunk::new() }
    }

    pub fn place_some_blocks(&mut self) {
        let blocks = CONTAINER.get_local::<BlockCatalog>();
        let ids: Vec<&String> = blocks.block_types.keys().collect();

        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();

        for y in 0..16 {
            for x in 0..16 {
                self.chunk.add_block(x, y, Block { id: ids.choose(&mut rng).unwrap() });
            }
        }

        self.chunk.remove_block(7, 7);
        self.chunk.remove_block(7, 8);
        self.chunk.remove_block(8, 8);
    }

    pub fn render(&self) {
        let block_catalog = CONTAINER.get_local::<BlockCatalog>();
        block_catalog.blocks_texture_atlas.activate(0);

        let shader = CONTAINER.get_local::<VoxelShader>();
        shader.bind();
        self.chunk.mesh.bind();

//        println!("LEN {}", self.chunk.indices_len);

        gl_call!(gl::DrawElements(gl::TRIANGLES,
                                  self.chunk.indices_len as i32,
                                  gl::UNSIGNED_INT, std::ptr::null()));
    }
}

fn gen_tile(
    mut i: u32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    tex_x: f32,
    tex_y: f32,
    tex_w: f32,
    tex_h: f32
) -> ([f32; 8], [f32; 8], [u32; 6]) {
    let tile = [
        x + width * 0.0f32, y + height * 0.0,
        x + width * 0.0, y + height * -1.0,
        x + width * 1.0, y + height * -1.0,
        x + width * 1.0, y + height * 0.0,
    ];

    let tex_coords = [
        tex_x, tex_y + tex_h,
        tex_x, tex_y,
        tex_x + tex_w, tex_y,
        tex_x + tex_w, tex_y + tex_h
    ];

    i *= 4;
    let indices = [i + 0, i + 1, i + 2, i + 2, i + 3, i + 0];

    (tile, tex_coords, indices)
}