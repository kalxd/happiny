use gtk::{glib, prelude::*, TreeStore};
use gtk::{CellRendererText, ScrolledWindow, TreeView, TreeViewColumn, TreeViewGridLines};

use crate::data::ColorNode;

const JSON_DATA: &'static str = include_str!("../../data/colors.json");

const COL_TYPE: &'static [glib::types::Type; 4] = &[
	glib::types::Type::STRING,
	glib::types::Type::STRING,
	glib::types::Type::STRING,
	glib::types::Type::STRING,
];

enum ColPosition {
	ID = 0,
	Name,
	Color,
	Intro,
}

struct ColorStore(TreeStore);

impl ColorStore {
	fn new() -> Self {
		let model = TreeStore::new(COL_TYPE);

		if let Err(x) = serde_json::from_str::<Vec<ColorNode>>(JSON_DATA) {
			dbg!(x);
		}

		serde_json::from_str::<Vec<ColorNode>>(JSON_DATA)
			.unwrap_or(vec![])
			.iter()
			.for_each(|data| {
				let iter = model.append(None);

				// 父结点
				model.set(
					&iter,
					&[
						(ColPosition::ID as u32, &data.id),
						(ColPosition::Name as u32, &data.name),
						(ColPosition::Color as u32, &data.hex),
						(ColPosition::Intro as u32, &data.name),
					],
				);

				data.colors.iter().for_each(|prop| {
					let iter = model.append(Some(&iter));
					model.set(
						&iter,
						&[
							(ColPosition::ID as u32, &prop.id),
							(ColPosition::Name as u32, &prop.name),
							(ColPosition::Color as u32, &prop.hex),
							(ColPosition::Intro as u32, &prop.intro),
						],
					);
				});
			});

		return Self(model);
	}
}

pub struct TableView {
	view: TreeView,
	store: ColorStore,
	pub layout: ScrolledWindow,
}

impl TableView {
	pub fn new() -> Self {
		let store = ColorStore::new();

		let view = TreeView::builder()
			.enable_grid_lines(TreeViewGridLines::Horizontal)
			.enable_search(false)
			.model(&store.0)
			.build();
		let layout = ScrolledWindow::builder().child(&view).build();

		let table = Self {
			view,
			store,
			layout,
		};

		table.setup_columns();

		return table;
	}

	fn setup_columns(&self) {
		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("序号").build();
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
			let col = TreeViewColumn::builder().title("色值").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "text", ColPosition::Color as i32);
			self.view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("明亮亮的颜色").build();
			col.pack_start(&text, false);
			col.add_attribute(&text, "background", ColPosition::Color as i32);
			self.view.append_column(&col);
		}

		{
			let text = CellRendererText::new();
			let col = TreeViewColumn::builder().title("详情说明").build();
			col.pack_start(&text, true);
			col.add_attribute(&text, "text", ColPosition::Intro as i32);
			self.view.append_column(&col);
		}
	}
}
