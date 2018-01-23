#![feature(generators, generator_trait, box_syntax)]
#![feature(inclusive_range_syntax)]
#![feature(specialization)]
#![feature(ord_max_min)]
#![feature(link_args)]
#![feature(const_fn)]

#[macro_use]
extern crate web_common;

pub use resources as res;
pub use web_common::*;
use web_common::events::*;

pub mod resources;
pub mod console;

pub mod flower;
pub mod particle;

use flower::*;
use particle::*;

use std::time::Instant;

fn main() {
	set_coro_as_main_loop(|| {
		console::init();
		console::set_color("#222");

		let _gl = WebGLContext::new(true);

		let mut events = Vec::new();

		unsafe {
			initialise_ems_event_queue(&mut events);

			gl::Enable(gl::BLEND);
			gl::BlendEquation(gl::FUNC_ADD);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
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

					_ => {}
				}
			}

			events.clear();

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
