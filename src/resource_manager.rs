/*******************************************************************
** This code is part of Breakout.
**
** Breakout is free software: you can redistribute it and/or modify
** it under the terms of the CC BY 4.0 license as published by
** Creative Commons, either version 4 of the License, or (at your
** option) any later version.
******************************************************************/

use std::{rc::Rc};
use std::cell::RefCell;
use std::collections::HashMap;
use super::texture::Texture;
use super::shader::Shader;

pub struct ResourceManager{
    gl : Rc<glow::Context>,    
    texture_pool : RefCell< HashMap<String, Rc<Texture>>>,
    shader_pool : RefCell< HashMap<String, Rc<Shader>>>,
}

impl ResourceManager {

    pub fn new(gl : Rc<glow::Context>) -> Self {
        Self {
            gl,
            texture_pool : RefCell::new( HashMap::new()),
            shader_pool : RefCell::new( HashMap::new()),
        }
    }

    pub fn get_gl(&self) -> Rc<glow::Context> {
        self.gl.clone()
    }

    pub fn load_texture( &self,  img_file_name :&str, name: String ) ->Rc<Texture> {

        if let Some(rc_texture) = self.texture_pool.borrow().get(&name) {
            return rc_texture.clone();
        }

        println!("Loading image: {}", img_file_name);
        //let img = image::open(img_file_name).unwrap().flipv().into_rgba8();
        let img = image::open(img_file_name).unwrap().into_rgba8();

        let (img_w, img_h) = img.dimensions();
        let raw_img = img.into_raw();

        let rc_texture = Rc::new( Texture::new(self.gl.clone(), img_w, img_h, raw_img) );
        self.texture_pool.borrow_mut().insert(name, rc_texture.clone());
        println!("Texture inserted into pool .. ");
        rc_texture
    }
    
    pub fn load_shader( &self, vx_shader_path:&str, fg_shader_path:&str, name: String )-> Rc<Shader> {

        let vx_shader = std::fs::read_to_string(vx_shader_path).expect(format!("Failed to read file: {}",vx_shader_path).as_str());
        let fg_shader = std::fs::read_to_string(fg_shader_path).expect(format!("Failed to read file: {}",fg_shader_path).as_str());
        let rc_shader = Rc::new( Shader::new(self.gl.clone(), vx_shader.as_str(), fg_shader.as_str()));
        self.shader_pool.borrow_mut().insert(name, rc_shader.clone() );
        rc_shader
    }

    pub fn get_texture(&self, name: String)-> Rc<Texture> {
        self.texture_pool.borrow().get(&name).unwrap().clone()
    }    

    pub fn get_shader(&self, name: String)-> Rc<Shader> {
        self.shader_pool.borrow().get(&name).unwrap().clone()
    }    
}