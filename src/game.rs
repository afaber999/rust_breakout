//use glow::*;
use std::{borrow::BorrowMut, rc::Rc};
use glutin::event::VirtualKeyCode;
use crate::game_object::GameObject;
use crate::sprite_renderer::SpriteRenderer;
use crate::resource_manager::ResourceManager;
use crate::texture::Texture;
use crate::game_level::GameLevel;

extern crate nalgebra_glm as glm;

#[derive(Debug,PartialEq)]
enum GameState {
    GameActive,
    GameMenu,
    GameWin
}

struct GlObjs{
    sprite_renderer  : SpriteRenderer,
    face_texture : Rc<Texture>,
    background_texture : Rc<Texture>,
    player : GameObject,
    game_levels : Vec<GameLevel>,
    level      : usize,  
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
            game_state : GameState::GameActive,
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
            self.resource_manager.get_gl(),
            sprite_shader,
        );

        let background_texture = self.resource_manager.load_texture(
            "resources/textures/background.jpg", 
            "background".into(),
        );

        let face_texture = self.resource_manager.load_texture(
            "resources/textures/awesomeface.png", 
            "face".into(),
        );


        let paddle_texture = self.resource_manager.load_texture(
            "resources/textures/paddle.png", 
            "paddle".into(),
        );

        const PLAYER_SIZE : glm::Vec2 = glm::Vec2::new(100.0, 20.0); 

        let player = GameObject::new(
            glm::vec2((self.width as f32 - PLAYER_SIZE.x) / 2.0, self.height as f32 - PLAYER_SIZE.y ),
            glm::vec2(100.0,20.0),
            glm::vec2(500.0,0.0),
            glm::vec3(1.0,1.0,1.0),
            0.0,
            paddle_texture, 
            false);

        let mut game_levels = Vec::new();

        let mut game_level = GameLevel::new(self.resource_manager.clone());
        game_level.load("levels/one.lvl" , self.width, self.height/2);
        game_levels.push(game_level);

        let mut game_level = GameLevel::new(self.resource_manager.clone());
        game_level.load("levels/two.lvl" , self.width, self.height/2);
        game_levels.push(game_level);

        let mut game_level = GameLevel::new(self.resource_manager.clone());
        game_level.load("levels/three.lvl" , self.width, self.height/2);
        game_levels.push(game_level);

        let mut game_level = GameLevel::new(self.resource_manager.clone());
        game_level.load("levels/four.lvl" , self.width, self.height/2);
        game_levels.push(game_level);


        self.globjs = Some( GlObjs {
            sprite_renderer,
            face_texture,
            background_texture,
            player,
            game_levels,
            level : 0,
        } );
    }

    pub fn process_input(&mut self, dt: f32, key : VirtualKeyCode) {

        if self.game_state != GameState::GameActive {
            return;
        }
        if let Some(objs) = self.globjs.borrow_mut() {
            let mut position = objs.player.get_position();
            let width_div_2 = objs.player.get_size().x as f32 / 2.0;

            let velocity = objs.player.get_velocity().x as f32 * dt;

            match key {
                VirtualKeyCode::Left | VirtualKeyCode::A    => {
                    position.x = ( position.x - velocity ).max(- width_div_2);
                    objs.player.set_position(position);
                },
                VirtualKeyCode::Right | VirtualKeyCode::D    => {
                    position.x = ( position.x + velocity ).min(self.width as f32 - width_div_2);
                    objs.player.set_position(position);
                },
                _ => (),
            }
        }
    }

    pub fn update( _dt: f32) {
        
    }

    pub fn render( &mut self, _dt: f32) {
        if let Some(objs) = &self.globjs {
            // draw background
            objs.sprite_renderer.draw(
                objs.background_texture.as_ref(),
                glm::vec2(0.0, 0.0),
                glm::vec2(self.width as f32, self.height as f32),
                0.0,
                glm::vec3(1.0,  1.0, 1.0)

            );

            let position = glm::vec2(200.0f32, 200.0f32);
            let rotate = 45.0f32;
            let size = glm::vec2(300.0f32, 400.0f32);
            let color = glm::vec3(0.0f32, 1.0f32,0.0f32);

            objs.sprite_renderer.draw(
                objs.face_texture.as_ref(),
                position,
                size,
                rotate,
                color
            );
            objs.game_levels[objs.level].draw(&objs.sprite_renderer);
            objs.player.draw(&objs.sprite_renderer);
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {

    }
} 
