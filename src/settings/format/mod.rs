//! This module contains a list of primitives to help to modify a [`Table`].
//!
//! [`Table`]: crate::Table

mod format_config;
mod format_content;
mod format_positioned;

use papergrid::GridConfig;

use crate::{
    grid::config::Entity,
    records::{ExactRecords, Records, RecordsMut},
};

use crate::CellOption;

pub use format_config::FormatConfig;
pub use format_content::FormatContent;
pub use format_positioned::FormatContentPositioned;

/// A formatting function of particular cells on a [`Table`].
///
/// [`Table`]: crate::Table
#[derive(Debug)]
pub struct Format;

impl Format {
    /// This function creates a new [`Format`] instance, so
    /// it can be used as a grid setting.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{Table, format::Format, object::Rows, Modify};
    ///
    /// let data = vec![
    ///     (0, "Grodno", true),
    ///     (1, "Minsk", true),
    ///     (2, "Hamburg", false),
    ///     (3, "Brest", true),
    /// ];
    ///
    /// let table = Table::new(&data)
    ///                .with(Modify::new(Rows::new(1..)).with(Format::new(|s| format!(": {} :", s))))
    ///                .to_string();
    ///
    /// assert_eq!(table, "+-------+-------------+-----------+\n\
    ///                    | i32   | &str        | bool      |\n\
    ///                    +-------+-------------+-----------+\n\
    ///                    | : 0 : | : Grodno :  | : true :  |\n\
    ///                    +-------+-------------+-----------+\n\
    ///                    | : 1 : | : Minsk :   | : true :  |\n\
    ///                    +-------+-------------+-----------+\n\
    ///                    | : 2 : | : Hamburg : | : false : |\n\
    ///                    +-------+-------------+-----------+\n\
    ///                    | : 3 : | : Brest :   | : true :  |\n\
    ///                    +-------+-------------+-----------+");
    /// ```
    ///
    pub fn content<F>(f: F) -> FormatContent<F>
    where
        F: FnMut(&str) -> String,
    {
        FormatContent(f)
    }

    /// This function creates a new [`FormatWithIndex`], so
    /// it can be used as a grid setting.
    ///
    /// It's different from [`Format::new`] as it also provides a row and column index.
    ///
    /// # Example
    ///
    /// ```
    /// use tabled::{Table, format::Format, object::Rows, Modify};
    ///
    /// let data = vec![
    ///     (0, "Grodno", true),
    ///     (1, "Minsk", true),
    ///     (2, "Hamburg", false),
    ///     (3, "Brest", true),
    /// ];
    ///
    /// let table = Table::new(&data)
    ///                .with(Modify::new(Rows::single(0)).with(Format::with_index(|_, (_, column)| column.to_string())))
    ///                .to_string();
    ///
    /// assert_eq!(table, "+---+---------+-------+\n\
    ///                    | 0 | 1       | 2     |\n\
    ///                    +---+---------+-------+\n\
    ///                    | 0 | Grodno  | true  |\n\
    ///                    +---+---------+-------+\n\
    ///                    | 1 | Minsk   | true  |\n\
    ///                    +---+---------+-------+\n\
    ///                    | 2 | Hamburg | false |\n\
    ///                    +---+---------+-------+\n\
    ///                    | 3 | Brest   | true  |\n\
    ///                    +---+---------+-------+");
    /// ```
    pub fn positioned<F>(f: F) -> FormatContentPositioned<F>
    where
        F: FnMut(&str, (usize, usize)) -> String,
    {
        FormatContentPositioned(f)
    }

    pub fn config<F>(f: F) -> FormatConfig<F>
    where
        F: FnMut(&mut GridConfig),
    {
        FormatConfig(f)
    }
}
