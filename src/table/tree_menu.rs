use css_color_parser2::Color;
use gtk::prelude::*;
use gtk::{Menu, MenuItem};

use std::ops::Deref;

pub(super) struct TreeMenu {
	menu: Menu,
	hex_item: MenuItem,
	rgb_item: MenuItem,
	hex_text_item: MenuItem,
	color: Color,
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
	pub(super) fn new(color: Color) -> Self {
		let menu = Menu::new();

		let hex_item = MenuItem::with_mnemonic(&format!("复制{}", color_hex(&color)));
		menu.append(&hex_item);

		let rgb_item = MenuItem::with_mnemonic(&format!("复制{}", color_rgb(&color)));
		menu.append(&rgb_item);

		let hex_text_item = MenuItem::with_mnemonic(&format!("复制{}", color_hex_text(&color)));
		menu.append(&hex_text_item);

		Self {
			menu,
			hex_item,
			rgb_item,
			hex_text_item,
			color,
		}
	}

	pub(super) fn connect_activate<F: FnOnce(String) + Copy + 'static>(&self, f: F) {
		{
			let f = f.clone();
			let color = self.color.clone();
			self.hex_item.connect_activate(move |_| {
				f(color_hex(&color));
			});
		}

		{
			let f = f.clone();
			let color = self.color.clone();
			self.rgb_item.connect_activate(move |_| {
				f(color_rgb(&color));
			});
		}

		{
			let f = f.clone();
			let color = self.color.clone();
			self.hex_text_item.connect_activate(move |_| {
				f(color_hex_text(&color));
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
