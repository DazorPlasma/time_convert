#![forbid(unsafe_code)]

mod clocktime;
use colored::Colorize;

fn get_input() -> String {
	let mut input: String = String::new();
	std::io::stdin()
		.read_line(&mut input)
		.expect("Couldn't read input!");
	return input;
}

fn main() {
	let input: String = get_input();

	let time = match clocktime::parse(&input) {
		Ok(val) => val,
		Err(err) => {
			println!("{} {:?}", "ERROR:".red().bold(), err);
			return;
		}
	};

	println!("24h format: {}", time.to_string(clocktime::TimeFormat::TwentyFour));
	println!("12h format: {}", time.to_string(clocktime::TimeFormat::Twelve));
}
