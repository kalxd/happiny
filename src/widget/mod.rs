use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box as GtkBox, Orientation};

mod action;
mod headerbar;
mod tableview;

use self::action::Action;

pub struct MainWindow {
	window: ApplicationWindow,
	receiver: glib::Receiver<Action>,
}

impl MainWindow {
	pub fn new(app: &Application) -> Self {
		let (_, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

		let window = ApplicationWindow::builder()
			.application(app)
			.title("中国传统颜色搭取器")
			.default_height(800)
			.default_width(800)
			.build();

		let main_layout = GtkBox::new(Orientation::Vertical, 0);

		let header_tool_bar = headerbar::HeaderToolBar::new();
		window.set_titlebar(Some(&header_tool_bar.header_bar));

		let header_search_bar = headerbar::HeaderSearchBar::new();
		header_tool_bar
			.toggle_search_btn
			.bind_property(
				"active",
				&header_search_bar.search_bar,
				"search-mode-enabled",
			)
			.flags(glib::BindingFlags::BIDIRECTIONAL)
			.build();
		main_layout.pack_start(&header_search_bar.search_bar, false, false, 0);

		let table_view = tableview::TableView::new();
		main_layout.pack_start(&table_view.layout, true, true, 0);

		window.add(&main_layout);

		Self { window, receiver }
	}

	pub fn run(app: &Application) {
		let main_window = Self::new(app);
		main_window.window.show_all();

		main_window.receiver.attach(None, |_| {
			dbg!("hello world");
			glib::Continue(true)
		});
	}
}
