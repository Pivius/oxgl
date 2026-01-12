use std::{cell::RefCell, rc::Rc};
use leptos::{mount::mount_to_body, prelude::*, *};
use console_error_panic_hook;
use stylance::import_style;
use glam::{Quat, Vec3, Vec4};

mod renderer;
use renderer::{
	Renderer, 
	camera::Camera, 
	mesh::{Mesh, UniformValue}, 
	scene::{Scene, Transform}, 
	primitive::Primitive
};

fn main() {
	console_error_panic_hook::set_once();
	console_log::init_with_level(log::Level::Debug).unwrap();

	mount_to_body(|| {
		view! { <App /> }
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

const VERTEX_SHADER: &str = include_str!("./shaders/standard.vert");
const FRAGMENT_SHADER: &str = include_str!("./shaders/standard.frag");

#[component]
fn Canvas() -> impl IntoView {
	let canvas_ref = NodeRef::new();

	Effect::new(move |_| {
		let renderer = Renderer::new("webgl-canvas");
		let aspect = renderer.canvas.width() as f32 / renderer.canvas.height() as f32;

		let camera = Camera::new(aspect)
			.with_position(Vec3::new(0.0, 1.0, 5.0))
			.with_target(Vec3::ZERO);

		let mut scene = Scene::new(camera);

		let cube_mesh = Mesh::builder(&renderer.gl, &Primitive::Cube.vertices())
			.shader(VERTEX_SHADER, FRAGMENT_SHADER)
			.uniform("color", UniformValue::Vec4(Vec4::new(0.4, 0.8, 1.0, 1.0)))
			.attribute("position", 3)
			.build();

		let cube_id = scene.add(
			cube_mesh,
			Transform::new()
				.with_position(Vec3::ZERO)
				.with_rotation(Quat::from_rotation_y(45f32.to_radians()))
		);

		let scene = Rc::new(RefCell::new(scene));
		let renderer = Rc::new(renderer);

		// Animation loop
		let scene_clone = scene.clone();
		let renderer_clone = renderer.clone();
		
		request_animation_frame(move |time| {
			let mut scene = scene_clone.borrow_mut();
			
			// Animate cube rotation
			if let Some(obj) = scene.get_mut(cube_id) {
				obj.transform.rotation = Quat::from_rotation_y((time / 1000.0) as f32);
				obj.transform.position = Vec3::new(0.0, (time / 1000.0).sin() as f32 * 0.5, 0.0);
			}

			// Render
			renderer_clone.clear();
			scene.render(&renderer_clone);
		});
	});

	view! {
		<canvas node_ref=canvas_ref class=style::gl_canvas id="webgl-canvas" width="800" height="600"></canvas>
	}
}

fn request_animation_frame<F: FnMut(f64) + 'static>(mut callback: F) {
	use wasm_bindgen::{closure::Closure, JsCast};
	
	let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
	let g = f.clone();

	*g.borrow_mut() = Some(Closure::new(move |time: f64| {
		callback(time);
		web_sys::window()
			.unwrap()
			.request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
			.unwrap();
	}));

	web_sys::window()
		.unwrap()
		.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
		.unwrap();
}