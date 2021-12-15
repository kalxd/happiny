#![windows_subsystem = "windows"]
use gtk::prelude::*;
use gtk::Application;

mod app;
mod store;
mod widget;

use app::App;

fn main() {
	let application = Application::builder()
		.application_id("com.xgley.chinese_color_picker")
		.build();

	application.connect_activate(|app| App::run(app.clone()));

	application.run();
}
