use gtk::glib::types::Type;
use gtk::prelude::*;
use gtk::TreeStore;
use serde::Deserialize;

use std::ops::Deref;

const JSON_DATA: &'static str = include_str!("../data/colors.json");

/// 颜色属性
#[derive(Deserialize)]
struct ColorProp {
	id: String,
	hex: String,
	name: String,
	intro: String,
}

/// 颜色分类对象
#[derive(Deserialize)]
struct ColorData {
	id: u8,
	name: String,
	hex: String,
	colors: Vec<ColorProp>,
}

impl ColorData {
	pub fn new() -> serde_json::Result<Vec<Self>> {
		serde_json::from_str(JSON_DATA)
	}
}

pub struct ColorStore(TreeStore);

const COL_TYPE: &'static [Type; 4] = &[
	// id
	Type::STRING,
	// 名称
	Type::STRING,
	// 色值
	Type::STRING,
	// 描述
	Type::STRING,
];

impl ColorStore {
	pub fn new() -> Self {
		let store = Self::create_store();
		Self(store)
	}

	fn create_store() -> TreeStore {
		let model = TreeStore::new(COL_TYPE);
		ColorData::new().unwrap_or(vec![]).iter().for_each(|data| {
			let iter = model.append(None);
			model.set(
				&iter,
				&[
					(0, &(data.id.to_string())),
					(1, &data.name),
					(2, &data.hex),
					(3, &data.name),
				],
			);

			data.colors.iter().for_each(|prop| {
				let iter = model.append(Some(&iter));
				model.set(
					&iter,
					&[
						(0, &prop.id),
						(1, &prop.name),
						(2, &prop.hex),
						(3, &prop.intro),
					],
				);
			});
		});

		return model;
	}
}

impl Deref for ColorStore {
	type Target = TreeStore;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
