#![feature(generators, generator_trait, box_syntax)]
#![feature(inclusive_range_syntax)]
#![feature(specialization)]
#![feature(ord_max_min)]
#![feature(link_args)]
#![feature(const_fn)]

extern crate common;

pub use resources as res;
pub use common::*;

#[macro_use] pub mod bindings;
#[macro_use] pub mod coro_util;

pub mod mut_rc;

pub mod resources;
pub mod rendering;
pub mod console;
pub mod webgl;

pub mod paper;
pub mod flower;
pub mod particle;

use bindings::emscripten::*;
use coro_util::*;
use webgl::*;

use paper::*;
use flower::*;
use particle::*;

use rendering::gl;
use rendering::shader::*;

use std::time::Instant;

fn main() {
	set_coro_as_main_loop(|| {
		console::init();
		console::set_color("#222");

		let _gl = WebGLContext::new();

		let mut events = Vec::new();

		unsafe {
			use std::ptr::null;

			let evt_ptr = std::mem::transmute(&mut events);

			on_resize(0, null(), evt_ptr);
			emscripten_set_resize_callback(null(), evt_ptr, 0, Some(on_resize));
			emscripten_set_click_callback(null(), evt_ptr, 0, Some(on_click));

			gl::Enable(gl::BLEND);
			gl::BlendEquation(gl::FUNC_ADD);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

			gl::ClearColor(0.41, 0.80, 0.51, 1.0);
		}

		let shader = Shader::new(res::shaders::PAPER_VS, res::shaders::PAPER_FS);
		shader.use_program();

		let mut screen_size = Vec2i::zero();

		let mut flowers = FlowerManager::new();
		let mut paper = Paper::new();

		let mut particles = ParticleManager::new();

		flowers.add_flower(Vec2::new(0.0, -0.1));

		loop {
			let frame_start = Instant::now();

			for e in events.iter() {
				match *e {
					Event::Resize(sz) => unsafe {
						screen_size = sz;

						gl::Viewport(0, 0, sz.x, sz.y);

						let aspect = sz.x as f32 / sz.y as f32;
						shader.set_proj(&Mat4::scale(Vec3::new(1.0/aspect, 1.0, 1.0)));
					}

					Event::Click(pos) => {
						let pos = screen_to_gl(screen_size, pos);
						particles.add_pop(pos);
						flowers.add_flower(pos);
					}
				}
			}

			events.clear();

			unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }

			flowers.update();

			paper.clear();
			flowers.draw(&mut paper);
			paper.draw();

			particles.draw();

			let now = Instant::now();
			if now > frame_start {
				let dur = now - frame_start;
				console::set_section("Stats", format!("frame time: {:.1}ms", dur.subsec_nanos() as f64 / 1000_000.0));
			}
			
			console::update();

			yield;
		}
	});
}

fn screen_to_gl(screen_size: Vec2i, v: Vec2i) -> Vec2{
	let sz = screen_size.to_vec2();
	let aspect = sz.x as f32 / sz.y as f32;

	let norm = v.to_vec2() / screen_size.to_vec2() * 2.0 - Vec2::splat(1.0);
	norm * Vec2::new(aspect, -1.0)
}

enum Event {
	Resize(Vec2i),
	Click(Vec2i),
}

unsafe extern "C"
fn on_resize(_: i32, _e: *const EmscriptenUiEvent, ud: *mut CVoid) -> i32 {
	let event_queue: &mut Vec<Event> = std::mem::transmute(ud);

	js! { b"Module.canvas = document.getElementById('canvas')\0" };

	let mut screen_size = Vec2i::zero();
	screen_size.x = js! { b"return (Module.canvas.width = Module.canvas.style.width = window.innerWidth)\0" };
	screen_size.y = js! { b"return (Module.canvas.height = Module.canvas.style.height = window.innerHeight)\0" };

	event_queue.push(Event::Resize(screen_size));
	
	0
}

unsafe extern "C"
fn on_click(_: i32, e: *const EmscriptenMouseEvent, ud: *mut CVoid) -> i32 {
	let event_queue: &mut Vec<Event> = std::mem::transmute(ud);
	let e: &EmscriptenMouseEvent = std::mem::transmute(e);

	event_queue.push(Event::Click(Vec2i::new(e.clientX as _, e.clientY as _)));
	
	0
}

