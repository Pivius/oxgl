use leptos::{mount::mount_to_body, prelude::*};
use console_error_panic_hook;
use stylance::import_style;
use glam::{Quat, Vec3};
use oxgl::{
	App, core::Transform3D, 
	common::{material::presets, Mesh}, 
	renderer_3d::{Light, Primitive, PostProcessStack, postprocessing::presets as pp_presets},
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

const CANVAS_WIDTH: i32 = 800;
const CANVAS_HEIGHT: i32 = 600;

#[component]
fn Canvas() -> impl IntoView {
	let canvas_ref = NodeRef::new();

	Effect::new(move |_| {
		let app = App::new("webgl-canvas");
		let gl = &app.renderer.gl;
		let _ = app.scene.borrow_mut().enable_shadows(gl);

		{
			//let mut debug = app.debug.borrow_mut();
			//debug.show_grid = true;
			//debug.show_axes = true;
			//debug.show_light_gizmos = true;
			//debug.show_object_bounds = true;
			//debug.grid_size = 10.0;
			//debug.grid_divisions = 10;
		}

		let mut post_process = PostProcessStack::new(gl, CANVAS_WIDTH, CANVAS_HEIGHT).unwrap();
		let _ = post_process.push(pp_presets::vignette(gl, 0.8, 0.4));
		let _ = post_process.push(pp_presets::chromatic_aberration(gl, 10.0));
		let _ = post_process.push(pp_presets::film_grain(gl, 0.1));

		app.scene.borrow_mut().set_post_process(post_process);
		
		let point_light_id = app.scene.borrow_mut().add_light(
			Light::point(
				Vec3::new(2.0, 1.0, 0.0),
				Vec3::new(1.0, 0.5, 0.0),
				3.0,
				20.0
			).with_shadows(true)
		);

		let point_light_id2 = app.scene.borrow_mut().add_light(
			Light::point(
				Vec3::new(-2.0, 1.0, 0.0),
				Vec3::new(0.0, 0.5, 1.0),
				3.0,
				20.0
			).with_shadows(true)
		);

		let ground = app.scene.borrow_mut().add(
			Mesh::with_normals(&app.renderer.gl, &Primitive::Quad.vertices_with_normals(), 
				presets::phong(&app.renderer.gl, Vec3::new(0.5, 0.5, 0.5))),
			Transform3D::new().with_scale(Vec3::splat(10.0)).with_rotation(Quat::from_rotation_x(-90f32.to_radians()))
		);

		let cube = app.scene.borrow_mut().add(
			Mesh::with_normals(
				&app.renderer.gl,
				&Primitive::Cube.vertices_with_normals(),
				presets::phong(&app.renderer.gl, Vec3::new(0.4, 0.8, 0.4))
			),
			Transform3D::new().with_position(Vec3::new(2.0, 0.5, 0.0))
		);

		let teapot = app.scene.borrow_mut().add(
			Mesh::from_obj(
				&app.renderer.gl,
				include_str!("./teapot.obj"),
				presets::phong(&app.renderer.gl, Vec3::new(0.8, 0.4, 0.4))
			).unwrap().remove(0),
			Transform3D::new().with_position(Vec3::new(0.0, 0.0, 0.0)).with_scale(Vec3::splat(0.5))
		);

		app.run(move |scene, time| {
			if let Some(obj) = scene.get_mut(cube) {
				obj.transform.rotation = Quat::from_rotation_y(time);
			}

			if let Some(light) = scene.get_light_mut(point_light_id) {
				light.position = Vec3::new(time.cos() * 3.0, 1.0, time.sin() * 3.0);
			}
		});
	});

	view! {
		<canvas node_ref=canvas_ref class=style::gl_canvas id="webgl-canvas" width=CANVAS_WIDTH height=CANVAS_HEIGHT></canvas>
	}
}