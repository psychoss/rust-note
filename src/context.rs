use std::path::{Path,PathBuf};

pub struct Context {
	pub  root:PathBuf
}

impl Context {
	
	pub fn new<P:AsRef<Path>>(root:P)->Context{
		let mut path=PathBuf::new();
		path.push(root);
		Context{
			root:path
		}
	}
}
