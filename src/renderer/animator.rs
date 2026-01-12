use std::{cell::RefCell, rc::Rc};
use web_sys::wasm_bindgen::prelude::{Closure, JsCast};

pub struct Animator {
	running: Rc<RefCell<bool>>,
}

impl Animator {
	pub fn start<F>(mut update: F) -> Self 
	where 
		F: FnMut(f32) + 'static  // time in seconds
	{
		let running = Rc::new(RefCell::new(true));
		let running_clone = running.clone();

		let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
		let g = f.clone();

		*g.borrow_mut() = Some(Closure::new(move |time_ms: f64| {
			if !*running_clone.borrow() {
				return;
			}
			
			update((time_ms / 1000.0) as f32);
			
			web_sys::window()
				.unwrap()
				.request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
				.unwrap();
		}));

		web_sys::window()
			.unwrap()
			.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
			.unwrap();

		Self { running }
	}

	pub fn stop(&self) {
		*self.running.borrow_mut() = false;
	}
}