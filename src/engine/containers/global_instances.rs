use std::collections::HashMap;
use std::path::Path;
use crate::gl_wrapper::vao::VAO;
use crate::gl_wrapper::vbo::VBO;
use crate::gl_wrapper::ebo::EBO;
use crate::gl_wrapper::texture_2d::Texture2D;
use std::cell::{RefCell};
use crate::ecs::components::*;
use crate::shaders::diffuse::{self, DiffuseData, PixelData};
use tobj;
use crate::utils::ToVec3;
use std::sync::{Arc, Weak};

pub static CONTAINER: state::Container = state::Container::new();

#[derive(Default)]
pub struct TextureCache {
    textures: RefCell<HashMap<String, Weak<Texture2D>>>,
    meshes: RefCell<HashMap<String, Weak<Mesh>>>
}

impl TextureCache {
    pub fn get_texture(&self, id: &str) -> Arc<Texture2D> {
        let update_tex = || {
            let t = Arc::new(Texture2D::new());
            self.textures.borrow_mut().insert(id.into(), Arc::downgrade(&t));
            t.bind().fill(id);
            t
        };

        let texture = self.textures.borrow().get(id).cloned();
        match texture {
            Some(tex) => {
                let tex = tex.upgrade();
                match tex {
                    Some(tex) => tex,
                    None => {
                        update_tex()
                    }
                }
            },
            None => {
                update_tex()
            }
        }
    }

    pub fn get_mesh(&self, id: &str) -> Option<Arc<Mesh>> {
        self.meshes.borrow().get(id).and_then(|m| m.upgrade())
    }

    pub fn insert_mesh(&self, id: String, mesh: &Arc<Mesh>) {
        self.meshes.borrow_mut().insert(id,Arc::downgrade(mesh));
    }
}

#[derive(Default)]
pub struct ModelLoader;

impl ModelLoader {
    fn load_mesh(model: &tobj::Model) -> Mesh {
        let vao = VAO::new();
        vao.bind();

        // Upload indices
        let ebo = EBO::new();
        ebo.bind().fill(&model.mesh.indices);

        // Upload positions
        let vbo_positions = VBO::new();
        vbo_positions.bind().fill(&model.mesh.positions);
        vao.set_attribute((0, 3, gl::FLOAT, std::mem::size_of::<f32>()));

        // TODO Maybe there aren't any normals/texture coords
        // Upload texture coordinates
        let vbo_texcoords = VBO::new();
        vbo_texcoords.bind().fill(&model.mesh.texcoords);
        vao.set_attribute((1, 2, gl::FLOAT, std::mem::size_of::<f32>()));

        // Upload normals
        let vbo_normals = VBO::new();
        vbo_normals.bind().fill(&model.mesh.normals);
        vao.set_attribute((2, 3, gl::FLOAT, std::mem::size_of::<f32>()));

        Mesh {
            vao,
            positions: model.mesh.positions.clone(),
            indices: model.mesh.indices.clone(),
            normals: model.mesh.normals.clone(),
            texcoords: model.mesh.texcoords.clone()
        }
    }

    fn load_material(obj_path: &Path, model: &tobj::Model, materials: &[tobj::Material]) -> Material {
        let mut shader_data = DiffuseData::default();

        let material = model.mesh.material_id.map(|id| &materials[id]);
        let shader_data = if let Some(material) = material {
            trace!("Loading material {}", material.name);
            debug!("material.Ka = {:?}", material.ambient);
            debug!("material.Kd = {:?}", material.diffuse);
            debug!("material.Ks = {:?}", material.specular);
            debug!("material.d = {}", material.dissolve);
            debug!("material.map_Ka = {}", material.ambient_texture);
            debug!("material.map_Kd = {}", material.diffuse_texture);
            debug!("material.map_Ks = {}", material.specular_texture);
            debug!("material.map_Ns = {}", material.normal_texture);
            debug!("material.map_d = {}", material.dissolve_texture);

            let texture_cache = CONTAINER.get_local::<TextureCache>();
            let files_path = dbg!(obj_path.parent().unwrap());

            // Load diffuse
            let diffuse = if material.diffuse_texture.is_empty() {
                warn!("No diffuse texture");
                PixelData::Color(material.diffuse.to_vec3())
            } else {
                // TODO load textures and material
                let diffuse_path = files_path.join(&material.diffuse_texture);
                PixelData::Texture(texture_cache.get_texture(diffuse_path.to_str().unwrap()))
            };

            // Load specular
            let specular = if material.specular_texture.is_empty() {
                warn!("No specular texture");
                PixelData::Color(material.specular.to_vec3())
            } else {
                let specular_path = files_path.join(&material.specular_texture);
                PixelData::Texture(texture_cache.get_texture(specular_path.to_str().unwrap()))
            };

            // Load normal
            let normal = if material.normal_texture.is_empty() {
                warn!("No normal texture");
                None
            } else {
                let normal_path = files_path.join(&material.normal_texture);
                Some(texture_cache.get_texture(normal_path.to_str().unwrap()))
            };

            let shininess = material.shininess;

            DiffuseData {
                diffuse,
                specular,
                normal,
                shininess
            }
        } else {
            warn!("Model {} doesn't have a material", model.name);
            warn!("Loading default material");
            DiffuseData::default()
        };

        Material {
            shader_data: Box::new(shader_data)
        }
    }

    pub fn load(&self, filename: &str) -> MeshRenderer {
        let obj_path = Path::new(filename);
        let obj = tobj::load_obj(&obj_path);
        assert!(obj.is_ok());

        let (models, materials) = obj.unwrap();
        let first_model = models.first().unwrap();

        let mesh = Self::load_mesh(&first_model);
        let material = Self::load_material(&obj_path, &first_model, &materials);

        MeshRenderer {
            mesh: Arc::new(mesh),
            material: Arc::new(material)
        }
    }
}