use gtk::prelude::*;
use gtk::ListStore;
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
	id: usize,
	name: String,
	rgb: (usize, usize, usize),
	hex: String,

	colors: Vec<ColorProp>,
}

impl ColorData {
	pub fn new() -> serde_json::Result<Self> {
		serde_json::from_str(JSON_DATA)
	}
}

pub struct ColorModel {
	model: ListStore,
}

impl Deref for ColorModel {
	type Target = ListStore;

	fn deref(&self) -> &Self::Target {
		&self.model
	}
}
