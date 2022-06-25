use gtk::{prelude::*, Clipboard, Menu, MenuItem};

use crate::data::RGBColor;

fn copy_text_to_clipboard(content: &str) {
	let clipboard = Clipboard::get(&gtk::gdk::SELECTION_CLIPBOARD);
	clipboard.set_text(&content);
}

pub struct ColorMenu {
	pub menu: Menu,
}

impl ColorMenu {
	pub fn new(name: String, hex: String) -> Option<Self> {
		let color = RGBColor::try_from(hex.as_ref()).ok()?;

		let menu = Menu::new();

		{
			let item = MenuItem::with_mnemonic(&format!("复制“{}”", name));
			menu.append(&item);
			item.connect_activate(move |_| {
				copy_text_to_clipboard(&name);
			});
		}

		{
			let text = format!("rgb({}, {}, {})", color.red, color.green, color.blue);
			let item = MenuItem::with_mnemonic(&format!("复制“{}”", &text));
			menu.append(&item);
			item.connect_activate(move |_| {
				copy_text_to_clipboard(&text);
			});
		}

		{
			let item = MenuItem::with_mnemonic(&format!("复制“{}”", &hex));
			menu.append(&item);
			item.connect_activate(move |_| {
				copy_text_to_clipboard(&hex);
			});
		}

		Some(Self { menu })
	}
}
