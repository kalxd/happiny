use gtk::prelude::*;
use gtk::Application;

mod data;
mod widget;

const JSON_DATA: &str = include_str!("../data/color.json");

fn main() {
	let colors: Vec<data::ColorData> = serde_json::from_str(JSON_DATA).expect("颜色初始化失败！");

	let application = Application::builder()
		.application_id("com.xgley.chinese_color_picker")
		.build();

	application.connect_activate(move |app| widget::MainWindow::run(app, &colors));

	application.run();
}
