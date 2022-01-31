use gtk::glib::types::Type;
use gtk::prelude::*;
use gtk::TreeStore;
use serde::Deserialize;

use std::ops::Deref;

use super::t;

const JSON_DATA: &'static str = include_str!("../../data/colors.json");

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

#[derive(Clone)]
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
					(t::ColType::ColID as u32, &(data.id.to_string())),
					(t::ColType::ColName as u32, &data.name),
					(t::ColType::ColHex as u32, &data.hex),
					(t::ColType::ColDesc as u32, &data.name),
				],
			);

			data.colors.iter().for_each(|prop| {
				let iter = model.append(Some(&iter));
				model.set(
					&iter,
					&[
						(t::ColType::ColID as u32, &prop.id),
						(t::ColType::ColName as u32, &prop.name),
						(t::ColType::ColHex as u32, &prop.hex),
						(t::ColType::ColDesc as u32, &prop.intro),
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
