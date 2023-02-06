//! This module contains an [`Alignment`] setting for cells on the [`Table`].
//!
//! An alignment strategy can be set by [`AlignmentStrategy`].
//!
//! # Example
//!
//! ```
//! use tabled::{
//!     formatting::AlignmentStrategy,
//!     object::Segment,
//!     Alignment, Modify, Style, Table,
//! };
//!
//! let data = [
//!     ["1", "2", "3"],
//!     ["Some\nMulti\nLine\nText", "and a line", "here"],
//!     ["4", "5", "6"],
//! ];
//!
//! let mut table = Table::new(&data);
//! table.with(Style::modern())
//!      .with(
//!         Modify::new(Segment::all())
//!             .with(Alignment::right())
//!             .with(Alignment::center())
//!             .with(AlignmentStrategy::PerCell)
//!     );
//!
//! assert_eq!(
//!     table.to_string(),
//!     concat!(
//!         "┌───────┬────────────┬──────┐\n",
//!         "│   0   │     1      │  2   │\n",
//!         "├───────┼────────────┼──────┤\n",
//!         "│   1   │     2      │  3   │\n",
//!         "├───────┼────────────┼──────┤\n",
//!         "│ Some  │ and a line │ here │\n",
//!         "│ Multi │            │      │\n",
//!         "│ Line  │            │      │\n",
//!         "│ Text  │            │      │\n",
//!         "├───────┼────────────┼──────┤\n",
//!         "│   4   │     5      │  6   │\n",
//!         "└───────┴────────────┴──────┘",
//!     ),
//! )
//! ```
//!
//! [`Table`]: crate::Table
//! [`AlignmentStrategy`]: crate::formatting::AlignmentStrategy

use crate::{
    grid::config::{AlignmentHorizontal, AlignmentVertical, Entity, GridConfig},
    CellOption, TableOption,
};

use AlignmentInner::*;

/// Alignment represent a horizontal and vertical alignment setting for any cell on a [`Table`].
///
/// ```rust,no_run
/// # use tabled::{Alignment, Modify, object::Rows, Table};
/// # let data: Vec<&'static str> = Vec::new();
/// let mut table = Table::new(&data);
/// table.with(Modify::new(Rows::single(0)).with(Alignment::center()));
/// ```
///
/// [`Table`]: crate::Table
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Alignment {
    inner: AlignmentInner,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum AlignmentInner {
    /// A horizontal alignment.
    Horizontal(AlignmentHorizontal),
    /// A vertical alignment.
    Vertical(AlignmentVertical),
}

impl Alignment {
    /// Left constructs a horizontal alignment to [`AlignmentHorizontal::Left`]
    pub fn left() -> Self {
        Self::horizontal(AlignmentHorizontal::Left)
    }

    /// Right constructs a horizontal alignment to [`AlignmentHorizontal::Right`]
    ///
    /// ## Notice
    ///
    /// When you use [`MinWidth`] the alignment might not work as you expected.
    /// You could try to apply [`TrimStrategy`] which may help.
    ///
    /// [`MinWidth`]: crate::width::MinWidth
    /// [`TrimStrategy`]: crate::formatting::TrimStrategy
    pub fn right() -> Self {
        Self::horizontal(AlignmentHorizontal::Right)
    }

    /// Center constructs a horizontal alignment to [`AlignmentHorizontal::Center`]
    ///
    /// ## Notice
    ///
    /// When you use [`MinWidth`] the alignment might not work as you expected.
    /// You could try to apply [`TrimStrategy`] which may help.
    ///
    /// [`MinWidth`]: crate::width::MinWidth
    /// [`TrimStrategy`]: crate::formatting::TrimStrategy
    pub fn center() -> Self {
        Self::horizontal(AlignmentHorizontal::Center)
    }

    /// Top constructs a vertical alignment to [`AlignmentVertical::Top`]
    pub fn top() -> Self {
        Self::vertical(AlignmentVertical::Top)
    }

    /// Bottom constructs a vertical alignment to [`AlignmentVertical::Bottom`]
    pub fn bottom() -> Self {
        Self::vertical(AlignmentVertical::Bottom)
    }

    /// `Center_vertical` constructs a vertical alignment to [`AlignmentVertical::Center`]
    pub fn center_vertical() -> Self {
        Self::vertical(AlignmentVertical::Center)
    }

    /// Returns an alignment with the given horizontal alignment.
    fn horizontal(alignment: AlignmentHorizontal) -> Self {
        Self::new(Horizontal(alignment))
    }

    /// Returns an alignment with the given vertical alignment.
    fn vertical(alignment: AlignmentVertical) -> Self {
        Self::new(Vertical(alignment))
    }

    fn new(inner: AlignmentInner) -> Self {
        Self { inner }
    }
}

impl<R> CellOption<R> for Alignment {
    fn change(&mut self, records: &mut R, cfg: &mut papergrid::GridConfig, entity: Entity) {
        match self.inner {
            Horizontal(a) => cfg.set_alignment_horizontal(entity, a),
            Vertical(a) => cfg.set_alignment_vertical(entity, a),
        }
    }
}

impl<R, D> TableOption<R, D> for Alignment {
    fn change(&mut self, records: &mut R, cfg: &mut papergrid::GridConfig, dimension: &mut D) {
        match self.inner {
            Horizontal(a) => cfg.set_alignment_horizontal(Entity::Global, a),
            Vertical(a) => cfg.set_alignment_vertical(Entity::Global, a),
        }
    }
}