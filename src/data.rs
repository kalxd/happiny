use gtk::{glib, prelude::ToValue};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct RGB(u8, u8, u8);

impl ToValue for RGB {
	fn to_value(&self) -> glib::Value {
		format!("rgb({}, {}, {})", self.0, self.1, self.2).to_value()
	}

	fn value_type(&self) -> glib::Type {
		glib::Type::STRING
	}
}

#[derive(Deserialize, Debug, Clone)]
pub struct CMYK(u8, u8, u8, u8);

impl ToValue for CMYK {
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
	pub rgb: RGB,
	#[serde(rename = "CMYK")]
	pub cmyk: CMYK,
}

pub struct RightMenuData<'a> {
	pub name: &'a str,
	pub rgb: &'a str,
	pub cmyk: &'a str,
	pub hex: &'a str,
}
