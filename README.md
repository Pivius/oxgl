# OxGL

A lightweight WebGL rendering library for Rust, targeting WebAssembly.

## Repository Structure

```
├── oxgl/       # Core library (publishable crate)
└── demo/       # Example Leptos application
```

## Quick Start

### Run the Demo

```bash
cd demo
trunk serve
```

Open [http://localhost:8080](http://localhost:8080)

### Use as a Dependency

```toml
[dependencies]
oxgl = "0.1.0"
```

## Demo Preview

The demo showcases:
- Phong-shaded rotating cube
- Animated point light
- Debug gizmos (grid, axes, light indicators)

```rust
use oxgl::{App, core::Transform3D, common::{material::presets, mesh::Mesh}, renderer_3d::{light::Light, primitive::Primitive}};
use glam::{Quat, Vec3};

let app = App::new("canvas-id");

// Add a lit cube
let cube = app.scene.borrow_mut().add(
    Mesh::with_normals(&app.renderer.gl, &Primitive::Cube.vertices_with_normals(), presets::phong(&app.renderer.gl, Vec3::new(0.4, 0.8, 0.4))),
    Transform3D::new().with_position(Vec3::new(0.0, 0.5, 0.0))
);

// Add a point light
app.scene.borrow_mut().add_light(
    Light::point(Vec3::new(2.0, 1.0, 0.0), Vec3::new(1.0, 0.5, 0.0), 3.0, 5.0)
);

// Animate
app.run(|scene, time| {
    if let Some(obj) = scene.get_mut(cube) {
        obj.transform.rotation = Quat::from_rotation_y(time);
    }
});
```

## Documentation

See [oxgl/README.md](oxgl/README.md) for full API documentation.

## License

[MIT](LICENSE)