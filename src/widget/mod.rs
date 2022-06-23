use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

mod action;

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
