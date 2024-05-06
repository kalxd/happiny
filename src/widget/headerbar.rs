use async_channel::Sender;
use gtk::{prelude::*, HeaderBar, Image, SearchBar, SearchEntry, ToggleButton};

use super::action::AppAction;

pub struct HeaderToolBar {
	pub header_bar: HeaderBar,
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

pub struct HeaderSearchBar {
	pub search_bar: SearchBar,
}

impl HeaderSearchBar {
	pub fn new(sender: Sender<AppAction>) -> Self {
		let entry = SearchEntry::builder().placeholder_text("颜色").build();

		entry.connect_activate(move |entry| {
			let text = entry.text().as_str().trim().to_string();
			sender.try_send(AppAction::StartSearch(text)).unwrap();
		});

		let search_bar = SearchBar::builder().show_close_button(true).build();
		search_bar.connect_entry(&entry);
		search_bar.add(&entry);

		Self { search_bar }
	}
}
