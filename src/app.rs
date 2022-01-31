use gtk::prelude::*;
use gtk::{AccelGroup, Application, ApplicationWindow, Box, Clipboard, Orientation};

use crate::search_bar::TopSearchBar;
use crate::table::ColorView;

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

		{
			let search_bar = search_bar.clone();
			window.connect_key_press_event(move |_, event| {
				let b = search_bar.handle_event(event);
				gtk::Inhibit(b)
			});
		}

		let color_view = ColorView::new();
		main_layout.pack_start(color_view.container(), true, true, 10);

		{
			let view = color_view.clone();
			let search_entry = (*search_bar).clone();
			search_entry.connect_search_changed(move |entry| {
				let text = entry.text();
				let text = text.as_str();

				let keyword = if text.is_empty() { None } else { Some(text) };

				view.update_filter(keyword);
			});

			let accel_group = AccelGroup::new();
			let (accel_key, accel_modifier) = gtk::accelerator_parse("<Control>c");
			let view = color_view.clone();
			accel_group.connect_accel_group(
				accel_key,
				accel_modifier,
				gtk::AccelFlags::empty(),
				move |_, _, _, _| {
					if let Some(hex) = view.selection_hex() {
						let clipboard = Clipboard::get(&gtk::gdk::SELECTION_CLIPBOARD);
						clipboard.set_text(&hex);
					}
					return true;
				},
			);
			window.add_accel_group(&accel_group);
		}

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
