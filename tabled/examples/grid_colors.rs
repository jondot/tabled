//! We can set colors for particular cells by using [`Grid`]
//!
//! The example can be run by this command
//! `cargo run --example grid_colors`
//!
//! [`Grid`]: tabled::grid::Grid

use tabled::{
    settings::{Color, Modify, Style},
    Table, Tabled,
};

#[derive(Tabled)]
struct Bsd {
    distribution: &'static str,
    year_of_first_release: usize,
    is_active: bool,
}

impl Bsd {
    fn new(distribution: &'static str, year_of_first_release: usize, is_active: bool) -> Self {
        Self {
            distribution,
            year_of_first_release,
            is_active,
        }
    }
}

fn main() {
    let data = vec![
        Bsd::new("BSD", 1978, false),
        Bsd::new("SunOS", 1982, false),
        Bsd::new("NetBSD", 1993, true),
        Bsd::new("FreeBSD", 1993, true),
        Bsd::new("OpenBSD", 1995, true),
    ];

    let mut table = Table::new(data);
    table
        .with(Style::psql())
        .with(Modify::new((0, 0)).with(Color::BG_BLUE))
        .with(Modify::new((1, 1)).with(Color::BG_GREEN))
        .with(Modify::new((2, 2)).with(Color::BG_RED));

    println!("{table}");
}
