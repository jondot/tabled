//! The example can be run by this command
//! `cargo run --example skip`

use tabled::{Table, Tabled};

#[allow(dead_code)]
#[derive(Tabled)]
struct Country {
    name: &'static str,
    capital_city: &'static str,
    #[tabled(skip)]
    surface_area_km2: f32,
    national_currency: &'static str,
    #[tabled(skip)]
    national_currency_short: &'static str,
}

impl Country {
    fn new(
        name: &'static str,
        national_currency: &'static str,
        national_currency_short: &'static str,
        capital_city: &'static str,
        surface_area_km2: f32,
    ) -> Self {
        Self {
            name,
            national_currency,
            national_currency_short,
            capital_city,
            surface_area_km2,
        }
    }
}

fn main() {
    let data = [
        Country::new("Afghanistan", "Afghani", "AFN", "Kabul", 652867.0),
        Country::new("Angola", "Kwanza", "AOA", "Luanda", 1246700.0),
        Country::new("Canada", "Canadian Dollar", "CAD", "Ottawa", 9984670.0),
    ];

    let table = Table::new(data);

    println!("{table}");
}
