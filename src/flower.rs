use std::cell::RefCell;
use std::rc::Rc;

use coro_util::*;
use common::*;
use paper::*;

struct Flower {
	pos: Vec2,
	color: Vec3,
	stem_length: f32,
	stem_width: f32,
	face_radius: f32,
}

macro_rules! parameter_lerp {
	( $rc_obj:ident.$param:ident -> $to:expr, $duration:expr, $ease:ident ) => {{
		let rc_obj = $rc_obj.clone();

		let from = rc_obj.borrow().$param;
		let to = $to;

		let num_frames = ($duration * 60.0) as u32;

		Coro::from(move || {
			for i in 0..num_frames {
				let prog = i as f32 / num_frames as f32;
				rc_obj.borrow_mut().$param = prog.$ease(from, to);
				yield;
			}
		})
	}};

	( $rc_obj:ident.$param:ident -> $to:expr, $duration:expr ) => {{
		parameter_lerp!( $rc_obj.$param -> $to, $duration, ease_linear )
	}};
}

pub struct FlowerManager {
	flower_descriptions: Vec<Rc<RefCell<Flower>>>,
	flower_updates: Vec<Coro<()>>,
	wind_phase: f32,
}

impl FlowerManager {
	pub fn new() -> Self {
		FlowerManager{
			flower_descriptions: Vec::new(),
			flower_updates: Vec::new(),
			wind_phase: 0.0,
		}
	}

	pub fn draw(&self, paper: &mut Paper) {
		let yskew = (PI/5.0).sin();

		for flower in self.flower_descriptions.iter() {
			let flower = flower.borrow();

			let wind_omega = (self.wind_phase - flower.pos.x - flower.pos.y) * PI * 2.0;
			let face_delay = 0.3;
			let sway_amt = PI/16.0;

			let stem_ang = wind_omega.sin() * sway_amt + PI/2.0;
			let face_ang = (wind_omega - face_delay).sin() * sway_amt + PI/2.0;

			let stem = flower.pos + Vec2::from_angle(stem_ang) * flower.stem_length;
			let face = flower.pos + Vec2::from_angle(face_ang) * flower.stem_length;

			paper.build_oval(flower.pos + Vec2::new(0.0, -flower.face_radius * yskew / 3.0),
				Vec2::splat(flower.face_radius * 0.6) * Vec2::new(1.0, yskew),
				Vec4::new(0.0, 0.0, 0.0, 0.05));

			paper.build_line(&[flower.pos, stem], flower.stem_width, Vec4::new(0.62, 0.90, 0.60, 1.0));
			paper.build_circle(face, flower.face_radius, flower.color.extend(1.0));
		}
	}

	pub fn update(&mut self) {
		let dt = 1.0 / 60.0;
		self.wind_phase += dt / 7.0;
		self.wind_phase = self.wind_phase % 1.0;

		for coro in self.flower_updates.iter_mut() { coro.next(); }

		self.flower_updates.retain(|c| c.valid);
	}

	pub fn add_flower(&mut self, pos: Vec2) {
		use rand::{thread_rng, Rng};

		let mut rng = thread_rng();
		let colors = [
			Vec3::new(0.92, 0.52, 0.53),
			Vec3::new(0.93, 0.85, 0.45),
			Vec3::new(1.00, 0.80, 0.50),
			Vec3::new(0.87, 0.69, 0.85),
			Vec3::new(0.74, 0.69, 0.85),
			Vec3::new(0.60, 0.76, 0.85),
		];

		let c0 = *rng.choose(&colors).unwrap();
		let c1 = *rng.choose(&colors).unwrap();

		let color = rng.next_f32().ease_linear(c0, c1);

		let flower = Rc::new(RefCell::new(Flower {
			pos, color,
			stem_length: 0.0,
			stem_width: 0.01,
			face_radius: 0.0,
		}));

		self.flower_descriptions.push(flower.clone());
		self.flower_descriptions.sort_unstable_by(|a, b| b.borrow().pos.y.partial_cmp(&a.borrow().pos.y).unwrap());

		self.flower_updates.push( parameter_lerp!(flower.stem_length -> 0.2, 0.5, ease_back_out) );
		self.flower_updates.push( parameter_lerp!(flower.stem_width -> 0.05, 0.5, ease_back_out) );
		self.flower_updates.push( parameter_lerp!(flower.face_radius -> 0.1, 0.5, ease_back_out) );
	}
}