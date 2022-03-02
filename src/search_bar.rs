use gtk::prelude::*;
use gtk::{SearchBar, SearchEntry};

use std::ops::Deref;

#[derive(Clone)]
pub struct TopSearchBar {
	entry: SearchEntry,
	search_bar: SearchBar,
}

impl TopSearchBar {
	pub fn new() -> Self {
		let entry = SearchEntry::new();

		let search_bar = SearchBar::builder().show_close_button(true).build();
		search_bar.connect_entry(&entry);
		search_bar.add(&entry);

		Self { entry, search_bar }
	}

	pub fn container(&self) -> &SearchBar {
		&self.search_bar
	}

	pub fn handle_event(&self, event: &gtk::gdk::EventKey) -> bool {
		self.search_bar.handle_event(&event)
	}
}

impl Deref for TopSearchBar {
	type Target = SearchEntry;

	fn deref(&self) -> &Self::Target {
		&self.entry
	}
}