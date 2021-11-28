#[cfg_attr(feature = "year2015-day1", path = "day1.rs")]
#[cfg_attr(feature = "year2015-day2", path = "day2.rs")]
pub mod day;

pub fn run() {
    day::solve();
}
