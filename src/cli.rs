use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug)]
pub enum Part {
    One,
    Two,
    Both,
}

#[derive(Debug)]
pub enum Day {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
    All,
}

type ParseError = &'static str;

impl FromStr for Part {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Part::One),
            "2" => Ok(Part::Two),
            "*" => Ok(Part::Both),
            _ => Err("Could not parse Part"),
        }
    }
}

impl FromStr for Day {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Day::One),
            "2" => Ok(Day::Two),
            "3" => Ok(Day::Three),
            "4" => Ok(Day::Four),
            "5" => Ok(Day::Five),
            "6" => Ok(Day::Six),
            "7" => Ok(Day::Seven),
            "8" => Ok(Day::Eight),
            "9" => Ok(Day::Nine),
            "10" => Ok(Day::Ten),
            "11" => Ok(Day::Eleven),
            "12" => Ok(Day::Twelve),
            "13" => Ok(Day::Thirteen),
            "14" => Ok(Day::Fourteen),
            "15" => Ok(Day::Fifteen),
            "16" => Ok(Day::Sixteen),
            "17" => Ok(Day::Seventeen),
            "18" => Ok(Day::Eighteen),
            "19" => Ok(Day::Nineteen),
            "20" => Ok(Day::Twenty),
            "21" => Ok(Day::TwentyOne),
            "22" => Ok(Day::TwentyTwo),
            "23" => Ok(Day::TwentyThree),
            "24" => Ok(Day::TwentyFour),
            "25" => Ok(Day::TwentyFive),
            "*" => Ok(Day::All),
            _ => Err("Could not parse Day"),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "AOC2022", about = "Advent of Code 2022")]
pub struct Opt {
    /// Set day
    #[structopt(short = "d", long = "day", default_value = "1")]
    pub day: Day,
    /// Input file
    #[structopt(short = "p", long = "part", default_value = "1")]
    pub part: Part,
}
