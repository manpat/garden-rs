
use rendering::mesh_builder::*;
use rendering::gl;
use common::*;

#[allow(dead_code)]
#[derive(Copy, Clone)]
struct PaperVertex {
	pos: Vec2,
	color: Vec4,
}

impl PaperVertex {
	pub fn new(pos: Vec2, color: Vec4) -> Self {
		PaperVertex {pos, color}
	}
}

impl Vertex for PaperVertex {
	fn get_layout() -> VertexLayout {
		VertexLayout::new::<Self>()
			.add_binding(0, 2, 0)
			.add_binding(1, 4, 8)
	}
}



pub struct Paper {
	builder: MeshBuilder<PaperVertex>,
	mesh: Mesh,
}

impl Paper {
	pub fn new() -> Self {
		Paper {
			builder: MeshBuilder::new(),
			mesh: Mesh::new()
		}
	}

	pub fn clear(&mut self) {
		self.builder.clear();
	}

	pub fn draw(&mut self) {
		self.builder.upload_to(&mut self.mesh);
		self.mesh.bind();
		self.mesh.draw(gl::TRIANGLES);
	}

	pub fn build_line(&mut self, vs: &[Vec2], thickness: f32, color: Vec4) {
		assert!(vs.len() >= 2);

		let thickness = thickness / 2.0;
		let mut ns = Vec::new();

		for seg in vs.windows(2) {
			let (a, b) = (seg[0], seg[1]);
			let diff = b-a;
			let n = diff.perp().normalize();

			let vs = [
				PaperVertex::new((a + n * thickness), color),
				PaperVertex::new((a - n * thickness), color),
				PaperVertex::new((b - n * thickness), color),
				PaperVertex::new((b + n * thickness), color),
			];

			self.builder.add_quad(&vs);
			ns.push(n);
		}

		for &(vert, n) in [(vs[0],-ns[0]), (*vs.last().unwrap(), *ns.last().unwrap())].iter() {
			let n0 =  n;
			let n1 = -n;

			let nm = n1.perp();

			let diff0 = (nm-n0) / 4.0;
			let diff1 = (n1-nm) / 4.0;

			for &(start, diff) in [(n0, diff0), (nm, diff1)].iter() {
				for i in 0..4i32 {
					let nn0 = start + diff * i as f32;
					let nn1 = start + diff * (i + 1) as f32;

					let v0 = vert + nn0.normalize() * thickness;
					let v1 = vert + nn1.normalize() * thickness;

					self.builder.add_vert(PaperVertex::new(vert, color));
					self.builder.add_vert(PaperVertex::new(v0, color));
					self.builder.add_vert(PaperVertex::new(v1, color));
				}
			}
		}

		for (i, seg) in vs.windows(3).enumerate() {
			let vert = seg[1];
			let n0 = ns[i];
			let n1 = ns[i+1];

			let ang = n0.dot(n1);

			let under_side = n0.dot(n1.perp()) > 0.0;
			let (n0, n1) = if under_side { (n0, n1) } else { (-n1,-n0) };

			if ang > 0.0 {
				let diff = (n1-n0) / 3.0;

				for i in 0..3i32 {
					let nn0 = n0 + diff * i as f32;
					let nn1 = n0 + diff * (i + 1) as f32;

					let v0 = vert + nn0.normalize() * thickness;
					let v1 = vert + nn1.normalize() * thickness;

					self.builder.add_vert(PaperVertex::new(vert, color));
					self.builder.add_vert(PaperVertex::new(v0, color));
					self.builder.add_vert(PaperVertex::new(v1, color));
				}
			} else {
				let d0 = vert - seg[0];
				let d2 = vert - seg[2];
				let nm = (d0 + d2).normalize();

				let diff0 = (nm-n0) / 3.0;
				let diff1 = (n1-nm) / 3.0;

				for &(start, diff) in [(n0, diff0), (nm, diff1)].iter() {
					for i in 0..3i32 {
						let nn0 = start + diff * i as f32;
						let nn1 = start + diff * (i + 1) as f32;

						let v0 = vert + nn0.normalize() * thickness;
						let v1 = vert + nn1.normalize() * thickness;

						self.builder.add_vert(PaperVertex::new(vert, color));
						self.builder.add_vert(PaperVertex::new(v0, color));
						self.builder.add_vert(PaperVertex::new(v1, color));
					}
				}
			}
		}
	}

	pub fn build_circle(&mut self, p: Vec2, r: f32, color: Vec4) {
		self.build_oval(p, Vec2::splat(r), color);
	}

	pub fn build_oval(&mut self, p: Vec2, rs: Vec2, color: Vec4) {
		let mut vs = Vec::new();
		let steps = 36i32;

		let inc = PI * 2.0 / steps as f32;

		for i in 0..steps {
			let dir = Vec2::from_angle(inc * i as f32);
			let v = p + dir * rs;
			vs.push(PaperVertex::new(v, color));
		}

		self.builder.add_convex_poly(&vs);
	}
}