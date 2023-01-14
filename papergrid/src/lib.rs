#![warn(
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility,
    missing_debug_implementations,
    unreachable_pub,
    missing_docs
)]
#![deny(unused_must_use)]

//! Papergrid is a library for generating text-based tables.
//!
//! It has relatively low level API.
//! If you're interested in a more friendly one take a look at [`tabled`](https://github.com/zhiburt/tabled).
//!
//! # Example
//!
//! ```
//! use papergrid::{
//!     height::HeightEstimator,
//!     records::vec_records::VecRecords,
//!     width::{CfgWidthFunction, WidthEstimator},
//!     Borders, Estimate, Grid, GridConfig,
//! };
//!
//! // Creating a borders structure of a grid.
//! let borders = Borders {
//!     top: Some('-'),
//!     top_left: Some('+'),
//!     top_right: Some('+'),
//!     top_intersection: Some('+'),
//!     bottom: Some('-'),
//!     bottom_left: Some('+'),
//!     bottom_right: Some('+'),
//!     bottom_intersection: Some('+'),
//!     horizontal: Some('-'),
//!     horizontal_left: Some('+'),
//!     horizontal_right: Some('+'),
//!     vertical: Some('|'),
//!     vertical_left: Some('|'),
//!     vertical_right: Some('|'),
//!     intersection: Some('+'),
//! };
//!
//! // Creating a grid config.
//! let mut cfg = GridConfig::default();
//! cfg.set_borders(borders);
//!
//! // Creating an actual data for grid.
//! let records = vec![vec!["Hello", "World"], vec!["Hi", "World"]];
//! let records = VecRecords::new(&records, (2, 2), CfgWidthFunction::from_cfg(&cfg));
//!
//! // Estimate width space for rendering.
//! let mut width = WidthEstimator::default();
//! width.estimate(&records, &cfg);
//!
//! // Estimate height space for rendering.
//! let mut height = HeightEstimator::default();
//! height.estimate(&records, &cfg);
//!
//! // Creating a grid.
//! let grid = Grid::new(&records, &cfg, &width, &height).to_string();
//!
//! assert_eq!(
//!     grid,
//!     concat!(
//!         "+-----+-----+\n",
//!         "|Hello|World|\n",
//!         "+-----+-----+\n",
//!         "|Hi   |World|\n",
//!         "+-----+-----+",
//!     ),
//! );
//! ```

mod color;
mod config;
mod estimation;
mod grid;
mod colors;

pub mod records;
pub mod util;

pub use self::{
    color::{AnsiColor, Color},
    config::{
        AlignmentHorizontal, AlignmentVertical, Border, Borders, Entity, EntityIterator,
        Formatting, GridConfig, HorizontalLine, Indent, Margin, Offset, Padding, Position, Sides,
        VerticalLine,
    },
    config::{MarginColor, PaddingColor},
    estimation::{height, width, Estimate, ExactEstimate},
    grid::Grid,
};
