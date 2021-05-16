use std::rc::Rc;
use crate::game_object::GameObject;
use crate::sprite_renderer::SpriteRenderer;
use crate::resource_manager::ResourceManager;

pub struct GameLevel
{
    resource_manager : Rc<ResourceManager>,
    bricks : Vec<GameObject>,
}

impl GameLevel {
    pub fn new(resource_manager : Rc<ResourceManager>) ->Self {
        Self {
            resource_manager,
            bricks : Vec::new(),
        }
    }

    pub fn load(&mut self, level_path : &str, level_width:u32, level_height:u32) {

        self.bricks.clear();

        let mut tile_data = Vec::new();

        let content = std::fs::read_to_string(level_path).expect(format!("Failed to read file: {}",level_path).as_str());
        for line in content.lines() {
            
            let row : Vec<u32> = line.split_ascii_whitespace().map(|s| 
                u32::from_str_radix(s, 10).unwrap() ).collect();

            println!("LINE: {:?}", &row);
            tile_data.push(row);
        }
        self.create_bricks(tile_data,level_width, level_height);
    }

    fn create_bricks(&mut self, tile_data: Vec<Vec<u32>>, level_width:u32, level_height:u32) {
        
        let height  = tile_data.len();
        let width = tile_data[0].len();
   
        let unit_width = level_width as f32 / width as f32;
        let unit_height = level_height as f32 / height as f32;
        
        // initialize level tiles based on tileData		
        for y in 0..height {
            for x in 0..width {
                let color;
                let mut solid = false;

                match tile_data[y][x] {
                    1 => {
                        color = glm::vec3(0.8, 0.8, 0.7);
                        solid = true;
                    },
                    2 => color = glm::vec3(0.2,0.6,1.0),
                    3 => color = glm::vec3(0.0,0.7,0.0),
                    4 => color = glm::vec3(0.8,0.8,0.4), 
                    5 => color = glm::vec3(1.0,0.5,0.0),                   
                    _level => {
                        //println!("SKIP {} {} {}", x, y, level);
                        continue;
                    }
                }
                let position = glm::vec2(unit_width * x as f32, unit_height * y as f32);
                let size = glm::vec2 (unit_width as f32, unit_height as f32);
                let texture = if solid {
                    self.resource_manager.load_texture("resources/textures/block_solid.png", "block_solid".into())
                } else {
                    self.resource_manager.load_texture("resources/textures/block.png", "block".into())
                };

                let brick = GameObject::new(
                    position, 
                    size,
                    glm::vec2(0.0f32,0.0f32),
                    color,
                    0.0,
                    texture, 
                    solid);
                
                self.bricks.push(brick);

            }
        }
    }

    pub fn draw(&self, renderer: &SpriteRenderer) {
        self.bricks.iter().for_each(|tile| {
            if !tile.is_destroyed() {
                tile.draw(renderer);
            }
        }); 
    }

    pub fn is_complete(&self) ->bool {
        self.bricks.iter().any( |tile| {
            !tile.is_solid() && !tile.is_destroyed()
        })
    }
}
