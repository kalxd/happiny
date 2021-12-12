use glib::types::Type;
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

const COL_TYPE: &'static [Type; 4] = &[Type::STRING, Type::STRING, Type::STRING, Type::STRING];

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
				&[(0, &(data.id.to_string())), (1, &data.name), (2, &data.hex)],
			);
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
