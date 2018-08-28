extern crate fasttyping;

use std::env;
use std::process;
use fasttyping::Config;

fn main() {
	let args: Vec<String> = env::args().collect();

	let config = Config::new(&args).unwrap_or_else(|e| {
		eprintln!("Problem parsing args: {}", e);
		process::exit(1);
	});

	if let Err(e) = fasttyping::run(config) {
		eprintln!("App not working: {}", e);
		process::exit(1);
	}
}


