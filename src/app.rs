use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Orientation};

use crate::widget::color_view::ColorView;
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
			.spacing(0)
			.build();

		let search_bar = TopSearchBar::new();
		main_layout.pack_start(search_bar.container(), false, false, 0);

		window.connect_key_press_event(move |_, event| {
			let b = search_bar.handle_event(event);
			gtk::Inhibit(b)
		});

		let color_view = ColorView::new();
		main_layout.pack_start(color_view.container(), true, true, 10);

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
