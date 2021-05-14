use glow::*;
use std::ops::Drop;
use std::rc::Rc;


#[derive(Debug)]
pub struct Texture {
    gl : Rc<glow::Context>,
    texture: glow::Texture,
}

impl Texture {
    pub fn new( gl : Rc<glow::Context>, img_w:u32, img_h: u32, img_raw:Vec<u8> ) ->Self {

        let texture = unsafe {
            // load and create a texture
            // -------------------------        
            let texture = gl.create_texture().expect("Create a texture" );

            // bind texture, all upcoming GL_TEXTURE_2D operations now have effect on this texture object
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));

            // set the texture wrapping & repeat parameters
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);

            // set texture filtering parameters
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);

        
            // Give the image to OpenGL
            gl.tex_image_2d(glow::TEXTURE_2D,
                                0, 
                                glow::RGBA as i32, 
                                img_w as i32, 
                                img_h as i32,
                                0, 
                                glow::RGBA, 
                                glow::UNSIGNED_BYTE,
                                Some(&img_raw) );

            gl.generate_mipmap(glow::TEXTURE_2D);
            texture
        };
        Self {
            gl,
            texture,
        }
    }
   
    pub fn bind(&self) {
        unsafe {
            self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture));
        }
    }
}


impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_texture(self.texture);
        }
    }
}
