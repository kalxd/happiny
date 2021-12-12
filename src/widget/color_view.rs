use gtk::prelude::*;
use gtk::{CellRendererText, ScrolledWindow, TreeView, TreeViewColumn};

use std::ops::Deref;

use crate::store::ColorStore;

pub struct ColorView {
	view: TreeView,
	scroll_window: ScrolledWindow,
	_store: ColorStore,
}

impl ColorView {
	pub fn new() -> Self {
		let store = ColorStore::new();
		let view = TreeView::with_model(&*store);
		let scroll_window = ScrolledWindow::builder().child(&view).build();

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("id").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "text", 0);
			view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("name").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "text", 1);
			col.add_attribute(&text, "placeholder-text", 0);
			view.append_column(&col);
		}

		Self {
			view,
			scroll_window,
			_store: store,
		}
	}

	pub fn container(&self) -> &ScrolledWindow {
		&self.scroll_window
	}
}

impl Deref for ColorView {
	type Target = TreeView;

	fn deref(&self) -> &Self::Target {
		&self.view
	}
}
