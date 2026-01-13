# OxGL

A lightweight WebGL rendering library for Rust, targeting WebAssembly.

## Installation

```toml
[dependencies]
oxgl = "0.1.0"
```

## Quick Start

```rust
use oxgl::{
    App, 
    core::Transform3D, 
    common::{material::presets, mesh::Mesh}, 
    renderer_3d::light::Light
};
use glam::{Vec3, Quat};

// Create the app
let app = App::new("canvas-id");

// Enable debug visuals
{
    let mut debug = app.debug.borrow_mut();
    debug.show_grid = true;
    debug.show_axes = true;
    debug.show_light_gizmos = true;
}

// Add a mesh with phong shading
let cube = app.scene.borrow_mut().add(
    Mesh::with_normals(
        presets::phong(&app.renderer.gl, Vec3::new(0.4, 0.8, 0.4))
    ),
    Transform3D::new().with_position(Vec3::new(0.0, 0.5, 0.0))
);

// Add a point light
let light_id = app.scene.borrow_mut().add_light(
    Light::point(Vec3::new(2.0, 1.0, 0.0))
);

// Run the animation loop
app.run(|scene, time| {
    // Rotate the cube
    if let Some(obj) = scene.get_mut(cube) {
        obj.transform.rotation = Quat::from_rotation_y(time);
    }

    // Animate the light position
    if let Some(light) = scene.get_light_mut(light_id) {
        light.set_position(Vec3::new(
            time.cos() * 2.0,
            1.0,
            time.sin() * 2.0
        ));
    }
});
```

## Modules

| Module | Description |
|--------|-------------|
| `core` | Transform, Color, Animator, ID types |
| `common` | Mesh, Material, Shader, Camera |
| `renderer_3d` | Scene, Light, Gizmo, Primitives |

## License

[MIT](LICENSE)