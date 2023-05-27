use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct RGB(u8, u8, u8);

#[derive(Deserialize, Debug, Clone)]
pub struct CMKY(u8, u8, u8, u8);

#[derive(Deserialize, Debug, Clone)]
pub struct ColorData {
	id: usize,
	#[serde(rename = "名称")]
	name: String,
	#[serde(rename = "拼音")]
	pinyin: String,
	#[serde(rename = "HEX")]
	hex: String,
	#[serde(rename = "RGB")]
	rgb: RGB,
	#[serde(rename = "CMYK")]
	cmyk: CMKY,
}
