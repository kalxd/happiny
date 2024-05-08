use async_channel::Sender;
use gtk::{
	prelude::{ContainerExt, EntryExt, SearchBarExt},
	SearchBar, SearchEntry,
};

use super::action::AppAction;

pub struct HeaderSearchBar {
	search_bar: SearchBar,
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

impl AsRef<SearchBar> for HeaderSearchBar {
	fn as_ref(&self) -> &SearchBar {
		&self.search_bar
	}
}
