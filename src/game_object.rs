use std::{rc::Rc};
use crate::sprite_renderer::SpriteRenderer;
use crate::texture::Texture;

extern crate nalgebra_glm as glm;

#[derive(Debug)]
pub struct GameObject {
    position : glm::Vec2,
    size : glm::Vec2,
    velocity : glm::Vec2,
    color : glm::Vec3,
    rotation : f32,
    solid: bool,
    texture : Rc<Texture>,
    destroyed : bool,
}

impl GameObject {
    pub fn new( 
        position : glm::Vec2,
        size : glm::Vec2,
        velocity : glm::Vec2,
        color : glm::Vec3,
        rotation : f32,
        texture : Rc<Texture>,
        solid: bool ) -> Self {
        Self {
            position,
            size,
            velocity,
            color,
            rotation,
            solid,
            texture,
            destroyed : false,
        }
    }
    pub fn draw(&self, renderer : &SpriteRenderer ) {
        renderer.draw(
            self.texture.as_ref(),
            self.position,
            self.size,
            self.rotation,
            self.color);
    }

    pub fn is_destroyed(&self) -> bool {
        self.destroyed
    }

    pub fn is_solid(&self) -> bool {
        self.solid
    }
    
    pub fn set_destroyed(&mut self)  {
        self.destroyed = true;
    }

    pub fn get_position(&self) -> glm::Vec2 {
        self.position
    }

    pub fn get_size(&self) -> glm::Vec2 {
        self.size
    }

    pub fn get_velocity(&self) -> glm::Vec2 {
        self.velocity
    }

    pub fn set_position(&mut self, position : glm::Vec2)  {
        self.position = position;
    }

    pub fn set_velocity(&mut self, velocity : glm::Vec2)  {
        self.velocity = velocity;
    }

    pub fn check_collision( &self, check_obj : &GameObject) -> bool {
        let p1 = self.position;
        let s1=self.size;
        let p2 = check_obj.position;
        let s2 = check_obj.size;

        let col_x = 
            p1.x + s1.x >= p2.x &&
            p2.x + s2.x  >= p1.x;
        let col_y = 
            p1.y + s1.y >= p2.y &&
            p2.y + s2.y  >= p1.y;
        return col_x && col_y;
    }
}
