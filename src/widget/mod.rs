use futures::channel::mpsc::{channel, Receiver};
use futures::StreamExt;
use gtk::prelude::{BoxExt, ContainerExt, GtkWindowExt, SearchBarExt, WidgetExt};
use gtk::{glib, Application, ApplicationWindow, Box as GtkBox, Orientation};

use std::future::ready;
use std::sync::{Arc, Mutex};

use self::searchbar::HeaderSearchBar;
use crate::data::ColorData;

mod action;
mod colormenu;
mod headerbar;
mod searchbar;
mod tableview;

pub struct MainWindow {
	window: ApplicationWindow,
	table_view: tableview::TableView,
	header_search_bar: HeaderSearchBar,
	receiver: Receiver<action::AppAction>,
}

impl MainWindow {
	fn new(app: &Application) -> Self {
		let colors = ColorData::new();
		let (sender, receiver) = channel(10);
		let sender = Arc::new(Mutex::new(sender));

		let window = ApplicationWindow::builder()
			.application(app)
			.title("中国传统色卡")
			.icon_name("happiny")
			.default_height(800)
			.default_width(800)
			.build();

		let main_layout = GtkBox::new(Orientation::Vertical, 0);

		let header_tool_bar = headerbar::HeaderToolBar::new();
		window.set_titlebar(Some(header_tool_bar.as_ref()));

		let header_search_bar = searchbar::HeaderSearchBar::new(sender.clone());
		header_tool_bar.connect_searchbar(&header_search_bar);
		window.connect_key_press_event({
			move |_, event| {
				sender
					.lock()
					.unwrap()
					.try_send(action::AppAction::WindowKeyPress(event.clone()))
					.unwrap();
				glib::Propagation::Proceed
			}
		});
		main_layout.pack_start(header_search_bar.as_ref(), false, false, 0);

		let table_view = tableview::TableView::new(&colors);
		main_layout.pack_start(&table_view.layout, true, true, 0);

		window.add(&main_layout);

		Self {
			window,
			table_view,
			header_search_bar,
			receiver,
		}
	}

	pub fn run(app: &Application) {
		let app = Self::new(app);
		app.window.show_all();

		glib::MainContext::default().spawn_local(async move {
			app.receiver
				.for_each(move |msg| {
					match msg {
						action::AppAction::StartSearch(key) => {
							app.table_view.filter(key);
						}
						action::AppAction::WindowKeyPress(event) => {
							app.header_search_bar.handle_event(&event);
						}
					}

					ready(())
				})
				.await;
		});
	}
}
