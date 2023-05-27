use std::cell::RefCell;
use std::rc::Rc;

use gtk::{
	glib,
	prelude::{
		GtkMenuExtManual, TreeModelExt, TreeModelFilterExt, TreeSelectionExt, TreeStoreExt,
		TreeStoreExtManual, TreeViewColumnExt, TreeViewExt, WidgetExt,
	},
};
use gtk::{
	CellRendererText, ScrolledWindow, TreeModelFilter, TreeStore, TreeView, TreeViewColumn,
	TreeViewGridLines,
};

use crate::data::ColorData;

// use super::colormenu;

const COL_TYPE: &[glib::types::Type; 7] = &[
	glib::types::Type::U32,
	glib::types::Type::STRING,
	glib::types::Type::STRING,
	glib::types::Type::STRING,
	glib::types::Type::STRING,
	glib::types::Type::STRING,
	glib::types::Type::STRING,
];

enum ColPosition {
	ID = 0,
	Name,
	Pinyin,
	RgbBackground,
	Rgb,
	Cmyk,
	Hex,
}

struct ColorStore(TreeStore);

impl ColorStore {
	fn new(colors: &[ColorData]) -> Self {
		let model = TreeStore::new(COL_TYPE);

		for color in colors {
			let iter = model.append(None);
			model.set(
				&iter,
				&[
					(ColPosition::ID as u32, &color.id),
					(ColPosition::Name as u32, &color.name),
					(ColPosition::Pinyin as u32, &color.pinyin),
					(ColPosition::RgbBackground as u32, &color.rgb),
					(ColPosition::Rgb as u32, &color.rgb),
					(ColPosition::Cmyk as u32, &color.cmyk),
					(ColPosition::Hex as u32, &color.hex),
				],
			);
		}

		return Self(model);
	}
}

enum TableAction {
	PopupMenu {
		color_name: String,
		color_hex: String,
		button: u32,
		time: u32,
	},
}

pub struct TableView {
	view: TreeView,
	filter_model: TreeModelFilter,
	pub layout: ScrolledWindow,
	search_keyword: Rc<RefCell<Option<String>>>,
}

impl TableView {
	pub fn new(colors: &[ColorData]) -> Self {
		let store = ColorStore::new(colors);
		let filter_model = TreeModelFilter::new(&store.0, None);

		let view = TreeView::builder()
			.enable_grid_lines(TreeViewGridLines::Horizontal)
			.enable_search(false)
			.model(&filter_model)
			.build();
		let layout = ScrolledWindow::builder().child(&view).build();

		let table = Self {
			view,
			layout,
			filter_model,
			search_keyword: Default::default(),
		};

		table.setup_columns();
		table.connect_signals();

		return table;
	}

	fn setup_columns(&self) {
		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("编号").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "text", ColPosition::ID as i32);
			self.view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("名称").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "text", ColPosition::Name as i32);
			self.view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("拼音").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "text", ColPosition::Pinyin as i32);
			self.view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("明亮亮的颜色").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "background", ColPosition::RgbBackground as i32);
			self.view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("RGB").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "text", ColPosition::Rgb as i32);
			self.view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("CMYK").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "text", ColPosition::Cmyk as i32);
			self.view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("HEX值").build();
			col.pack_start(&text, true);
			col.add_attribute(&text, "text", ColPosition::Hex as i32);
			self.view.append_column(&col);
		}
	}

	fn connect_signals(&self) {
		let (sender, receive) =
			gtk::glib::MainContext::channel::<TableAction>(gtk::glib::PRIORITY_DEFAULT);

		/*
		self.view.connect_button_release_event(move |view, event| {
			if event.button() == 3 {
				view.selection()
					.selected()
					.and_then(|(model, iter)| {
						let color = model
							.value(&iter, ColPosition::Color as i32)
							.get::<String>()
							.ok();
						let name = model
							.value(&iter, ColPosition::Name as i32)
							.get::<String>()
							.ok();
						return name.zip(color);
					})
					.and_then(|(name, hex)| {
						sender
							.send(TableAction::PopupMenu {
								color_name: name,
								color_hex: hex,
								button: event.button(),
								time: event.time(),
							})
							.ok()
					});
			}

			gtk::Inhibit(false)
		});

		let keyword = self.search_keyword.clone();
		self.filter_model.set_visible_func(move |model, iter| {
			keyword
				.borrow()
				.as_ref()
				.and_then(|keyword| {
					if model.iter_has_child(&iter) {
						return None;
					}

					model
						.value(&iter, ColPosition::Name as i32)
						.get::<String>()
						.ok()
						.map(|name| name.contains(keyword))
				})
				.unwrap_or(true)
		});

		receive.attach(None, |action| {
			match action {
				TableAction::PopupMenu {
					color_name,
					color_hex,
					button,
					time,
				} => {
					colormenu::ColorMenu::new(color_name, color_hex).map(|menu| {
						menu.menu.popup_easy(button, time);
						menu.menu.show_all();
					});
				}
			}
			gtk::glib::Continue(true)
		});
		 */
	}

	pub fn filter(&self, keyword: String) {
		if keyword.is_empty() {
			self.search_keyword.replace(None);
		} else {
			self.search_keyword.replace(Some(keyword));
		}

		self.filter_model.refilter();
		self.view.expand_all();
	}
}
