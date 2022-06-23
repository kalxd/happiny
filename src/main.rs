use gtk::prelude::*;
use gtk::Application;

/*
mod app;
mod search_bar;
mod table;
*/

// use app::App;

mod widget;

fn main() {
	let application = Application::builder()
		.application_id("com.xgley.chinese_color_picker")
		.build();

	application.connect_activate(widget::MainWindow::run);

	application.run();
}
