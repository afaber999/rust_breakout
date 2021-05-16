
use std::rc::Rc;
use crate::texture::Texture;
use crate::game_object::GameObject;
use crate::sprite_renderer::SpriteRenderer;


pub struct BallObject {

    game_object: GameObject,
    radius: f32,
    stuck : bool,
}

impl BallObject {

    pub fn new( 
        position : glm::Vec2,
        radius   : f32,
        velocity : glm::Vec2,
        texture : Rc<Texture> ) -> Self {

        Self {
            game_object: GameObject::new(
                position,
                glm::vec2(radius * 2.0, radius * 2.0),
                velocity,
                glm::vec3(1.0, 1.0, 1.0),
                0.0,
                texture,
                false,
            ),
            radius,
            stuck : true,
        }
    }

    pub fn draw(&self, renderer : &SpriteRenderer ) {
        self.game_object.draw(renderer);
    }

    pub fn do_move( &mut self, dt : f32, window_width:u32) -> glm::Vec2 {
        let mut position = self.game_object.get_position();
        let size_x = self.game_object.get_size().x;

        if ! self.stuck
        {
            let mut velocity = self.game_object.get_velocity();

            // move the ball
            position = position + velocity.scale(dt);

            // check if outside window bounds; if so, reverse velocity
            // and restore at correct position
            if position.x <= 0.0
            {
                velocity.x = -velocity.x;
                position.x = 0.0;
            }
            else if position.x + size_x >= window_width as f32
            {
                velocity.x = -velocity.x;
                position.x = window_width as f32 - size_x;
            }
            
            if position.y <= 0.0
            {
                velocity.y = -velocity.y;
                position.y = 0.0;
            }

            self.game_object.set_velocity( velocity );
            self.game_object.set_position( position );

        }
        return position;
    }

    pub fn reset( &mut self, position : glm::Vec2,  velocity : glm::Vec2) {
        self.game_object.set_velocity( velocity );
        self.game_object.set_position( position );
    }    
}
