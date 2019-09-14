use std::collections::HashMap;
use std::path::Path;
use crate::gl_wrapper::vao::VAO;
use crate::gl_wrapper::vbo::{VBO, VertexAttribute};
use crate::gl_wrapper::ebo::EBO;
use crate::gl_wrapper::texture_2d::Texture2D;
use std::cell::{RefCell};
use crate::ecs::components::*;
use crate::shaders::diffuse::{DiffuseData, PixelData};
use tobj;
use crate::utils::ToVec3;
use std::sync::{Arc, Weak};
use crate::gl_wrapper::{BufferUpdateFrequency, TextureFormat};
use image::GenericImageView;
use std::borrow::BorrowMut;

pub static CONTAINER: state::Container = state::Container::new();

#[derive(Default)]
pub struct TextureCache {
    textures: RefCell<HashMap<String, Weak<Texture2D>>>,
    meshes: RefCell<HashMap<String, Weak<Mesh>>>
}

impl TextureCache {
    pub fn get_texture(&self, id: &str) -> Arc<Texture2D> {
        let update_tex = || {
            let mut t = Texture2D::new();
            let img = image::open(id);
            let img = match img {
                Ok(img) => img.flipv(),
                Err(err) => panic!("Filename: {}, error: {}", id, err.to_string())
            };

            let format = match img.color() {
                image::RGB(8) => TextureFormat::RGB,
                image::RGBA(8) => TextureFormat::RGBA,
                _ => panic!("Texture format not supported")
            };

            t.allocate(format, img.width(), img.height(), 8);
            t.update(0, 0, &img);

            let t = Arc::new(t);
            self.textures.borrow_mut().insert(id.into(), Arc::downgrade(&t));
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
        // TODO Maybe there aren't any normals/texture coords

        let ebo = {
            let mut ebo = EBO::new();
            ebo.with(&model.mesh.indices, BufferUpdateFrequency::Never);
            ebo
        };

        let vao = VAO::new(
            &[
                // positions
                {
                    let mut vbo = VBO::new(vec![
                        VertexAttribute { index: 0, components: 3 }
                    ]);
                    vbo.with(&model.mesh.positions, BufferUpdateFrequency::Never);
                    vbo
                },
                // texcoords
                {
                    let mut vbo = VBO::new(vec![
                        VertexAttribute { index: 1, components: 2 }
                    ]);
                    vbo.with(&model.mesh.texcoords, BufferUpdateFrequency::Never);
                    vbo
                },
                // normals
                {
                    let mut vbo = VBO::new(vec![
                        VertexAttribute { index: 2, components: 3 }
                    ]);
                    vbo.with(&model.mesh.normals, BufferUpdateFrequency::Never);
                    vbo
                }
            ],
            Some(&ebo)
        );

        Mesh {
            vao,
            positions: model.mesh.positions.clone(),
            indices: model.mesh.indices.clone(),
            normals: model.mesh.normals.clone(),
            texcoords: model.mesh.texcoords.clone()
        }
    }

    fn load_material(obj_path: &Path, model: &tobj::Model, materials: &[tobj::Material]) -> Material {
        let _shader_data = DiffuseData::default();

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