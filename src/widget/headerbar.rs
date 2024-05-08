use gtk::{prelude::*, HeaderBar, Image, ToggleButton};

pub struct HeaderToolBar {
	header_bar: HeaderBar,
	pub toggle_search_btn: ToggleButton,
}

impl HeaderToolBar {
	pub fn new() -> Self {
		let header_bar = HeaderBar::builder()
			.show_close_button(true)
			.title("中国传统色卡")
			.build();

		let toggle_search_btn = ToggleButton::builder()
			.image(&Image::from_icon_name(Some("find"), gtk::IconSize::Button))
			.tooltip_text("搜索颜色")
			.build();

		header_bar.pack_start(&toggle_search_btn);

		Self {
			header_bar,
			toggle_search_btn,
		}
	}
}

impl AsRef<HeaderBar> for HeaderToolBar {
	fn as_ref(&self) -> &HeaderBar {
		&self.header_bar
	}
}
