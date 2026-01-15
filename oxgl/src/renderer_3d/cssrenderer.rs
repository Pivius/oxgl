//! CSS3D Renderer
//!
//! Overlays HTML elements in 3D space, synchronized with the WebGL camera.
//!

use std::cell::RefCell;
use glam::{Mat4, Vec3};
use slotmap::SlotMap;
use web_sys::{HtmlElement, wasm_bindgen::JsCast};

use crate::common::Camera;
use crate::core::{Transform3D, Transformable, CSS3DElementId};

/// A CSS3D renderable object.
pub struct CSS3DObject {
	pub element: HtmlElement,
	pub transform: Transform3D,
	pub billboard: bool,
}

/// Renders HTML elements in 3D space synchronized with a WebGL camera.
pub struct CSS3DRenderer {
	container: HtmlElement,
	camera_element: HtmlElement,
	scene_element: HtmlElement,
	objects: RefCell<SlotMap<CSS3DElementId, CSS3DObject>>,
	width: f32,
	height: f32,
	fov: f32,
}

impl CSS3DRenderer {
	/// Creates a new CSS3D renderer.
	/// 
	/// The container will be created as a sibling of the canvas element.
	pub fn new(canvas_id: &str, width: u32, height: u32, fov: f32) -> Result<Self, String> {
		let window = web_sys::window().ok_or("No window")?;
		let document = window.document().ok_or("No document")?;
		
		let canvas = document
			.get_element_by_id(canvas_id)
			.ok_or("Canvas not found")?;
		
		let canvas_html = canvas
			.clone()
			.dyn_into::<HtmlElement>()
			.map_err(|_| "Canvas is not an HtmlElement")?;
		
		let canvas_parent = canvas
			.parent_element()
			.ok_or("Canvas has no parent")?;

		if let Ok(parent_el) = canvas_parent.clone().dyn_into::<HtmlElement>() {
			let _ = parent_el.style().set_property("position", "relative");
		}

		let _ = canvas_html.style().set_property("display", "block");

		let fov_rad = fov.to_radians();
		let perspective = (height as f32 / 2.0) / (fov_rad / 2.0).tan();

		let container = document
			.create_element("div")
			.map_err(|_| "Failed to create container")?
			.dyn_into::<HtmlElement>()
			.map_err(|_| "Failed to cast container")?;

		let style = container.style();
		let _ = style.set_property("position", "absolute");
		let _ = style.set_property("top", "0");
		let _ = style.set_property("left", "0");
		let _ = style.set_property("width", &format!("{}px", width));
		let _ = style.set_property("height", &format!("{}px", height));
		let _ = style.set_property("overflow", "hidden");
		let _ = style.set_property("pointer-events", "none");
		let _ = style.set_property("z-index", "1");

		let camera_element = document
			.create_element("div")
			.map_err(|_| "Failed to create camera element")?
			.dyn_into::<HtmlElement>()
			.map_err(|_| "Failed to cast camera element")?;

		let cam_style = camera_element.style();
		let _ = cam_style.set_property("position", "absolute");
		let _ = cam_style.set_property("left", "50%");
		let _ = cam_style.set_property("top", "50%");
		let _ = cam_style.set_property("width", "0");
		let _ = cam_style.set_property("height", "0");
		let _ = cam_style.set_property("transform-style", "preserve-3d");
		let _ = cam_style.set_property("perspective", &format!("{}px", perspective));

		let scene_element = document
			.create_element("div")
			.map_err(|_| "Failed to create scene element")?
			.dyn_into::<HtmlElement>()
			.map_err(|_| "Failed to cast scene element")?;

		let scene_style = scene_element.style();
		let _ = scene_style.set_property("position", "absolute");
		let _ = scene_style.set_property("transform-style", "preserve-3d");

		camera_element
			.append_child(&scene_element)
			.map_err(|_| "Failed to append scene element")?;

		container
			.append_child(&camera_element)
			.map_err(|_| "Failed to append camera element")?;

		if let Some(next_sibling) = canvas.next_sibling() {
			canvas_parent
				.insert_before(&container, Some(&next_sibling))
				.map_err(|_| "Failed to insert container")?;
		} else {
			canvas_parent
				.append_child(&container)
				.map_err(|_| "Failed to append container")?;
		}

		Ok(Self {
			container,
			camera_element,
			scene_element,
			objects: RefCell::new(SlotMap::with_key()),
			width: width as f32,
			height: height as f32,
			fov,
		})
	}

	/// Adds an HTML element to the 3D scene.
	pub fn add_element(&self, html: &str, transform: Transform3D) -> Result<CSS3DElementId, String> {
		let window = web_sys::window().ok_or("No window")?;
		let document = window.document().ok_or("No document")?;
		
		let wrapper = document
			.create_element("div")
			.map_err(|_| "Failed to create element")?
			.dyn_into::<HtmlElement>()
			.map_err(|_| "Failed to cast element")?;

		wrapper.set_inner_html(html);

		let style = wrapper.style();
		let _ = style.set_property("position", "absolute");
		let _ = style.set_property("transform-style", "preserve-3d");
		let _ = style.set_property("pointer-events", "auto");
		let _ = style.set_property("white-space", "nowrap");

		self.scene_element
			.append_child(&wrapper)
			.map_err(|_| "Failed to append element")?;

		let object = CSS3DObject {
			element: wrapper,
			transform,
			billboard: false,
		};

		let id = self.objects.borrow_mut().insert(object);
		Ok(id)
	}

	/// Adds a billboard element that always faces the camera.
	pub fn add_billboard(&self, html: &str, position: Vec3) -> Result<CSS3DElementId, String> {
		let id = self.add_element(html, Transform3D::new().with_position(position))?;
		
		if let Some(obj) = self.objects.borrow_mut().get_mut(id) {
			obj.billboard = true;
		}

		Ok(id)
	}

	/// Removes an element from the scene.
	pub fn remove_element(&self, id: CSS3DElementId) -> bool {
		if let Some(obj) = self.objects.borrow_mut().remove(id) {
			let _ = obj.element.remove();
			true
		} else {
			false
		}
	}

	/// Gets a mutable reference to an element's object.
	pub fn with_element_mut<F>(&self, id: CSS3DElementId, f: F)
	where
		F: FnOnce(&mut CSS3DObject),
	{
		if let Some(obj) = self.objects.borrow_mut().get_mut(id) {
			f(obj);
		}
	}

	/// Updates the element's HTML content.
	pub fn set_html(&self, id: CSS3DElementId, html: &str) {
		if let Some(obj) = self.objects.borrow().get(id) {
			obj.element.set_inner_html(html);
		}
	}

	/// Updates the element's transform.
	pub fn set_transform(&self, id: CSS3DElementId, transform: Transform3D) {
		if let Some(obj) = self.objects.borrow_mut().get_mut(id) {
			obj.transform = transform;
		}
	}

	/// Renders all CSS3D elements using the given camera.
	pub fn render(&self, camera: &Camera) {
		let scale = 100.0;
		
		let fov_rad = self.fov.to_radians();
		let perspective = (self.height / 2.0) / (fov_rad / 2.0).tan();

		let view = camera.view_matrix();

		// flip Y
		let scene_transform = self.get_css_matrix_string(&view, scale, true);
		
		let _ = self.scene_element.style().set_property(
			"transform",
			&format!("translateZ({}px) {}", perspective, scene_transform),
		);

		let objects = self.objects.borrow();

		for obj in objects.values() {
			let model = if obj.billboard {
				Mat4::from_translation(obj.transform.position)
			} else {
				obj.transform.to_matrix()
			};

			let css_transform = self.get_css_matrix_string(&model, scale, false);
			
			let style = obj.element.style();
			let _ = style.set_property("transform", &format!("translate(-50%, -50%) {}", css_transform));
		}
	}

	/// Converts a Mat4 to a CSS matrix3d string.
	fn get_css_matrix_string(&self, mat: &Mat4, scale: f32, flip_y: bool) -> String {
		let m = mat.to_cols_array();
		let y_flip = if flip_y { -1.0 } else { 1.0 };

		format!(
			"matrix3d({},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{})",
			m[0], m[1] * y_flip, m[2], m[3],
			m[4] * y_flip, m[5], m[6] * y_flip, m[7],
			m[8], m[9] * y_flip, m[10], m[11],
			m[12] * scale, m[13] * scale * y_flip, m[14] * scale, m[15]
		)
	}

	/// Shows or hides the CSS3D layer.
	pub fn set_visible(&self, visible: bool) {
		let _ = self.container.style().set_property(
			"display",
			if visible { "block" } else { "none" },
		);
	}

	/// Resizes the CSS3D viewport.
	pub fn resize(&mut self, width: u32, height: u32) {
		self.width = width as f32;
		self.height = height as f32;

		let fov_rad = self.fov.to_radians();
		let perspective = (self.height / 2.0) / (fov_rad / 2.0).tan();

		let _ = self.container.style().set_property("width", &format!("{}px", width));
		let _ = self.container.style().set_property("height", &format!("{}px", height));
		let _ = self.camera_element.style().set_property("perspective", &format!("{}px", perspective));
	}
}