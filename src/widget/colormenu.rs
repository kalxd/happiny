use gtk::{prelude::*, Menu};

use crate::data::RGBColor;

pub struct ColorMenu {
	pub menu: Menu,
	color: RGBColor,
}

impl ColorMenu {
	pub fn new<T: AsRef<str>>(color: T) -> Option<Self> {
		let color = RGBColor::try_from(color.as_ref()).ok()?;

		let menu = Menu::new();

		Some(Self { menu, color })
	}
}
