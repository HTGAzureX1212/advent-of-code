#![feature(type_alias_impl_trait)]

#[cfg_attr(feature = "year2015", path = "year2015/mod.rs")]
#[cfg_attr(feature = "year2021", path = "year2021/mod.rs")]
pub mod year;

pub fn main() {
    year::run();
}
