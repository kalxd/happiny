use gtk::{prelude::*, Image};
use gtk::{HeaderBar, ToggleButton};

pub struct HeaderToolBar {
	pub header_bar: HeaderBar,
}

impl HeaderToolBar {
	pub fn new() -> Self {
		let header_bar = HeaderBar::builder()
			.show_close_button(true)
			.title("中国传统颜色搭取器")
			.build();

		let find_button = ToggleButton::builder()
			.image(&Image::from_icon_name(Some("find"), gtk::IconSize::Button))
			.build();

		header_bar.pack_start(&find_button);

		Self { header_bar }
	}
}
