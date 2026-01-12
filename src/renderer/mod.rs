pub mod mesh;
pub mod shader;
pub mod particles;
pub mod scene;
pub mod primitive;
pub mod camera;
pub mod util;
pub mod object;
pub mod animator;

use std::{cell::RefCell, rc::Rc};
use glam::Vec3;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext as GL, wasm_bindgen::JsCast};
use crate::renderer::{animator::Animator, camera::Camera, scene::Scene};

pub struct Renderer {
	pub gl: GL,
	pub canvas: HtmlCanvasElement,
}

impl Renderer {
	pub fn new(canvas_id: &str) -> Self {
		let window = web_sys::window().unwrap();
		let document = window.document().unwrap();
		let canvas = document
			.get_element_by_id(canvas_id)
			.unwrap()
			.dyn_into::<HtmlCanvasElement>()
			.unwrap();
		let gl: GL = canvas
			.get_context("webgl")
			.unwrap()
			.unwrap()
			.dyn_into()
			.unwrap();

		Self { gl, canvas }
	}

	pub fn clear(&self) {
		self.gl.clear_color(0.1, 0.1, 0.12, 1.0);
		self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
	}
}

pub struct App {
	pub renderer: Rc<Renderer>,
	pub scene: Rc<RefCell<Scene>>,
}

impl App {
	pub fn new(canvas_id: &str) -> Self {
		let renderer = Rc::new(Renderer::new(canvas_id));
		let aspect = renderer.canvas.width() as f32 / renderer.canvas.height() as f32;
		
		let camera = Camera::new(aspect)
			.with_position(Vec3::new(0.0, 1.0, 5.0))
			.with_target(Vec3::ZERO);
		
		let scene = Rc::new(RefCell::new(Scene::new(camera)));
		
		Self { renderer, scene }
	}

	pub fn run<F>(self, mut update: F) -> Animator
	where
		F: FnMut(&mut Scene, f32) + 'static,
	{
		let scene = self.scene;
		let renderer = self.renderer;

		Animator::start(move |time| {
			{
				let mut scene = scene.borrow_mut();
				update(&mut scene, time);
			}
			renderer.clear();
			scene.borrow().render(&renderer);
		})
	}
}