use gtk::prelude::TreeModelExt;
use gtk::{TreeIter, TreeModel};

pub enum ColType {
	ColID = 0,
	ColName,
	ColHex,
	ColDesc,
}

pub fn selection_string(model: &TreeModel, iter: &TreeIter, col: ColType) -> Option<String> {
	let value = model.value(&iter, col as i32);
	value.get().ok()
}
