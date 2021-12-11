use gtk::prelude::*;
use gtk::TableView;

pub struct TableWidget {
	view: TableView,
}

impl TableWidget {
	pub fn new() -> Self {
		let view = TableView::new();

		Self { view }
	}
}
