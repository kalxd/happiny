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

use super::colormenu;

const COL_TYPE: &[glib::types::Type; 7] = &[
	glib::types::Type::U32,
	glib::types::Type::STRING,
	glib::types::Type::STRING,
	glib::types::Type::STRING,
	glib::types::Type::STRING,
	glib::types::Type::STRING,
	glib::types::Type::STRING,
];

enum ColField {
	ID = 0,
	Name,
	Pinyin,
	RgbBackground,
	Rgb,
	Cmyk,
	Hex,
}

impl ColField {
	fn into_i32(self) -> i32 {
		self as i32
	}

	fn into_u32(self) -> u32 {
		self as u32
	}
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
					(ColField::ID.into_u32(), &color.id),
					(ColField::Name.into_u32(), &color.name),
					(ColField::Pinyin.into_u32(), &color.pinyin),
					(ColField::RgbBackground.into_u32(), &color.rgb),
					(ColField::Rgb.into_u32(), &color.rgb),
					(ColField::Cmyk.into_u32(), &color.cmyk),
					(ColField::Hex.into_u32(), &color.hex),
				],
			);
		}

		Self(model)
	}
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

		table
	}

	fn setup_columns(&self) {
		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("编号").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "text", ColField::ID.into_i32());
			self.view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("名称").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "text", ColField::Name.into_i32());
			self.view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("拼音").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "text", ColField::Pinyin.into_i32());
			self.view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("明亮亮的颜色").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "background", ColField::RgbBackground.into_i32());
			self.view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("RGB").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "text", ColField::Rgb.into_i32());
			self.view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("CMYK").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "text", ColField::Cmyk.into_i32());
			self.view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("HEX值").build();
			col.pack_start(&text, true);
			col.add_attribute(&text, "text", ColField::Hex.into_i32());
			self.view.append_column(&col);
		}
	}

	fn connect_signals(&self) {
		let keyword = self.search_keyword.clone();
		self.filter_model.set_visible_func(move |model, iter| {
			keyword
				.borrow()
				.as_ref()
				.and_then(|keyword| {
					model
						.value(iter, ColField::Name.into_i32())
						.get::<String>()
						.ok()
						.map(|name| name.contains(keyword))
				})
				.unwrap_or(true)
		});

		self.view.connect_button_release_event(move |view, event| {
			if event.button() == 3 {
				if let Some((model, iter)) = view.selection().selected() {
					let name = model.value(&iter, ColField::Name as i32);
					let name = name.get().expect("name is empty");

					let rgb = model.value(&iter, ColField::Rgb as i32);
					let rgb = rgb.get().expect("rgb is empty");

					let hex = model.value(&iter, ColField::Hex as i32);
					let hex = hex.get().expect("hex is empty");

					let cmyk = model.value(&iter, ColField::Cmyk as i32);
					let cmyk = cmyk.get().expect("cmyk is empty");

					let menu = colormenu::ColorMenu::new(&[name, hex, rgb, cmyk]);
					menu.menu.popup_easy(event.button(), event.time());
					menu.menu.show_all();
				}
			}

			glib::Propagation::Proceed
		});
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
