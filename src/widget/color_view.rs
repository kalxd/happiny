use gtk::prelude::*;
use gtk::{
	CellRendererText, ScrolledWindow, TreeModelFilter, TreeView, TreeViewColumn, TreeViewGridLines,
};

use std::cell::Cell;
use std::ops::Deref;
use std::rc::Rc;

use crate::store::ColorStore;

pub struct ColorView {
	view: TreeView,
	scroll_window: ScrolledWindow,
	_store: ColorStore,
	search_key: Rc<Cell<Option<String>>>,
	filter_model: TreeModelFilter,
}

impl ColorView {
	pub fn new() -> Self {
		let store = ColorStore::new();

		let filter_model = TreeModelFilter::new(&*store, None);

		let view = TreeView::builder()
			.enable_grid_lines(TreeViewGridLines::Horizontal)
			.enable_search(false)
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

		let myself = Self {
			view,
			scroll_window,
			_store: store,
			search_key: Default::default(),
			filter_model,
		};

		myself.connect_signal();

		return myself;
	}

	fn connect_signal(&self) {
		{
			self.filter_model.set_visible_func(|model, iter| {
				let name = model.value(iter, 1);
				println!("{:?}", name);
				return true;
			});
			self.filter_model.refilter();
		}
	}

	pub fn container(&self) -> &ScrolledWindow {
		&self.scroll_window
	}

	pub fn update_filter(&self, keyword: Option<&str>) {
		let keyword = keyword.map(String::from);
		self.search_key.replace(keyword);
		self.filter_model.refilter();
	}
}

impl Deref for ColorView {
	type Target = TreeView;

	fn deref(&self) -> &Self::Target {
		&self.view
	}
}
