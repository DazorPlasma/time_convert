#![forbid(unsafe_code)]

const HOURS_RANGE: std::ops::Range<u8> = 0..24;
const MINUTES_RANGE: std::ops::Range<u8> = 0..60;
const SECONDS_RANGE: std::ops::Range<u8> = 0..60;

pub struct ClockTime {
	hours: u8,
	minutes: u8,
	seconds: u8,
}

#[allow(unused)]
pub enum TimeFormat {
	TwentyFour,
	Twelve,
}

#[derive(Debug)]
pub enum ParseClockTimeError {
	BadFormat,
	HoursOverflow,
	InvalidHours,
	InvalidMinutes,
	InvalidSeconds,
}

impl ClockTime {
	pub fn to_string(&self, format: TimeFormat) -> String {
		match format {
			TimeFormat::TwentyFour => {
				format!("{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
			}
			TimeFormat::Twelve => {
				let cycle: &str = if self.hours < 12 { "AM" } else { "PM" };
				let mod_hours = self.hours % 12;
				let display_hours = if mod_hours == 0 { 12 } else { mod_hours };

				format!(
					"{:02}:{:02}:{:02} {cycle}",
					display_hours, self.minutes, self.seconds
				)
			}
		}
	}

	fn validate(&self) -> Result<(), ParseClockTimeError> {
		if HOURS_RANGE.contains(&self.hours) == false {
			return Err(ParseClockTimeError::InvalidHours);
		}
		if MINUTES_RANGE.contains(&self.minutes) == false {
			return Err(ParseClockTimeError::InvalidMinutes);
		}
		if SECONDS_RANGE.contains(&self.seconds) == false {
			return Err(ParseClockTimeError::InvalidSeconds);
		}
		return Ok(());
	}
}

type ClockResult = Result<ClockTime, ParseClockTimeError>;

pub fn new(hours: u8, minutes: u8, seconds: u8) -> ClockResult {
	let new_time = ClockTime {
		hours: hours,
		minutes: minutes,
		seconds: seconds,
	};

	match new_time.validate() {
		Ok(_) => Ok(new_time),
		Err(err) => Err(err),
	}
}

#[derive(Debug)]
enum CycleType {
	AM,
	PM,
}

#[inline]
fn cycle_hours_to_24(hours: u8, cycle: CycleType) -> u8 {
	match cycle {
		CycleType::AM => hours,
		CycleType::PM => {
			if hours != 12 {
				hours + 12
			} else {
				hours
			}
		}
	}
}

#[inline]
fn has_no_numerics(s: &String) -> bool {
	s.chars().any(|c| !c.is_numeric())
}

pub fn parse(input: &str) -> ClockResult {
	let instances: Vec<&str> = input.split(":").collect();

	if instances.len() != 3 {
		return Err(ParseClockTimeError::BadFormat);
	}

	let instances: [&str; 3] = [instances[0].trim(), instances[1].trim(), instances[2].trim()];
	let possible_pm_container: String = instances[2].to_uppercase();
	let current_cycle: Option<CycleType> = match has_no_numerics(&possible_pm_container) {
		true => if possible_pm_container.contains("PM") {
			Some(CycleType::PM)
		} else if possible_pm_container.contains("AM") {
			Some(CycleType::AM)
		} else {
			return Err(ParseClockTimeError::BadFormat);
		},
		false => None
	};

	let hours: u8 = instances[0]
		.parse()
		.or(Err(ParseClockTimeError::BadFormat))?;
	let minutes: u8 = instances[1]
		.parse()
		.or(Err(ParseClockTimeError::BadFormat))?;
	let seconds: u8 = instances[2]
		.split_whitespace()
		.nth(0)
		.ok_or(ParseClockTimeError::BadFormat)?
		.parse()
		.or(Err(ParseClockTimeError::BadFormat))?;

	let hours: u8 = match current_cycle {
		Some(val) => {
			if hours > 12 {
				return Err(ParseClockTimeError::HoursOverflow);
			}
			cycle_hours_to_24(hours, val)
		}
		None => hours,
	};

	return new(hours, minutes, seconds);
}

#[cfg(test)]
mod tests {
	use crate::clocktime::{parse, ParseClockTimeError, TimeFormat};

	#[test]
	fn twelve_bounds1() {
		let invalid_time = "13:00:00 PM";
		match parse(invalid_time) {
			Ok(_) => panic!(),
			Err(err) => assert!(matches!(err, ParseClockTimeError::HoursOverflow)),
		}
	}

	#[test]
	fn twelve_bounds2() {
		let invalid_time = "14:03:01 AM";
		match parse(invalid_time) {
			Ok(_) => panic!(),
			Err(err) => assert!(matches!(err, ParseClockTimeError::HoursOverflow)),
		}
	}

	#[test]
	fn midnight() {
		let variants = vec!["0:0:0", "  0:  000: 0", "0:0:00 AM", "  000:0: 0 "];
		for var in variants {
			assert_eq!(
				"00:00:00",
				parse(var)
					.unwrap()
					.to_string(TimeFormat::TwentyFour)
					.as_str()
			)
		}
	}

	#[test]
	fn basic_conversion1() {
		let string_time = "3:  000 :0";
		assert_eq!(
			"03:00:00 AM",
			parse(string_time).unwrap().to_string(TimeFormat::Twelve)
		);
	}

	#[test]
	fn basic_conversion2() {
		let string_time = "4:  000  :1 PM";
		assert_eq!(
			"16:00:01",
			parse(string_time).unwrap().to_string(TimeFormat::TwentyFour)
		);
	}

	#[test]
	fn back_and_forth() {
		let string_time = "4: 32: 0";
		let new_clocktime = parse(string_time).unwrap();
		assert_eq!("04:32:00", new_clocktime.to_string(TimeFormat::TwentyFour));
		assert_eq!(
			new_clocktime.to_string(TimeFormat::Twelve),
			parse(new_clocktime.to_string(TimeFormat::Twelve).as_str())
				.unwrap()
				.to_string(TimeFormat::Twelve)
		)
	}
}
