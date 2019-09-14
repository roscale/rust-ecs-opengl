use crate::gl_wrapper::vao::VAO;
use crate::gl_wrapper::vbo::{VBO, VertexAttribute};
use std::collections::HashMap;
use crate::gl_wrapper::{EBO, BufferUpdateFrequency};

pub struct PredefinedShapes {
    pub shapes: HashMap<&'static str, VAO>,
}

impl Default for PredefinedShapes {
    fn default() -> Self {
        let mut shapes = HashMap::new();
        shapes.insert("unit_cube", parallelepiped());
        shapes.insert("unit_quad", quad());

        PredefinedShapes {
            shapes
        }
    }
}

fn parallelepiped() -> VAO {
    let positions = [
        // positions
        -1.0f32,  1.0, -1.0,
        -1.0, -1.0, -1.0,
        1.0, -1.0, -1.0,
        1.0, -1.0, -1.0,
        1.0,  1.0, -1.0,
        -1.0,  1.0, -1.0,

        -1.0, -1.0,  1.0,
        -1.0, -1.0, -1.0,
        -1.0,  1.0, -1.0,
        -1.0,  1.0, -1.0,
        -1.0,  1.0,  1.0,
        -1.0, -1.0,  1.0,

        1.0, -1.0, -1.0,
        1.0, -1.0,  1.0,
        1.0,  1.0,  1.0,
        1.0,  1.0,  1.0,
        1.0,  1.0, -1.0,
        1.0, -1.0, -1.0,

        -1.0, -1.0,  1.0,
        -1.0,  1.0,  1.0,
        1.0,  1.0,  1.0,
        1.0,  1.0,  1.0,
        1.0, -1.0,  1.0,
        -1.0, -1.0,  1.0,

        -1.0,  1.0, -1.0,
        1.0,  1.0, -1.0,
        1.0,  1.0,  1.0,
        1.0,  1.0,  1.0,
        -1.0,  1.0,  1.0,
        -1.0,  1.0, -1.0,

        -1.0, -1.0, -1.0,
        -1.0, -1.0,  1.0,
        1.0, -1.0, -1.0,
        1.0, -1.0, -1.0,
        -1.0, -1.0,  1.0,
        1.0, -1.0,  1.0
    ];

    VAO::new(
        &[
            {
                let mut vbo = VBO::new(vec![
                    VertexAttribute { index: 0, components: 3 }
                ]);
                vbo.with(&positions, BufferUpdateFrequency::Never);
                vbo
            }
        ],
        None
    )
}

fn quad() -> VAO {
    let quad_pos = [
        -1.0f32, 1.0,
        -1.0, -1.0,
        1.0, -1.0,
        -1.0, 1.0,
        1.0, -1.0,
        1.0, 1.0,
    ];

    let quad_tex = [
        0.0f32, 1.0,
        0.0, 0.0,
        1.0, 0.0,
        0.0, 1.0,
        1.0, 0.0,
        1.0, 1.0
    ];

    VAO::new(
        &[
            {
                let mut vbo = VBO::new(vec![
                    VertexAttribute { index: 0, components: 2 }
                ]);
                vbo.with(&quad_pos, BufferUpdateFrequency::Never);
                vbo
            },
            {
                let mut vbo = VBO::new(vec![
                    VertexAttribute { index: 1, components: 2 }
                ]);
                vbo.with(&quad_tex, BufferUpdateFrequency::Never);
                vbo
            }
        ],
        None
    )
}
