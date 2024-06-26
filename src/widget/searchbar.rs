use std::ops::Deref;
use std::sync::{Arc, Mutex};

use futures::channel::mpsc::Sender;
use gtk::{
	prelude::{ContainerExt, EntryExt, SearchBarExt},
	SearchBar, SearchEntry,
};

use super::action::AppAction;

pub struct HeaderSearchBar {
	search_bar: SearchBar,
}

impl HeaderSearchBar {
	pub fn new(sender: Arc<Mutex<Sender<AppAction>>>) -> Self {
		let entry = SearchEntry::builder().placeholder_text("颜色").build();

		entry.connect_activate({
			let sender = sender.clone();
			move |entry| {
				let text = entry.text().as_str().trim().to_string();
				sender
					.lock()
					.unwrap()
					.try_send(AppAction::StartSearch(text))
					.unwrap();
			}
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

impl Deref for HeaderSearchBar {
	type Target = SearchBar;

	fn deref(&self) -> &Self::Target {
		&self.search_bar
	}
}
