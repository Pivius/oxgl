pub mod core;
pub mod common;
pub mod renderer_3d;

use std::{cell::RefCell, rc::Rc};
use glam::Vec3;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext as GL, wasm_bindgen::JsCast};

use crate::{renderer_3d::{Scene, GizmoRenderer, DebugSettings}, common::Camera, core::Animator};

pub struct Renderer {
	pub gl: GL,
	pub canvas: HtmlCanvasElement,
}

impl Renderer {
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

pub struct App {
	pub renderer: Rc<Renderer>,
	pub scene: Rc<RefCell<Scene>>,
	pub gizmos: Rc<GizmoRenderer>,
	pub debug: Rc<RefCell<DebugSettings>>,
}

impl App {
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