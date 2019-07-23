use std::collections::HashMap;
use std::path::Path;
use crate::gl_wrapper::vao::VAO;
use crate::gl_wrapper::vbo::VBO;
use crate::gl_wrapper::ebo::EBO;
use crate::gl_wrapper::texture_2d::Texture2D;
use std::cell::{RefCell, RefMut};
use crate::ecs::components::{self, *};
use crate::shaders::diffuse::{self, DiffuseShader, DiffuseData};
use tobj;
use nalgebra_glm::vec3;
use crate::utils;
use crate::utils::ToVec3;
use std::sync::{Arc, Weak};
use tobj::Model;

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

        let texture = self.textures.borrow().get(id).map(|t| t.clone());
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
    pub fn load(&self, filename: &str) -> MeshRenderer {
        let obj = tobj::load_obj(&Path::new(filename));
        assert!(obj.is_ok());

        let (models, materials) = obj.unwrap();
        let first_model = models.first().unwrap();

        /// Mesh
        let texture_cache = CONTAINER.get_local::<TextureCache>();

        let mesh_name = format!("{}/{}", filename, first_model.name);
//        let mesh = texture_cache.get_mesh(&mesh_name).unwrap_or_else( || {
        let mesh = {
            println!("NEW MESH: {}", &mesh_name);

            let vao = VAO::new();
            vao.bind();

            // Indices
            let ebo = EBO::new();
            ebo.bind().fill(&first_model.mesh.indices);
            println!("Len ind: {}", &first_model.mesh.indices.len());

            // Positions
            let vbo_vertices = VBO::new();
            vbo_vertices.bind().fill(&first_model.mesh.positions);
            println!("Len pos: {}", &first_model.mesh.positions.len());

            vao.set_attribute((0, 3, gl::FLOAT, std::mem::size_of::<f32>()));

            // TODO Maybe there aren't any normals/texture coords

            // Texture coordinates
            let vbo_tex = VBO::new();
            vbo_tex.bind().fill(&first_model.mesh.texcoords);
            vao.set_attribute((1, 2, gl::FLOAT, std::mem::size_of::<f32>()));

            // Normals
            let vbo_normals = VBO::new();
            vbo_normals.bind().fill(&first_model.mesh.normals);
            println!("Len norm: {}", &first_model.mesh.normals.len());

            vao.set_attribute((2, 3, gl::FLOAT, std::mem::size_of::<f32>()));

            // TODO TexCoords

            // TODO Cache the mesh
            let mesh = Arc::new(Mesh {
                vao,
                positions: first_model.mesh.positions.clone(),
                indices: first_model.mesh.indices.clone(),
                normals: first_model.mesh.normals.clone(),
                texcoords: first_model.mesh.texcoords.clone()
            });
            dbg!(&mesh.texcoords);
//            texture_cache.insert_mesh(mesh_name, &mesh);
            mesh
        };

        let material = {
            let mut shader_data = DiffuseData::default();

            let material = first_model.mesh.material_id.map(|id| &materials[id]);
            if let Some(material) = material {
                println!("{}", material.name);
                println!("    material.Ka = ({}, {}, {})", material.ambient[0], material.ambient[1], material.ambient[2]);
                println!("    material.Kd = ({}, {}, {})", material.diffuse[0], material.diffuse[1], material.diffuse[2]);
                println!("    material.Ks = ({}, {}, {})", material.specular[0], material.specular[1], material.specular[2]);
                println!("    material.Ns = {}", material.shininess);
                println!("    material.d = {}", material.dissolve);
                println!("    material.map_Ka = {}", material.ambient_texture);
                println!("    material.map_Kd = {}", material.diffuse_texture);
                println!("    material.map_Ks = {}", material.specular_texture);
                println!("    material.map_Ns = {}", material.normal_texture);
                println!("    material.map_d = {}", material.dissolve_texture);
                if material.diffuse_texture.is_empty() {
                    shader_data = diffuse::DiffuseData::Colors {
                        diffuse_color: material.diffuse.to_vec3(),
                        specular_color: material.specular.to_vec3(),
                        shininess: material.shininess.clone()
                    };
                } else {
                    // TODO load textures and material
                    let diffuse_texture = texture_cache.get_texture(&material.diffuse_texture);
                    let specular_texture = texture_cache.get_texture(&material.specular_texture);
                    let normal_texture = texture_cache.get_texture(&material.normal_texture);
                    let shininess = material.shininess.clone();
                    shader_data = diffuse::DiffuseData::Textures {
                        diffuse_texture,
                        specular_texture,
                        normal_texture,
                        shininess
                    };
//                    shader_data = DiffuseData::default();
                }
            } else {
                shader_data = diffuse::DiffuseData::default();
            }

            Arc::new(Material {
                shader_data: Box::new(shader_data)
            })
        };

        MeshRenderer {
            mesh,
            material
        }
    }
}