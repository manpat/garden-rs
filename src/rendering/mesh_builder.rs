#![allow(dead_code)]

use std::mem::size_of;
use rendering::gl;
use rendering::types::*;

pub struct VertexAttributeBinding {
	pub index: u32,
	pub width: i32,
	pub offset: u32,
}

pub struct VertexLayout {
	pub size: u32,
	pub attributes: Vec<VertexAttributeBinding>,
}

impl VertexLayout {
	pub fn new<V: Vertex>() -> Self {
		VertexLayout {
			size: size_of::<V>() as _,
			attributes: Vec::new()
		}
	}

	pub fn null() -> Self {
		VertexLayout { size: 0, attributes: Vec::new() }
	}

	pub fn add_binding(mut self, index: u32, width: i32, offset: u32) -> Self {
		self.attributes.push(VertexAttributeBinding{index, width, offset});
		self
	}
}

pub trait Vertex: Copy + Clone {
	fn get_layout() -> VertexLayout;
}



#[derive(Copy, Clone)]
pub struct DefaultVertex {
	pos: Vec3,
}

impl DefaultVertex {
	pub fn new(pos: Vec3) -> Self {
		DefaultVertex{pos}
	}
}

impl Vertex for DefaultVertex {
	fn get_layout() -> VertexLayout {
		VertexLayout::new::<Self>()
			.add_binding(0, 3, 0)
	}
}



pub struct Mesh {
	pub vbo: u32,
	pub count: u32,
	pub layout: VertexLayout,
}

impl Mesh {
	pub fn new() -> Self {
		Mesh {
			vbo: gl::pls_make_buffer(),
			count: 0,
			layout: VertexLayout::null(),
		}
	}

	pub fn bind(&self) {
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);

			for ab in self.layout.attributes.iter() {
				gl::EnableVertexAttribArray(ab.index);
				gl::VertexAttribPointer(ab.index, ab.width, gl::FLOAT, gl::FALSE, self.layout.size as i32, ab.offset as _);
			}

			// gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, Vertex::get_size() as _, 0 as _);
			// gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, Vertex::get_size() as _, 12 as _);
			// gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, Vertex::get_size() as _, 24 as _);
		}
	}

	pub fn draw(&self, mode: u32) {
		unsafe {
			gl::DrawArrays(mode, 0, self.count as _);
		}
	}
}



pub struct MeshBuilder<V: Vertex> {
	verts: Vec<V>,
}

impl<V> MeshBuilder<V> where V: Vertex {
	pub fn new() -> Self {
		MeshBuilder {
			verts: Vec::new(),
		}
	}

	pub fn clear(&mut self) {
		self.verts.clear();
	}

	pub fn upload_to(&self, mesh: &mut Mesh) {
		unsafe {
			mesh.layout = V::get_layout();
			mesh.count = self.verts.len() as _;
			let size = mesh.layout.size * mesh.count;

			gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);
			gl::BufferData(gl::ARRAY_BUFFER, size as _, self.verts.as_ptr() as _, gl::STATIC_DRAW);
		}
	}

	pub fn add_vert(&mut self, v: V) {
		self.verts.push(v);
	}

	pub fn add_quad(&mut self, vs: &[V]) {
		assert!(vs.len() >= 4);

		self.verts.push(vs[0]);
		self.verts.push(vs[1]);
		self.verts.push(vs[2]);

		self.verts.push(vs[0]);
		self.verts.push(vs[2]);
		self.verts.push(vs[3]);
	}

	pub fn add_convex_poly(&mut self, vs: &[V]) {
		assert!(vs.len() >= 3);

		for i in 1..vs.len()-1 {
			self.verts.push(vs[0]);
			self.verts.push(vs[i]);
			self.verts.push(vs[i+1]);
		}
	}
}