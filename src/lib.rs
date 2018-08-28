use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::time;
use std::io;

#[derive(Debug)]
pub struct Config {
	pub words: Vec<String>,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
		let mut text= String::new();

		// if file not specified, use default text
		if args.len() < 2 {
			text = String::from("On the other hand, we denounce with righteous indignation and dislike men who are so beguiled and demoralized by the charms of pleasure of the moment, so blinded by desire, that they cannot foresee the pain and trouble that are bound to ensue; and equal blame belongs to those who fail in their duty through weakness of will, which is the same as saying through shrinking from toil and pain. These cases are perfectly simple and easy to distinguish. In a free hour, when our power of choice is untrammelled and when nothing prevents our being able to do what we like best, every pleasure is to be welcomed and every pain avoided. But in certain circumstances and owing to the claims of duty or the obligations of business it will frequently occur that pleasures have to be repudiated and annoyances accepted. The wise man therefore always holds in these matters to this principle of selection: he rejects pleasures to secure other greater pleasures, or else he endures pains to avoid worse pains.");
		} else {
			let filename = args[1].clone();
			let mut f = File::open(&filename).expect("file not found");
			f.read_to_string(&mut text).expect("Can't rad file!");
		}

		let words = split_in_words(text);
		Ok(Config {words})
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let mut words = config.words;
	let timer = time::Instant::now();
	println!("START TYPING!!");

	loop {
		println!("{:?}", words);
		let mut guess = String::new();

		io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    guess.pop();
    println!("{}", guess);
    let index = words.iter().position(|ref w| w.to_string() == guess);

    match index {
    	Some(i) => {
    		words.remove(i);
    		Some(i);
    	},
    	None => (),
    }
    print!("{}[2J", 27 as char);
    if words.is_empty() {
    	break
    }
	}

	println!("ALL DONE!!");
	println!("It took you {:?}s", timer.elapsed().as_secs());
  Ok(())
}

fn split_in_words(txt: String) -> Vec<String> {
	let mut words = Vec::new();
	let filter: &[_] = &['.', ',', ';', ':', '?', '!'];

	for w in txt.split_whitespace() {
		words.push(w.trim_matches(filter).to_string());
	}
	words
}
