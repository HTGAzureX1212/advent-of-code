#![feature(type_alias_impl_trait)]

#[cfg_attr(feature = "year2015", path = "year2015/mod.rs")]
pub mod year;

pub fn main() {
    year::run();
}
