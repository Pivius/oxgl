use leptos::{mount::mount_to_body, prelude::*, *};
use console_error_panic_hook;
use stylance::import_style;
use glam::{Quat, Vec3, Vec4};

mod renderer;
use renderer::{
	App, 
	mesh::Mesh, 
	scene::Transform, 
	primitive::Primitive,
	material::presets,
	light::Light,
};

fn main() {
	console_error_panic_hook::set_once();
	console_log::init_with_level(log::Level::Debug).unwrap();
	mount_to_body(|| view! { <Root /> })
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

#[component]
fn Canvas() -> impl IntoView {
	let canvas_ref = NodeRef::new();

	Effect::new(move |_| {
		let app = App::new("webgl-canvas");

		{
			let mut debug = app.debug.borrow_mut();
			debug.show_grid = true;
			debug.show_axes = true;
			debug.show_light_gizmos = true;
			debug.show_object_bounds = true;
			debug.grid_size = 10.0;
			debug.grid_divisions = 10;
		}

		let point_light_id = app.scene.borrow_mut().add_light(
			Light::point(
				Vec3::new(2.0, 1.0, 0.0),
				Vec3::new(1.0, 0.5, 0.0),
				3.0,
				5.0
			)
		);

		let cube = app.scene.borrow_mut().add(
			Mesh::with_normals(
				&app.renderer.gl,
				&Primitive::Cube.vertices_with_normals(),
				presets::phong(&app.renderer.gl, Vec3::new(0.4, 0.8, 0.4))
			),
			Transform::new().with_position(Vec3::new(0.0, 0.5, 0.0))
		);

		app.run(move |scene, time| {
			if let Some(obj) = scene.get_mut(cube) {
				obj.transform.rotation = Quat::from_rotation_y(time);
			}

			// Animate point light position
			if let Some(light) = scene.get_light_mut(point_light_id) {
				light.position = Vec3::new(time.cos() * 3.0, 1.0, time.sin() * 3.0);
			}
		});
	});

	view! {
		<canvas node_ref=canvas_ref class=style::gl_canvas id="webgl-canvas" width="800" height="600"></canvas>
	}
}