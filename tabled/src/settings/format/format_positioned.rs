use crate::{
    grid::config::{Entity, GridConfig},
    records::{ExactRecords, Records, RecordsMut},
    settings::CellOption,
};

/// [`FormatContentPositioned`] is like a [`FormatContent`] an abstraction over a function you can use against a cell.
///
/// It differerent from [`FormatContent`] that it provides a row and column index.
///
/// [`FormatContent`]: crate::settings::format::FormatContent
#[derive(Debug)]
pub struct FormatContentPositioned<F>(pub F)
where
    F: FnMut(&str, (usize, usize)) -> String;

impl<F, R> CellOption<R> for FormatContentPositioned<F>
where
    F: FnMut(&str, (usize, usize)) -> String,
    R: Records + ExactRecords + RecordsMut<String>,
{
    fn change(&mut self, records: &mut R, _: &mut GridConfig, entity: Entity) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        for pos in entity.iter(count_rows, count_cols) {
            let content = records.get_cell(pos).as_ref();
            let content = (self.0)(content, pos);
            records.set(pos, content);
        }
    }
}