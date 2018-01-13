use mut_rc::*;

use coro_util::*;
use common::*;
use paper::*;

#[derive(Copy, Clone)]
enum FlowerType {
	Circle,
	Fern{count: i32, face_ratio: f32},
}

struct Flower {
	pos: Vec2,
	color: Vec3,
	stem_length: f32,
	stem_width: f32,
	face_radius: f32,

	variation: FlowerType,
}

pub struct FlowerManager {
	flower_descriptions: Vec<MutRc<Flower>>,
	flower_updates: Vec<Coro<()>>,
	wind_strength_phase: f32,
	wind_phase: f32,
}

impl FlowerManager {
	pub fn new() -> Self {
		FlowerManager{
			flower_descriptions: Vec::new(),
			flower_updates: Vec::new(),
			wind_strength_phase: 0.0,
			wind_phase: 0.0,
		}
	}

	pub fn draw(&self, paper: &mut Paper) {
		let yskew = (PI/5.0).sin();

		for flower in self.flower_descriptions.iter() {
			let flower = flower.borrow();

			let wind_variability = 1.0 / 4.0;
			let wind_variation = (flower.pos.x + flower.pos.y) * wind_variability;

			let wind_strength = ((self.wind_strength_phase - wind_variation) * PI * 2.0).cos() * 0.5 + 0.5;
			let wind_strength = wind_strength.ease_linear(0.2, 1.0);

			let wind_omega = (self.wind_phase - wind_variation) * PI * 2.0;
			let face_delay = 0.3;
			let sway_amt = PI/16.0 * wind_strength;

			let stem_ang = wind_omega.sin() * sway_amt + PI/2.0;
			let stem = flower.pos + Vec2::from_angle(stem_ang) * flower.stem_length;

			paper.build_ellipse(flower.pos + Vec2::new(0.0, -flower.face_radius * yskew / 3.0),
				Vec2::splat(flower.face_radius * 0.8) * Vec2::new(1.0, yskew),
				Vec4::new(0.0, 0.0, 0.0, 0.05));

			paper.build_line(&[flower.pos, stem], flower.stem_width, Vec4::new(0.62, 0.90, 0.60, 1.0));

			match flower.variation {
				FlowerType::Circle => {
					let face_ang = (wind_omega - face_delay).sin() * sway_amt + PI/2.0;
					let face = flower.pos + Vec2::from_angle(face_ang) * flower.stem_length;
					paper.build_circle(face, flower.face_radius, flower.color.extend(1.0));
				}

				FlowerType::Fern{count, face_ratio} => {
					let mut face = stem;
					let mut radius = flower.face_radius;

					for i in 0..count {
						let angle_inc = (wind_omega - face_delay * i as f32).sin() * sway_amt * (1.0 + i as f32 / 3.0);

						paper.build_circle(face, radius, flower.color.extend(1.0));
						face = face + Vec2::from_angle(angle_inc + PI/2.0) * radius * (0.8 + face_ratio);
						radius *= face_ratio;
					}
				}
			}
		}
	}

	pub fn update(&mut self) {
		let dt = 1.0 / 60.0;
		self.wind_phase += dt / 7.0;
		self.wind_strength_phase += dt / 13.0;

		self.wind_phase %= 1.0;
		self.wind_strength_phase %= 1.0;

		for coro in self.flower_updates.iter_mut() { coro.next(); }

		self.flower_updates.retain(Coro::is_valid);

		::console::set_section("Flowers", format!("{} flowers", self.flower_descriptions.len()));
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

		let variations = [
			FlowerType::Circle,
			FlowerType::Fern{
				count: rng.gen_range(2, 6),
				face_ratio: rng.gen_range(0.7, 1.0)
			},
		];

		let variation = *rng.choose(&variations).unwrap();

		let c0 = *rng.choose(&colors).unwrap();
		let c1 = *rng.choose(&colors).unwrap();

		let color = rng.next_f32().ease_linear(c0, c1);

		let flower = MutRc::new(Flower {
			pos, color,
			stem_length: 0.0,
			stem_width: 0.01,
			face_radius: 0.0,

			variation,
		});

		let target_face_radius: f32 = match variation {
			FlowerType::Circle =>			rng.gen_range(0.08, 0.11),
			FlowerType::Fern{..} =>			rng.gen_range(0.06, 0.09),
		};

		self.flower_updates.push( parameter_lerp!(flower.stem_width -> 0.05, 0.5, ease_back_out) );
		self.flower_updates.push( parameter_lerp!(flower.stem_length -> rng.gen_range(0.13, 0.2), 0.5, ease_back_out) );
		self.flower_updates.push( parameter_lerp!(flower.face_radius -> target_face_radius, 0.5, ease_back_out) );

		self.flower_descriptions.push(flower);
		self.flower_descriptions.sort_unstable_by(|a, b| b.borrow().pos.y.partial_cmp(&a.borrow().pos.y).unwrap());
	}
}