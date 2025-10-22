use serde::Deserialize;

#[cxx::bridge(namespace = XGFFI)]
pub mod ffi {
	struct ColorItem {
		id: u32,
		name: String,
		pinyin: String,
		rgb: [u8; 3],
		hex: String,
		cmyk: [u32; 4],
	}

	extern "Rust" {
		#[cxx_name = "readColorItem"]
		fn read_color_item() -> Vec<ColorItem>;
	}
}

const JSON_DATA: &str = include_str!("../data/color.json");

#[derive(Deserialize)]
struct ColorItemWrapper {
	id: u32,
	#[serde(rename = "名称")]
	name: String,
	#[serde(rename = "拼音")]
	pinyin: String,
	#[serde(rename = "RGB")]
	rgb: (u8, u8, u8),
	#[serde(rename = "HEX")]
	hex: String,
	#[serde(rename = "CMYK")]
	cmyk: (u32, u32, u32, u32),
}

fn read_color_item() -> Vec<ffi::ColorItem> {
	let colors = serde_json::from_str::<Vec<ColorItemWrapper>>(JSON_DATA).unwrap();
	colors
		.into_iter()
		.map(|c| ffi::ColorItem {
			id: c.id,
			name: c.name,
			pinyin: c.pinyin,
			rgb: [c.rgb.0, c.rgb.1, c.rgb.2],
			hex: c.hex,
			cmyk: [c.cmyk.0, c.cmyk.1, c.cmyk.2, c.cmyk.3],
		})
		.collect()
}
