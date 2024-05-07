use gtk::prelude::*;
use gtk::Application;

mod data;
mod widget;

fn main() {
	let application = Application::builder()
		.application_id("person.xgley.happiny")
		.build();
	application.connect_activate(widget::MainWindow::run);
	application.run();
}
