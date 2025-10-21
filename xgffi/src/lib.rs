use serde::Deserialize;

#[cxx::bridge(namespace = XGFFI)]
pub mod ffi {
	extern "Rust" {
		type ColorItem;

		#[cxx_name = "readColorItem"]
		fn read_color_item() -> Vec<ColorItem>;

		#[cxx_name = "getId"]
		fn get_id(&self) -> u32;
	}
}

const JSON_DATA: &str = include_str!("../data/color.json");

#[derive(Deserialize)]
struct ColorItem {
	id: u32,
}

fn read_color_item() -> Vec<ColorItem> {
	serde_json::from_str(JSON_DATA).unwrap()
}

impl ColorItem {
	fn get_id(&self) -> u32 {
		self.id
	}
}
