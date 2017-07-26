extern crate three;

fn main() {
    let mut window = three::Window::new("Steroids", "shaders");

    let mut camera = window.factory.perspective_camera(45.0, 0.1, 100.0);
    camera.look_at([3.0, 3.0, 1.0], [0.0, 0.0, 0.0], None);

    let cube_geom = three::Geometry::new_box(1.0, 1.0, 1.0);
    let cube_mesh = window.factory.mesh(cube_geom, three::Material::LineBasic { color: 0xFFFF_FFFF });
    window.scene.add(&cube_mesh);

    while window.update() {
        window.render(&camera);
    }
}
