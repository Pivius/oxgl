# OxGL

A lightweight WebGL rendering library for Rust, targeting WebAssembly.

## Features

- **3D Rendering** — Scene graph with transforms, meshes, and materials
- **Lighting** — Point, directional, and spot lights with Phong/Lambert shading
- **Materials** — Prebuilt shaders (unlit, lambert, phong) with custom uniform support
- **Primitives** — Cube, sphere, plane, cylinder out of the box
- **Gizmos** — Debug visualization for lights and objects
- **Animation Loop** — Built-in requestAnimationFrame wrapper

## Installation

```toml
[dependencies]
oxgl = "0.1.0"
```

## Quick Start

```rust
use oxgl::{App, Scene, Mesh, Light, Transform3D, material::presets, primitive::Primitive};
use glam::{Vec3, Quat};

// Create the app
let app = App::new("canvas-id").expect("Failed to create app");

// Build a scene
let mut scene = Scene::new();

// Add a mesh
let cube = Mesh::builder()
    .geometry(Primitive::Cube.into())
    .material(presets::lambert())
    .transform(Transform3D::new().with_position(Vec3::ZERO))
    .build();

scene.add_object(cube);

// Add a light
let light = Light::point()
    .with_position(Vec3::new(2.0, 3.0, 2.0))
    .with_color(Vec3::ONE)
    .with_intensity(1.0);

scene.add_light(light);

// Render loop
Animator::new(move |dt| {
    app.render(&scene);
}).start();
```

## Modules

| Module | Description |
|--------|-------------|
| `core` | Transform, Color, Animator, ID types |
| `common` | Mesh, Material, Shader, Camera |
| `renderer_3d` | Scene, Light, Gizmo, Primitives |


## Shaders

Built-in shaders:

- **Unlit** — No lighting, flat color/texture
- **Lambert** — Diffuse-only lighting
- **Phong** — Diffuse + specular highlights

## License

[MIT](LICENSE)