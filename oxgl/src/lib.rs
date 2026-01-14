//! OxGL - A WebGL2 Rendering Library
//!
//! This library provides high-level functionality for 3D rendering in the browser using WebGL2.
//! It includes scene management, lighting, shadows, post-processing effects, and debug gizmos.
//!
//! ## Getting Started
//!
//! Add oxgl to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! oxgl = { path = "../oxgl" }
//! ```
//!
//! Ensure you have a `<canvas>` element in your HTML with a unique ID.
//!
//! ## Examples and Usage
//!
//! ```ignore
//! use oxgl::{App, core::Transform3D, common::material::presets, renderer_3d::Primitive};
//! use glam::Vec3;
//!
//! let app = App::new("webgl-canvas");
//!
//! // Add a cube to the scene
//! let cube = app.scene.borrow_mut().add(
//!		Mesh::with_normals(&app.renderer.gl, &Primitive::Cube.vertices_with_normals(),
//!			presets::phong(&app.renderer.gl, Vec3::new(0.4, 0.8, 0.4))),
//!		Transform3D::new().with_position(Vec3::new(0.0, 0.5, 0.0))
//! );
//!
//! // Run the render loop
//! app.run(|scene, time| {
//!		// Update scene here
//! });
//! ```
//! 
//! ## License
//!
//! MIT License
//!

pub mod core;
pub mod common;
pub mod renderer_3d;

use std::{cell::RefCell, rc::Rc};
use glam::Vec3;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL, wasm_bindgen::JsCast};

use crate::{renderer_3d::{Scene, GizmoRenderer, DebugSettings}, common::Camera, core::Animator};

/// Low-level WebGL2 renderer wrapper.
///
/// Provides access to the WebGL2 context and canvas element.
/// Typically used through [`App`] rather than directly.
///
/// ## Examples
///
/// ```ignore
/// let renderer = Renderer::new("my-canvas");
/// renderer.clear();
/// ```
pub struct Renderer {
	pub gl: GL,
	pub canvas: HtmlCanvasElement,
}

impl Renderer {
	/// Creates a new renderer attached to the specified canvas element.
	///
	/// ## Panics
	///
	/// Panics if the canvas element with the given ID is not found,
	/// or if WebGL2 context creation fails.
	///
	/// ## Examples
	///
	/// ```ignore
	/// let renderer = Renderer::new("webgl-canvas");
	/// ```
	pub fn new(canvas_id: &str) -> Self {
		let window = web_sys::window().expect("No window");
		let document = window.document().expect("No document");
		let canvas = document
			.get_element_by_id(canvas_id)
			.expect("No canvas")
			.dyn_into::<HtmlCanvasElement>()
			.expect("Not a canvas");

		let gl = canvas
			.get_context("webgl2")
			.unwrap()
			.unwrap()
			.dyn_into::<GL>()
			.unwrap();

		gl.enable(GL::DEPTH_TEST);

		Self { gl, canvas }
	}

	pub fn canvas(&self) -> &HtmlCanvasElement {
		&self.canvas
	}

	pub fn clear(&self) {
		self.gl.clear_color(0.1, 0.1, 0.1, 1.0);
		self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
	}
}

/// High-level application wrapper for 3D rendering.
///
/// Combines a renderer, scene, and debug settings into a single interface. 
/// This is the main entry point for most applications using oxgl.
///
/// ## Examples
///
/// ```ignore
/// let app = App::new("webgl-canvas");
///
/// // Configure debug settings
/// app.debug.borrow_mut().show_grid = true;
///
/// // Add objects to the scene
/// let cube_id = app.scene.borrow_mut().add(mesh, transform);
///
/// // Start the render loop
/// app.run(|scene, time| {
///		// Update logic here
/// });
/// ```
pub struct App {
	pub renderer: Rc<Renderer>,
	pub scene: Rc<RefCell<Scene>>,
	pub gizmos: Rc<GizmoRenderer>,
	pub debug: Rc<RefCell<DebugSettings>>,
}

impl App {
	/// Creates a new application attached to the specified canvas element.
	///
	/// Initializes the renderer, creates an empty scene with a default camera,
	/// and sets up debug gizmo rendering.
	///
	/// ## Panics
	///
	/// Panics if the canvas element is not found or WebGL2 initialization fails.
	///
	/// ## Examples
	///
	/// ```ignore
	/// let app = App::new("webgl-canvas");
	/// ```
	pub fn new(canvas_id: &str) -> Self {
		let renderer = Rc::new(Renderer::new(canvas_id));
		let aspect = renderer.canvas.width() as f32 / renderer.canvas.height() as f32;
		
		let camera = Camera::new(aspect)
			.with_position(Vec3::new(0.0, 2.0, 5.0))
			.with_target(Vec3::ZERO);
		
		let scene = Rc::new(RefCell::new(Scene::new(camera)));
		let gizmos = Rc::new(GizmoRenderer::new(&renderer.gl));
		let debug = Rc::new(RefCell::new(DebugSettings::default()));
		
		Self { renderer, scene, gizmos, debug }
	}

	pub fn set_debug(&self, enabled: bool) {
		let mut settings = self.debug.borrow_mut();
		settings.show_grid = enabled;
		settings.show_axes = enabled;
		settings.show_light_gizmos = enabled;
	}

	/// Starts the render loop with the provided update callback.
	///
	/// The callback is called every frame with mutable access to the scene
	/// and the elapsed time in seconds since the application started.
	///
	/// This method consumes the `App` and runs indefinitely.
	///
	/// ## Examples
	///
	/// ```ignore
	/// app.run(|scene, time| {
	///		if let Some(obj) = scene.get_mut(cube_id) {
	///			obj.transform.rotation = Quat::from_rotation_y(time);
	///		}
	/// });
	/// ```
	pub fn run<F>(self, mut update: F) -> Animator
	where
		F: FnMut(&mut Scene, f32) + 'static,
	{
		let scene = self.scene;
		let renderer = self.renderer;
		let gizmos = self.gizmos;
		let debug = self.debug;

		Animator::start(move |time| {
			//renderer.clear();

			{
				let mut scene = scene.borrow_mut();
				update(&mut scene, time);
			}

			{
				let mut scene = scene.borrow_mut();
				scene.render(&renderer, time);

				let settings = debug.borrow();
				scene.render_debug(&renderer, &gizmos, &settings, false);
			}
		})
	}
}