#[cxx::bridge]
pub mod ffi {
	extern "Rust" {
		fn say_hello();
	}
}

fn say_hello() {
	println!("你好啊");
}
