use rustc_serialize::{Encodable, Encoder, Decodable, Decoder};
use std::cmp::Ordering;
use cli::clap::ArgMatches;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Risk {
    Null,
    Low,
    Moderate,
    High,
    Critical,
}

impl Risk {
    // to_u8, matches Risk variant to an assigned u8 field
    fn to_u8(&self) -> u8 {
        match *self {
            Risk::Null => 0,
            Risk::Low => 10,
            Risk::Moderate => 20,
            Risk::High => 30,
            Risk::Critical => 40,
        }
    }

    // from_u8, matches u8 to Risk variant
    fn from_u8(v: u8) -> Self {
        match v {
            10 => Risk::Low,
            20 => Risk::Moderate,
            30 => Risk::High,
            40 => Risk::Critical,
            _ => Risk::Null,
        }
    }

    // from_option_matches, finds risk classification based on explicit cli arguments and alias
    pub fn from_option_matches(o: &ArgMatches) -> Self {
        if o.is_present("low") {
            Risk::Low
        } else if o.is_present("moderate") {
            Risk::Moderate
        } else if o.is_present("high") {
            Risk::High
        } else if o.is_present("critical") {
            Risk::Critical
        } else if let Some(e) = o.value_of("pr") {
            match e {
                "low" | "l" => Risk::Low,
                "moderate" | "m" => Risk::Moderate,
                "high" | "h" => Risk::High,
                "critical" | "crit" | "c" => Risk::Critical,
                _ => Risk::Null,
            }
        } else {
            Risk::Null
        }
    }
}
