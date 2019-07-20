use std::collections::HashMap;
use std::path::Path;
use crate::gl_wrapper::vao::VAO;
use crate::gl_wrapper::vbo::VBO;
use crate::gl_wrapper::ebo::EBO;
use crate::gl_wrapper::texture_2d::Texture2D;
use std::cell::RefCell;
use crate::ecs::components::{self, MeshRenderer};
use crate::shaders::diffuse::DiffuseShader;
use tobj::{Material, Mesh};
use nalgebra_glm::vec3;
use crate::utils;
use crate::utils::ToVec3;

#[derive(Default)]
pub struct TextureCache {
    textures: RefCell<HashMap<String, Texture2D>>
}

impl TextureCache {
    pub fn get(&self, id: &str) -> Texture2D {
        let mut textures = self.textures.borrow_mut();
        match textures.get(id) {
            Some(t) => t.clone(),
            None => {
                let t = Texture2D::new();
                textures.insert(id.into(), t.clone());
                t.bind().fill(id);
                t
            }
        }
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

        // Normals
        let vbo_normals = VBO::new();
        vbo_normals.bind().fill(&first_model.mesh.normals);
        println!("Len norm: {}", &first_model.mesh.normals.len());

        vao.set_attribute((2, 3, gl::FLOAT, std::mem::size_of::<f32>()));

        // TODO TexCoords

        // TODO Cache the mesh
        let mesh = components::Mesh {
            vao,
            positions: first_model.mesh.positions.clone(),
            indices: first_model.mesh.indices.clone(),
            normals: first_model.mesh.normals.clone(),
            texcoords: first_model.mesh.texcoords.clone()
        };

        let material = {
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
                    components::Material {
                        shader: Box::new(DiffuseShader::new_without_textures(
                          material.diffuse.to_vec3(),
                            material.specular.to_vec3(),
                            1.0.to_vec3(),
                            1.0,
                            0.0,
                            material.shininess
                        ))
                    }
                } else {
                    // TODO load textures and material
                    components::Material {
                        shader: Box::new(DiffuseShader::default())
                    }
                }
            } else {
                components::Material {
                    shader: Box::new(DiffuseShader::default())
                }
            }
        };

        MeshRenderer {
            mesh,
            material
        }
    }
}