use std::collections::HashMap;
use bindings::console::*;

enum ConsoleDirtiness {
	Clean,
	Buffer,
	Map,
}

pub struct Console {
	entries: HashMap<&'static str, String>,
	buffer: String,
	dirty: ConsoleDirtiness
}

impl Console {
	pub fn new() -> Self {
		unsafe { init_console(); }

		Console {
			entries: HashMap::new(),
			buffer: String::new(),
			dirty: ConsoleDirtiness::Clean,
		}
	}

	pub fn set_text(&mut self, s: &str) {
		self.buffer = String::from(s);
		self.dirty = ConsoleDirtiness::Buffer;
	}

	pub fn set_section(&mut self, sect: &'static str, s: &str) {
		self.entries.insert(sect, String::from(s));
		self.dirty = ConsoleDirtiness::Map;
	}

	pub fn update(&mut self) {
		use std::fmt::Write;
		use std::ffi::CString;
		use self::ConsoleDirtiness::*;
		
		match self.dirty {
			Buffer => unsafe {
				set_console_text(CString::new(self.buffer.as_str()).unwrap().as_ptr());
			}

			Map => unsafe {
				let buf = &mut self.buffer;
				buf.clear();

				for (k, v) in self.entries.iter() {
					write!(buf, "<h3>{}</h3><div>{}</div><br/>", k, v).unwrap();
				}

				set_console_text(CString::new(buf.as_str()).unwrap().as_ptr());
			}

			Clean => {}
		}

		self.dirty = Clean;
	}
}