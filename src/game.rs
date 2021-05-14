//use glow::*;
use std::{rc::Rc};
use crate::sprite_renderer::SpriteRenderer;
use crate::resource_manager::ResourceManager;
use crate::shader::Shader;
use crate::texture::Texture;

extern crate nalgebra_glm as glm;


enum GameState {
    GameActive,
    GameMenu,
    GameWin
}

struct GlObjs{
    sprite_renderer  : SpriteRenderer,
    sprite_shader  : Rc<Shader>,
    face_texture : Rc<Texture>,
}

pub struct Game {
    resource_manager : Rc<ResourceManager>,
    game_state: GameState,
    keys : [bool; 1024],
    width : u32,
    height : u32,
    globjs : Option<GlObjs>,
}

impl Game {
    pub fn new(resource_manager : Rc<ResourceManager>, width:u32, height: u32) -> Self {
        Self {
            resource_manager,
            game_state : GameState::GameMenu,
            keys : [false;1024],
            width,
            height,
            globjs : None,
        }
    }

    pub fn init(&mut self) 
    {
        let sprite_shader = self.resource_manager.load_shader(  
            "src/shaders/sprite.vs",
            "src/shaders/sprite.fs",
            "sprite".into()
        );

        let projection = glm::ortho(
            0.0, 
            self.width as f32,
            self.height as f32,
            0.0,
            -1.0,
            1.0);
      
        // select shader before setting uniforms
        sprite_shader.use_program();
        // AF TODO REMOVE let projection = glm::Mat4::identity();
        sprite_shader.set_uniform_i32("image", 0);
        sprite_shader.set_uniform_mat4("projection", &projection);

        let sprite_renderer = SpriteRenderer::new(
            self.resource_manager.clone());

        let face_texture = self.resource_manager.load_texture(
            "resources/textures/awesomeface.png", 
            "face".into(),
        );

        self.globjs = Some( GlObjs {
            sprite_shader,
            sprite_renderer,
            face_texture,
        } );
    }

    pub fn process_input( _dt: f32) {

    }

    pub fn update( _dt: f32) {
        
    }

    pub fn render( &mut self, _dt: f32) {
        if let Some(objs) = &self.globjs {

            let position = glm::vec2(200.0f32, 200.0f32);
            let rotate = 45.0f32;
            let size = glm::vec2(300.0f32, 400.0f32);
            let color = glm::vec3(0.0f32, 1.0f32,0.0f32);

            objs.sprite_renderer.draw(
                objs.sprite_shader.as_ref(),
                objs.face_texture.as_ref(),
                position,
                size,
                rotate,
                color
            );
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {

    }
} 
