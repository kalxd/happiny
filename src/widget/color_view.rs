use gtk::prelude::*;
use gtk::{CellRendererText, ScrolledWindow, TreeView, TreeViewColumn, TreeViewGridLines};

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
		let view = TreeView::builder()
			.enable_grid_lines(TreeViewGridLines::Horizontal)
			.model(&*store)
			.build();
		let scroll_window = ScrolledWindow::builder().child(&view).build();

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("序号").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "text", 0);
			view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("名称").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "text", 1);
			view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("色值").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "text", 2);
			view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("明亮亮的颜色").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "background", 2);
			view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("描述").build();
			col.pack_start(&text, true);
			col.add_attribute(&text, "text", 3);
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
