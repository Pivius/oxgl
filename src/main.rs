use std::{cell::RefCell, rc::Rc};
use leptos::{mount::mount_to_body, prelude::*, *};
use console_error_panic_hook;
use stylance::import_style;
use glam::{Quat, Vec3, Vec4};

mod renderer;
use renderer::{
	App, 
	mesh::{Mesh, UniformValue}, 
	scene::{Scene, Transform}, 
	primitive::Primitive
};

fn main() {
	console_error_panic_hook::set_once();
	console_log::init_with_level(log::Level::Debug).unwrap();

	mount_to_body(|| {
		view! { <Root /> }
	})
}

import_style!(style, "main.module.scss");

#[component]
fn Root() -> impl IntoView {
	view! {
		<div class=style::app>
			<Canvas />
		</div>
	}
}

const VERTEX_SHADER: &str = include_str!("./shaders/standard.vert");
const FRAGMENT_SHADER: &str = include_str!("./shaders/standard.frag");

#[component]
fn Canvas() -> impl IntoView {
	let canvas_ref = NodeRef::new();

	Effect::new(move |_| {
		let app = App::new("webgl-canvas");

		let cube_mesh = Mesh::builder(&app.renderer.gl, &Primitive::Cube.vertices())
			.shader(VERTEX_SHADER, FRAGMENT_SHADER)
			.uniform("color", UniformValue::Vec4(Vec4::new(0.4, 0.8, 1.0, 1.0)))
			.attribute("position", 3)
			.build();

		let cube_id = app.scene.borrow_mut().add(cube_mesh, Transform::new());

		app.run(move |scene, time| {
			if let Some(obj) = scene.get_mut(cube_id) {
				obj.transform.rotation = Quat::from_rotation_y(time);
				obj.transform.position.y = time.sin() * 0.5;
			}
		});
	});

	view! {
		<canvas node_ref=canvas_ref class=style::gl_canvas id="webgl-canvas" width="800" height="600"></canvas>
	}
}