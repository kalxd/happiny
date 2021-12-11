use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Orientation, ScrolledWindow, TreeView};

use crate::widget::search_bar::TopSearchBar;

pub struct App {
	window: ApplicationWindow,
}

impl App {
	fn new(application: Application) -> Self {
		let window = ApplicationWindow::builder()
			.application(&application)
			.title("中国传统颜色拾取器")
			.default_width(800)
			.default_height(600)
			.build();

		let main_layout = Box::builder()
			.orientation(Orientation::Vertical)
			.spacing(10)
			.build();

		let search_bar = TopSearchBar::new();
		main_layout.pack_start(search_bar.container(), false, false, 0);

		let table_view = TreeView::new();
		let scroll_view = ScrolledWindow::builder().child(&table_view).build();
		main_layout.pack_start(&scroll_view, true, true, 0);

		window.connect_key_press_event(move |_, event| {
			let b = search_bar.handle_event(event);
			gtk::Inhibit(b)
		});
		window.add(&main_layout);

		Self { window }
	}

	fn show(&self) {
		self.window.show_all();
	}

	pub fn run(app: Application) {
		let app = App::new(app);

		app.show();
	}
}
