//! This module contains primitivies to create a spread row.
//! Ultimately it is a cell with a span set to a number of columns on the [Table].
//!
//! You can use a [Span] to set a custom span.
//!
//! # Example
//!
//! ```
//! use tabled::{object::Cell, Panel, Modify, TableIteratorExt, Span};
//!
//! let data = [[1, 2, 3], [4, 5, 6]];
//!
//! let table = data.table()
//!     .with(Panel("Tabled Releases", 0))
//!     .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
//!     .to_string();
//!
//! println!("{}", table);
//!
//! assert_eq!(
//!     table,
//!     concat!(
//!         "+-----+-----+-----+\n",
//!         "| Tabled Releases |\n",
//!         "+-----+-----+-----+\n",
//!         "|  0  |  1  |  2  |\n",
//!         "+-----+-----+-----+\n",
//!         "|     1     |  3  |\n",
//!         "+-----+-----+-----+\n",
//!         "|  4  |  5  |  6  |\n",
//!         "+-----+-----+-----+",
//!     )
//! )
//! ```
//!
//! [Table]: crate::Table
//! [Span]: crate::Span

use papergrid::{Entity, Grid, Settings};

use crate::TableOption;

/// Panel allows to add a Row which has 1 continues Cell to a [Table].
///
/// See `examples/panel.rs`.
///
/// [Table]: crate::Table
#[derive(Debug)]
pub struct Panel<S: AsRef<str>>(pub S, pub usize);

impl<S: AsRef<str>> TableOption for Panel<S> {
    fn change(&mut self, grid: &mut Grid) {
        let mut new = grid.resize(grid.count_rows() + 1, grid.count_columns());
        for row in 0..grid.count_rows() {
            for column in 0..grid.count_columns() {
                let cell_settings = grid.get_settings(row, column);
                if row >= self.1 {
                    new.set(Entity::Cell(row + 1, column), cell_settings);
                } else {
                    new.set(Entity::Cell(row, column), cell_settings);
                }

                #[cfg(feature = "color")]
                {
                    let colored = grid.get_colored_border((row, column));
                    if row >= self.1 {
                        new.set_colored_border(Entity::Cell(row + 1, column), colored);
                    } else {
                        new.set_colored_border(Entity::Cell(row, column), colored);
                    }
                }
            }
        }

        new.set(
            Entity::Cell(self.1, 0),
            Settings::new()
                .text(self.0.as_ref().to_owned())
                .span(new.count_columns()),
        );

        *grid = new;
    }
}

/// Header inserts a [Panel] at the top.
/// See [Panel].
#[derive(Debug)]
pub struct Header<S: AsRef<str>>(pub S);

impl<S: AsRef<str>> TableOption for Header<S> {
    fn change(&mut self, grid: &mut Grid) {
        Panel(self.0.as_ref(), 0).change(grid)
    }
}

/// Footer renders a [Panel] at the bottom.
/// See [Panel].
#[derive(Debug)]
pub struct Footer<S: AsRef<str>>(pub S);

impl<S: AsRef<str>> TableOption for Footer<S> {
    fn change(&mut self, grid: &mut Grid) {
        Panel(self.0.as_ref(), grid.count_rows()).change(grid)
    }
}