use gtk::prelude::{BoxExt, ContainerExt, GtkWindowExt, ObjectExt, WidgetExt};
use gtk::{glib, Application, ApplicationWindow, Box as GtkBox, Orientation};

use crate::data::ColorData;

mod action;
mod colormenu;
mod headerbar;
mod tableview;

pub struct MainWindow {
	window: ApplicationWindow,
	table_view: tableview::TableView,
	receiver: glib::Receiver<action::AppAction>,
}

impl MainWindow {
	fn new(app: &Application, colors: &[ColorData]) -> Self {
		let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

		let window = ApplicationWindow::builder()
			.application(app)
			.title("中国传统色卡")
			.icon_name("happiny")
			.default_height(800)
			.default_width(800)
			.build();

		let main_layout = GtkBox::new(Orientation::Vertical, 0);

		let header_tool_bar = headerbar::HeaderToolBar::new();
		window.set_titlebar(Some(&header_tool_bar.header_bar));

		let header_search_bar = headerbar::HeaderSearchBar::new(sender);
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

		let table_view = tableview::TableView::new(colors);
		main_layout.pack_start(&table_view.layout, true, true, 0);

		window.add(&main_layout);

		Self {
			window,
			table_view,
			receiver,
		}
	}

	pub fn run(app: &Application, colors: &[ColorData]) {
		let app = Self::new(app, colors);
		app.window.show_all();

		app.receiver.attach(None, move |action| {
			match action {
				action::AppAction::StartSearch(key) => {
					app.table_view.filter(key);
				}
			}
			glib::Continue(true)
		});
	}
}

/*
mod action;
mod colormenu;
mod headerbar;
mod tableview;

use self::action::AppAction;
use tableview::TableView;

pub struct MainWindow {
	window: ApplicationWindow,
	table_view: TableView,
	receiver: glib::Receiver<AppAction>,
}

impl MainWindow {
	pub fn new(app: &Application) -> Self {
		let (sender, receiver) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

		let window = ApplicationWindow::builder()
			.application(app)
			.title("中国传统颜色搭取器")
			.icon_name("happiny")
			.default_height(800)
			.default_width(800)
			.build();

		let main_layout = GtkBox::new(Orientation::Vertical, 0);

		let header_tool_bar = headerbar::HeaderToolBar::new();
		window.set_titlebar(Some(&header_tool_bar.header_bar));

		let header_search_bar = headerbar::HeaderSearchBar::new(sender);
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

		let table_view = TableView::new();
		main_layout.pack_start(&table_view.layout, true, true, 0);

		window.add(&main_layout);

		Self {
			window,
			receiver,
			table_view,
		}
	}

	pub fn run(app: &Application) {
		let main_window = Self::new(app);
		main_window.window.show_all();

		main_window.receiver.attach(None, move |action| {
			match action {
				AppAction::StartSearch(key) => {
					main_window.table_view.filter(key);
				}
			}
			glib::Continue(true)
		});
	}
}
*/
