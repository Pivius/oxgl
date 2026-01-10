use leptos::{mount::mount_to_body, prelude::*, *};
use console_error_panic_hook;
use stylance::import_style;
use glam::Vec3;

mod renderer;
use renderer::{Renderer, mesh::Mesh, scene::Scene};
use crate::renderer::{primitive::Primitive, scene::Transform};

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

const VERTEX_SHADER_SRC: &str = r#"
	attribute vec3 position;
	uniform mat4 model;
	uniform mat4 view;
	uniform mat4 projection;

	void main() {
		gl_Position = projection * view * model * vec4(position, 1.0);
	}
"#;

const FRAGMENT_SHADER_SRC: &str = r#"
	void main() { gl_FragColor = vec4(0.4, 0.8, 1.0, 1.0); }
"#;

#[component]
fn Canvas() -> impl IntoView {
	let canvas_ref = NodeRef::new();

	Effect::new(move |_| {
		let mut renderer = Renderer::new("webgl-canvas");

		renderer.camera.set_position(Vec3::new(0.0, 0.0, -3.0));
		renderer.camera.target = Vec3::new(0.0, -0.5, 0.0);
		renderer.set_shader(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);

		let quad_vertices = Primitive::Quad.vertices();
		let quad_mesh = Mesh::new(&renderer.gl, &quad_vertices);

		let mut scene = Scene::new();
		scene.add(
			quad_mesh,
			Transform {
				position: Vec3::new(0.0, 0.0, 0.0),
				rotation: 0.0,
			},
		);

		renderer.clear();
		scene.render(&renderer);
	});

	view! {
		<canvas node_ref=canvas_ref class=style::gl_canvas id="webgl-canvas" width="800" height="600"></canvas>
	}
}