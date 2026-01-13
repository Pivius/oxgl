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
	material::{Material, MaterialBuilder, Uniform, presets},
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

// Custom shader example
const CUSTOM_VERT: &str = include_str!("./shaders/phong.vert");
const CUSTOM_FRAG: &str = include_str!("./shaders/phong.frag");

#[component]
fn Canvas() -> impl IntoView {
	let canvas_ref = NodeRef::new();

	Effect::new(move |_| {
		let app = App::new("webgl-canvas");

		app.scene.borrow_mut().add_light(
			Light::directional(
				Vec3::new(-1.0, -1.0, -1.0),
				Vec3::new(1.0, 1.0, 1.0),
				1.0
			)
		);

		let cube1 = app.scene.borrow_mut().add(
			Mesh::new(
				&app.renderer.gl,
				&Primitive::Cube.vertices(),
				presets::unlit(&app.renderer.gl, Vec4::new(0.4, 0.8, 1.0, 1.0))
			),
			Transform::new().with_position(Vec3::new(-3.0, 0.0, 0.0))
		);

		let cube2 = app.scene.borrow_mut().add(
			Mesh::with_normals(
				&app.renderer.gl,
				&Primitive::Cube.vertices_with_normals(),
				MaterialBuilder::new(
					&app.renderer.gl,
					include_str!("./shaders/lambert.vert"),
					include_str!("./shaders/lambert.frag"),
				)
				.color3(0.8, 0.4, 0.4)
				.ambient(0.15)
				.build()
			),
			Transform::new().with_position(Vec3::new(0.0, 0.0, 0.0))
		);

		let mut custom_mat = Material::from_source(
			&app.renderer.gl,
			CUSTOM_VERT,
			CUSTOM_FRAG,
		).unwrap();

		custom_mat
			.set_vec3("color", Vec3::new(0.4, 0.8, 0.4))
			.set_float("ambient", 0.1)
			.set_float("shininess", 64.0)
			.set_float("specularStrength", 0.8);

		let cube3 = app.scene.borrow_mut().add(
			Mesh::with_normals(
				&app.renderer.gl,
				&Primitive::Cube.vertices_with_normals(),
				custom_mat
			),
			Transform::new().with_position(Vec3::new(3.0, 0.0, 0.0))
		);

		app.run(move |scene, time| {
			for id in [cube1, cube2, cube3] {
				if let Some(obj) = scene.get_mut(id) {
					obj.transform.rotation = Quat::from_rotation_y(time);
				}
			}

			if let Some(light) = scene.get_light_mut(0) {
				light.direction = Vec3::new(time.cos(), -1.0, time.sin()).normalize();
			}
		});
	});

	view! {
		<canvas node_ref=canvas_ref class=style::gl_canvas id="webgl-canvas" width="800" height="600"></canvas>
	}
}