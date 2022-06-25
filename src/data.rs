use serde::Deserialize;
use std::num::ParseIntError;

pub struct RGBColor {
	red: u8,
	green: u8,
	blue: u8,
}

impl ToString for RGBColor {
	fn to_string(&self) -> String {
		format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
	}
}

impl<'a> TryFrom<&'a str> for RGBColor {
	type Error = ParseIntError;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		let s = value.trim_start_matches("#");

		let (redvalue, xs) = s.split_at(2);
		let (greenvalue, xs) = xs.split_at(2);
		let (bluevalue, _) = xs.split_at(2);

		let red = u8::from_str_radix(redvalue, 16)?;
		let green = u8::from_str_radix(greenvalue, 16)?;
		let blue = u8::from_str_radix(bluevalue, 16)?;
		Ok(Self { red, green, blue })
	}
}

/// 颜色属性
#[derive(Deserialize)]
pub struct ColorProp {
	pub id: String,
	pub hex: String,
	pub name: String,
	pub intro: String,
}

/// 颜色结点
#[derive(Deserialize)]
pub struct ColorNode {
	pub id: u8,
	pub name: String,
	pub hex: String,
	pub colors: Vec<ColorProp>,
}
