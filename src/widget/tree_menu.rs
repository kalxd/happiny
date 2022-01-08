use gtk::prelude::*;
use gtk::{Menu, MenuItem};

use std::ops::Deref;

pub(super) struct TreeMenu {
	menu: Menu,
	hex_item: MenuItem,
	rgb_item: MenuItem,
	hex_text_item: MenuItem,
}

impl TreeMenu {
	pub(super) fn new(text: &str) -> Self {
		let menu = Menu::new();

		let hex_item = MenuItem::with_mnemonic(&format!("{}(&h)", text));
		menu.append(&hex_item);

		let rgb_item = MenuItem::with_mnemonic(&format!("{}(&r)", text));
		menu.append(&rgb_item);

		let hex_text_item = MenuItem::with_mnemonic(&format!("{}(&c)", text));
		menu.append(&hex_text_item);

		Self {
			menu,
			hex_item,
			rgb_item,
			hex_text_item,
		}
	}

	pub(super) fn connect_activate<F: FnOnce(String) + Copy + 'static>(&self, f: F) {
		{
			let f = f.clone();
			self.hex_item.connect_activate(move |_| {
				f("hello".into());
			});
		}

		{
			let f = f.clone();
			self.rgb_item.connect_activate(move |_| {
				f("world".into());
			});
		}

		{
			let f = f.clone();
			self.hex_text_item.connect_activate(move |_| {
				f("sb".into());
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
