#![allow(non_snake_case)]

use Library::Fn::Cache;
use clap::{Command, arg};

fn main() {
	let matches = Command::new("📄 Document.")
		.version(env!("CARGO_PKG_VERSION"))
		.author("Nikola Hristov")
		.about("Build.")
		.get_matches();
}
