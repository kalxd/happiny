use css_color_parser2::Color;
use gtk::prelude::*;
use gtk::{Menu, MenuItem, TreeIter, TreeModel};

use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;

use super::t::{selection_string, ColType};

pub(super) struct TreeMenu {
	menu: Menu,
	hex_item: MenuItem,
	hex_value: Rc<String>,
	rgb_item: MenuItem,
	rgb_value: Rc<String>,
	hex_text_item: MenuItem,
	hex_text_value: Rc<String>,
	name_item: MenuItem,
	name_value: Rc<String>,
}

fn color_hex(color: &Color) -> String {
	format!("#{}", color_hex_text(color))
}

fn color_rgb(Color { r, g, b, .. }: &Color) -> String {
	format!("rgb({}, {}, {})", r, g, b)
}

fn color_hex_text(Color { r, g, b, .. }: &Color) -> String {
	format!("{:02X}{:02X}{:02X}", r, g, b)
}

impl TreeMenu {
	pub(super) fn new(model: &TreeModel, iter: &TreeIter) -> Option<Self> {
		let menu = Menu::new();

		let color_value = selection_string(&model, &iter, ColType::ColHex)?;
		let color = Color::from_str(&color_value).ok()?;

		let hex_value = color_hex(&color);
		let hex_item = MenuItem::with_mnemonic(&format!("复制{}", &hex_value));
		menu.append(&hex_item);

		let rgb_value = color_rgb(&color);
		let rgb_item = MenuItem::with_mnemonic(&format!("复制{}", &rgb_value));
		menu.append(&rgb_item);

		let hex_text_value = color_hex_text(&color);
		let hex_text_item = MenuItem::with_mnemonic(&format!("复制{}", &hex_text_value));
		menu.append(&hex_text_item);

		let name_value = selection_string(&model, &iter, ColType::ColName)?;
		let name_item = MenuItem::with_mnemonic(&format!("复制{}", &name_value));
		menu.append(&name_item);

		Some(Self {
			menu,
			hex_item,
			hex_value: Rc::new(hex_value),
			rgb_item,
			rgb_value: Rc::new(rgb_value),
			hex_text_item,
			hex_text_value: Rc::new(hex_text_value),
			name_item,
			name_value: Rc::new(name_value),
		})
	}

	pub(super) fn connect_activate<F: Fn(String) + Copy + 'static>(&self, f: F) {
		{
			let value = self.hex_value.clone();
			self.hex_item.connect_activate(move |_| {
				f(value.to_string());
			});
		}

		{
			let value = self.rgb_value.clone();
			self.rgb_item.connect_activate(move |_| {
				f(value.to_string());
			});
		}

		{
			let value = self.hex_text_value.clone();
			self.hex_text_item.connect_activate(move |_| {
				f(value.to_string());
			});
		}

		{
			let value = self.name_value.clone();
			self.name_item.connect_activate(move |_| {
				f(value.to_string());
			});
		}
	}
}

impl Deref for TreeMenu {
	type Target = Menu;

	fn deref(&self) -> &Self::Target {
		&self.menu
	}
}
