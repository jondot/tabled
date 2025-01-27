//! The example can be run by this command
//! `cargo run --example compact_table`

use tabled::{settings::Style, tables::compact::CompactTable};

fn main() {
    let data = [
        ["Debian", "", "true"],
        ["Arch", "", "true"],
        ["Manjaro", "Arch", "true"],
    ];

    let table = CompactTable::new(data)
        .columns(3)
        .width([7, 5, 5])
        .with(Style::markdown());

    #[cfg(feature = "std")]
    println!("{}", table.to_string());
}
