#![warn(
    clippy::all,
    // clippy::restriction,
    clippy::pedantic,
    clippy::cargo
)]

pub mod activity;
pub mod aggregates;
pub mod filter;
pub mod loader;
pub mod parser;
