use leptos::{mount::mount_to_body, prelude::*, *};
use console_error_panic_hook;
use stylance::import_style;
use glam::{Vec3, Quat};

mod renderer;
use renderer::{Renderer, mesh::Mesh, scene::Scene, object::Object, primitive::Primitive, scene::Transform};

fn main() {
	console_error_panic_hook::set_once();
	console_log::init_with_level(log::Level::Debug).unwrap();

	mount_to_body(|| {
		view! {
			<App />
		}
	})
}

import_style!(style, "main.module.scss");

#[component]
fn App() -> impl IntoView {
	view! {
		<div class=style::app>
			<Canvas />
		</div>
	}
}

const VERTEX_SHADER_SRC: &str = include_str!("./shaders/standard.vert");
const FRAGMENT_SHADER_SRC: &str = include_str!("./shaders/standard.frag");

#[component]
fn Canvas() -> impl IntoView {
	let canvas_ref = NodeRef::new();

	Effect::new(move |_| {
		let mut renderer = Renderer::new("webgl-canvas");

		renderer.camera.set_position(Vec3::new(0.0, 0.0, -3.0));
		renderer.camera.target = Vec3::new(0.0, -0.5, 0.0);
		renderer.set_shader(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);

		let cube_vertices = Primitive::Cube.vertices();
		let cube_mesh = Mesh::new(&renderer.gl, &cube_vertices);
		let rotation = Quat::from_rotation_y(45f32.to_radians());

		let mut scene = Scene::new();
		scene.add(
			Object {
				kind: renderer::object::ObjectKind::Mesh(cube_mesh),
				transform: Transform {
					position: Vec3::new(0.0, 0.0, 0.0),
					rotation: rotation,
				}
			},
		);

		renderer.clear();
		scene.render(&renderer);
	});

	view! {
		<canvas node_ref=canvas_ref class=style::gl_canvas id="webgl-canvas" width="800" height="600"></canvas>
	}
}