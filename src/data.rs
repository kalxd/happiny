use gtk::{glib, prelude::ToValue};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Rgb(u8, u8, u8);

impl ToValue for Rgb {
	fn to_value(&self) -> glib::Value {
		format!("rgb({}, {}, {})", self.0, self.1, self.2).to_value()
	}

	fn value_type(&self) -> glib::Type {
		glib::Type::STRING
	}
}

#[derive(Deserialize, Debug, Clone)]
pub struct Cmyk(u8, u8, u8, u8);

impl ToValue for Cmyk {
	fn to_value(&self) -> glib::Value {
		format!("CMYK({}, {}, {}, {})", self.0, self.1, self.2, self.3).to_value()
	}

	fn value_type(&self) -> glib::Type {
		glib::Type::STRING
	}
}

#[derive(Deserialize, Debug, Clone)]
pub struct ColorData {
	pub id: u32,
	#[serde(rename = "名称")]
	pub name: String,
	#[serde(rename = "拼音")]
	pub pinyin: String,
	#[serde(rename = "HEX")]
	pub hex: String,
	#[serde(rename = "RGB")]
	pub rgb: Rgb,
	#[serde(rename = "CMYK")]
	pub cmyk: Cmyk,
}

impl ColorData {
	pub fn new() -> Vec<Self> {
		const JSON_DATA: &str = include_str!("../data/color.json");
		serde_json::from_str(JSON_DATA).expect("颜色初始化失败！")
	}
}
