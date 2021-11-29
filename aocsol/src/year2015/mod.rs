#[cfg_attr(feature = "year2015-day1", path = "day1.rs")]
#[cfg_attr(feature = "year2015-day2", path = "day2.rs")]
#[cfg_attr(feature = "year2015-day3", path = "day3.rs")]
#[cfg_attr(feature = "year2015-day4", path = "day4.rs")]
#[cfg_attr(feature = "year2015-day5", path = "day5.rs")]
pub mod day;

pub fn run() {
    day::solve();
}
