use gtk::{prelude::*, Clipboard, Menu, MenuItem};

fn copy_text_to_clipboard<S: AsRef<str>>(content: S) {
	let clipboard = Clipboard::get(&gtk::gdk::SELECTION_CLIPBOARD);
	clipboard.set_text(content.as_ref());
}

pub struct ColorMenu {
	pub menu: Menu,
}

impl ColorMenu {
	pub fn new(items: &[&str]) -> Self {
		let menu = Menu::new();

		for item in items {
			let item = MenuItem::with_label(item);
			menu.append(&item);
			item.connect_activate(move |item| {
				if let Some(text) = item.label() {
					copy_text_to_clipboard(&text);
				}
			});
		}

		Self { menu }
	}
}
