use gtk::gdk::EventKey;

pub enum AppAction {
	StartSearch(String),
	WindowKeyPress(EventKey),
}
