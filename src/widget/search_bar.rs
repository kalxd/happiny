use gtk::prelude::*;
use gtk::{SearchBar, SearchEntry};

use std::ops::Deref;

pub struct TopSearchBar {
	_entry: SearchEntry,
	search_bar: SearchBar,
}

impl TopSearchBar {
	pub fn new() -> Self {
		let entry = SearchEntry::new();

		let search_bar = SearchBar::builder().show_close_button(true).build();
		search_bar.connect_entry(&entry);
		search_bar.add(&entry);

		Self {
			_entry: entry,
			search_bar,
		}
	}

	pub fn container(&self) -> &SearchBar {
		&self
	}
}

impl Deref for TopSearchBar {
	type Target = SearchBar;

	fn deref(&self) -> &Self::Target {
		&self.search_bar
	}
}
