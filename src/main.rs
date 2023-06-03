#![forbid(unsafe_code)]

mod clocktime;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read input!");

    let time = match clocktime::parse(&input) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("ERROR: {:?}", err);
            return;
        }
    };

    println!(
        "24h format: {}",
        time.to_string(clocktime::TimeFormat::TwentyFour)
    );
    println!(
        "12h format: {}",
        time.to_string(clocktime::TimeFormat::Twelve)
    );
}
