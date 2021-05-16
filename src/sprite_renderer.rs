use glow::*;
use std::ops::Drop;
use std::rc::Rc;

use crate::shader::Shader;
use crate::texture::Texture;

pub struct SpriteRenderer {
    gl : Rc<glow::Context>,
    vao: glow::Buffer,
    vbo: glow::Buffer,
    shader: Rc<Shader>, 
}

impl SpriteRenderer {
    /// constructor, expects a filepath to a 3D model.
    pub fn new(  gl : Rc<Context>, shader : Rc<Shader> ) -> Self {

        unsafe {
            let vao  = gl.create_vertex_array().expect("Create VAO");
            let vbo = gl.create_buffer().expect("Create VBO");

            gl.bind_vertex_array( Some(vao) );

            let positions : [f32;24] = [
                // pos      // tex
                0.0, 1.0, 0.0, 1.0,
                1.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 
            
                0.0, 1.0, 0.0, 1.0,
                1.0, 1.0, 1.0, 1.0,
                1.0, 0.0, 1.0, 0.0
            ];

            // upload data
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo) );
            let u8_buffer = bytemuck::cast_slice(&positions);
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, u8_buffer, glow::STATIC_DRAW);

            // set position/vertex attribute (0) as vec4
            gl.vertex_attrib_pointer_f32(
                0,
                4,
                glow::FLOAT,
                false,
                std::mem::size_of::<f32>() as i32 * 4,
                std::mem::size_of::<f32>() as i32 * 0);
            gl.enable_vertex_attrib_array(0);

            Self {
                gl,
                vao,
                vbo,
                shader,
            }
        }
    }
    pub fn draw(&self, texture: &Texture,  position: glm::Vec2, size: glm::Vec2, rotate : f32, color:glm::Vec3) {
        self.shader.use_program();
        let mut model = glm::translate(&glm::Mat4::identity(), &glm::vec3(position.x,position.y,0.0f32) );

        // unsafe {
        //     self.gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);
        // }
        model = glm::translate(&model, &glm::vec3(0.5f32 * size.x, 0.5f32 * size.y, 0.0f32)); 
        model = glm::rotate(&model, rotate.to_radians(), &glm::vec3(0.0f32, 0.0f32, 1.0f32)); 
        model = glm::translate(&model, &glm::vec3(-0.5f32 * size.x, -0.5f32 * size.y, 0.0f32));
        model = glm::scale(&model, &glm::vec3(size.x, size.y, 1.0f32)); 
  
        self.shader.set_uniform_mat4("model", &model);
        self.shader.set_uniform_vec3("spriteColor", &color);
  
        unsafe {
            self.gl.active_texture(glow::TEXTURE0);
            texture.bind();
            self.gl.bind_vertex_array( Some(self.vao));
            self.gl.draw_arrays(glow::TRIANGLES, 0, 6);
            self.gl.bind_vertex_array( None);
        }
    }
}

impl Drop for SpriteRenderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.vao);
            self.gl.delete_buffer(self.vbo);
        }
    }
}
