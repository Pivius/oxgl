//! Shader Compilation and Linking
//!
//! Provides low-level utilities for compiling GLSL shaders and linking them
//! into shader programs for WebGL2.
//!
//! ## Examples
//!
//! ```
//! use oxgl::common::{compile_shader, link_program};
//! use web_sys::WebGl2RenderingContext as GL;
//!
//! let vert_src = r#"
//!		attribute vec3 position;
//!		void main() {
//!			gl_Position = vec4(position, 1.0);
//!		}
//! "#;
//!
//! let frag_src = r#"
//!		precision mediump float;
//!		void main() {
//!			gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
//!		}
//! "#;
//!
//! let vert_shader = compile_shader(&gl, vert_src, GL::VERTEX_SHADER)?;
//! let frag_shader = compile_shader(&gl, frag_src, GL::FRAGMENT_SHADER)?;
//! let program = link_program(&gl, &vert_shader, &frag_shader)?;
//! ```
//!

use web_sys::{WebGlProgram, WebGl2RenderingContext as GL, WebGlShader};

/// Compiles a GLSL shader from source code.
///
/// Takes GLSL source code and compiles it into a shader object that can be
/// linked into a program with [`link_program`].
///
/// # Errors
///
/// Returns an error string containing the shader compilation log if:
/// - The shader object could not be created
/// - The shader source contains syntax errors
/// - The shader uses unsupported GLSL features
///
/// # Examples
///
/// ```
/// use oxgl::common::compile_shader;
/// use web_sys::WebGl2RenderingContext as GL;
///
/// let vertex_source = r#"
///		attribute vec3 position;
///		uniform mat4 model;
///		uniform mat4 view;
///		uniform mat4 projection;
///
///		void main() {
///			gl_Position = projection * view * model * vec4(position, 1.0);
///		}
/// "#;
///
/// let shader = compile_shader(&gl, vertex_source, GL::VERTEX_SHADER)?;
/// ```
///
/// Handling compilation errors:
///
/// ```
/// match compile_shader(&gl, bad_source, GL::FRAGMENT_SHADER) {
///		Ok(shader) => { /* use shader */ }
///		Err(log) => {
///			log::error!("Shader compilation failed:\n{}", log);
///		}
/// }
/// ```
pub fn compile_shader(gl: &GL, source: &str, shader_type: u32) -> Result<WebGlShader, String> {
	let shader = gl.create_shader(shader_type).ok_or("Unable to create shader")?;

	gl.shader_source(&shader, source);
	gl.compile_shader(&shader);

	if gl.get_shader_parameter(&shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false) {
		Ok(shader)
	} else {
		Err(gl.get_shader_info_log(&shader).unwrap_or_else(|| "Unknown error".to_string()))
	}
}

/// Links vertex and fragment shaders into a shader program.
///
/// Combines compiled vertex and fragment shaders into a complete shader program
/// that can be used for rendering.
///
/// # Errors
///
/// Returns an error string containing the program link log if:
/// - The program object could not be created
/// - The shaders have mismatched varyings (outputs don't match inputs)
/// - Required attributes or uniforms are missing
///
/// # Examples
///
/// ```
/// use oxgl::common::{compile_shader, link_program};
/// use web_sys::WebGl2RenderingContext as GL;
///
/// let vert = compile_shader(&gl, vert_source, GL::VERTEX_SHADER)?;
/// let frag = compile_shader(&gl, frag_source, GL::FRAGMENT_SHADER)?;
///
/// let program = link_program(&gl, &vert, &frag)?;
///
/// // Use the program for rendering
/// gl.use_program(Some(&program));
/// ```
///
/// Complete shader pipeline:
///
/// ```
/// use oxgl::common::{compile_shader, link_program};
/// use web_sys::WebGl2RenderingContext as GL;
///
/// fn create_program(gl: &GL, vert_src: &str, frag_src: &str) -> Result<WebGlProgram, String> {
///		let vert = compile_shader(gl, vert_src, GL::VERTEX_SHADER)?;
///		let frag = compile_shader(gl, frag_src, GL::FRAGMENT_SHADER)?;
///		link_program(gl, &vert, &frag)
/// }
/// ```
pub fn link_program(gl: &GL, vert_shader: &WebGlShader, frag_shader: &WebGlShader) -> Result<WebGlProgram, String> {
	let program = gl.create_program().ok_or("Unable to create program")?;

	gl.attach_shader(&program, vert_shader);
	gl.attach_shader(&program, frag_shader);
	gl.link_program(&program);

	if gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false) {
		Ok(program)
	} else {
		Err(gl.get_program_info_log(&program).unwrap_or_else(|| "Unknown error".to_string()))
	}
}