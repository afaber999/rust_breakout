use std::ops::Drop;
use std::path::Path;
use std::rc::Rc;

use crate::resource_manager::ResourceManager;
use crate::mesh::Mesh;
use crate::mesh::MeshTexture;
use crate::shader::Shader;

use tobj;

pub struct Model {
    meshes : Vec<Mesh>,
}

impl Model {
    /// constructor, expects a filepath to a 3D model.
    pub fn new( resource_manager: Rc<ResourceManager>, path: &str) -> Self {
        
        let mut meshes = Vec::new();

        let path = Path::new(path);

        // retrieve the directory path of the filepath
        let directory : String = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();
        let obj = tobj::load_obj(path, true);

        let (models, materials) = obj.unwrap();
        
        for model in models.into_iter() {

            let mesh = &model.mesh;

            let mut textures:Vec<MeshTexture>= Vec::new();

            if let Some(material_id) = mesh.material_id {
                let material = &materials[material_id];     
                if material.diffuse_texture.len() > 0 {
                    println!("material.diffuse_texture {}", &material.diffuse_texture);
                    let filename = format!("{}/{}", &directory, material.diffuse_texture);
                    let rc_texture = resource_manager.load_texture(&filename,filename.clone());
                    textures.push( MeshTexture::DiffuseMap( rc_texture ));
                }

                if material.specular_texture.len() > 0 {
                    println!("material.normal_texture {}", &material.specular_texture);
                    let filename = format!("{}/{}", &directory, material.specular_texture);
                    let rc_texture = resource_manager.load_texture(&filename,filename.clone());
                    textures.push( MeshTexture::SpecularMap( rc_texture ));
                }

                if material.normal_texture.len() > 0 {
                    println!("material.normal_texture {}", &material.normal_texture);
                    let filename = format!("{}/{}", &directory, material.normal_texture);
                    let rc_texture = resource_manager.load_texture(&filename,filename.clone());
                    textures.push( MeshTexture::NormalMap( rc_texture ));
                }
            }

            let new_mesh = Mesh::new(
                resource_manager.get_gl(),
                &mesh.positions,
                &mesh.normals, 
                &mesh.texcoords,
                &mesh.indices,
                textures);

            meshes.push(new_mesh);
        }

        Self {
            meshes,
        }
    }
    pub fn draw(&self, shader: &Shader) {
        for mesh in &self.meshes {
            mesh.draw(shader);
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {

    }
}
