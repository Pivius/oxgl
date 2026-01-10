use leptos::{mount::mount_to_body, prelude::*, *};
use console_error_panic_hook;
use stylance::import_style;

mod renderer;
use renderer::{Renderer, mesh::Mesh, scene::Scene, shader};

use crate::renderer::{primitive::Primitive, shader::{compile_shader, link_program}};

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

#[component]
fn Canvas() -> impl IntoView {
	let canvas_ref = NodeRef::new();

	Effect::new(move |_| {
		let mut renderer = Renderer::new("webgl-canvas");

		renderer.set_shader(
			r#"
				attribute vec3 position;
				void main() { gl_Position = vec4(position, 1.0); }
			"#,
			r#"
				void main() { gl_FragColor = vec4(0.4, 0.8, 1.0, 1.0); }
			"#
		);
		
		let quad_vertices = Primitive::Quad.vertices();
		let quad_mesh = Mesh::new(&renderer.gl, &quad_vertices);
		
		renderer.clear();
		renderer.draw_mesh(&quad_mesh); 
	});

	view! {
		<canvas node_ref=canvas_ref class=style::gl_canvas id="webgl-canvas" width="800" height="600"></canvas>
	}
}