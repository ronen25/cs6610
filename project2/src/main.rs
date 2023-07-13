mod obj_loader;

fn main() {
    let obj_model_result = obj_loader::ObjModel::new("assets/teapot.obj");
    if let Ok(obj_model) = obj_model_result {
        println!("Loaded file.\nVertices: {}\nVertex normals: {}\nTexture coords: {}", obj_model.vertices.len(),
        obj_model.vertex_normals.len(), obj_model.texture_coords.len());
    } else if let Err(err) = obj_model_result {
    }
}
