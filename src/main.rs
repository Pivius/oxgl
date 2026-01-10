use leptos::{mount::mount_to_body, prelude::*, *};
use console_error_panic_hook;
use stylance::import_style;

fn main() {
	console_error_panic_hook::set_once();
	console_log::init_with_level(log::Level::Debug).unwrap();

	mount_to_body(|| {
		view! {
			<App />
		}
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

#[component]
fn Canvas() -> impl IntoView {
	view! {
		<canvas class=style::gl_canvas width="800" height="600"></canvas>
	}
}