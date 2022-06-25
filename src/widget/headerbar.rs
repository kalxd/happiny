use gtk::{prelude::*, Entry, HeaderBar, Image, SearchBar, ToggleButton};

pub struct HeaderToolBar {
	pub header_bar: HeaderBar,
	pub toggle_search_btn: ToggleButton,
}

impl HeaderToolBar {
	pub fn new() -> Self {
		let header_bar = HeaderBar::builder()
			.show_close_button(true)
			.title("中国传统颜色搭取器")
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

pub struct HeaderSearchBar {
	pub search_bar: SearchBar,
}

impl HeaderSearchBar {
	pub fn new() -> Self {
		let entry = Entry::builder().placeholder_text("颜色").build();

		let search_bar = SearchBar::builder().show_close_button(true).build();
		search_bar.connect_entry(&entry);
		search_bar.add(&entry);

		Self { search_bar }
	}
}
