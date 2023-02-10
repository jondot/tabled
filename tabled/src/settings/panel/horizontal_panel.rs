use crate::{
    grid::config::GridConfig,
    records::{ExactRecords, Records, RecordsMut, Resizable},
    settings::TableOption,
};

/// A horizontal/column span from 0 to a count rows.
#[derive(Debug)]
pub struct HorizontalPanel<S> {
    text: S,
    row: usize,
}

impl<S> HorizontalPanel<S> {
    /// Creates a new horizontal panel.
    pub fn new(row: usize, text: S) -> Self {
        Self { row, text }
    }
}

impl<S, R, D> TableOption<R, D> for HorizontalPanel<S>
where
    S: AsRef<str>,
    R: Records + ExactRecords + Resizable + RecordsMut<String>,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, _: &mut D) {
        let count_rows = records.count_rows();
        let count_cols = records.count_columns();

        if self.row > count_rows {
            return;
        }

        move_rows_aside(records, self.row);
        move_row_spans(cfg, self.row);

        let text = self.text.as_ref().to_owned();
        records.set((self.row, 0), text);

        cfg.set_column_span((self.row, 0), count_cols);
    }
}

fn move_rows_aside<R: ExactRecords + Resizable>(records: &mut R, row: usize) {
    records.push_row();

    let count_rows = records.count_rows();

    let shift_count = count_rows - row;
    for i in 1..shift_count {
        let row = count_rows - i;
        records.swap_row(row, row - 1);
    }
}

fn move_row_spans(cfg: &mut GridConfig, target_row: usize) {
    let spans = cfg.iter_span_columns().collect::<Vec<_>>();
    for ((row, col), span) in spans {
        if row >= target_row {
            cfg.set_column_span((row, col), 1);
            cfg.set_column_span((row + 1, col), span);
        }
    }

    let spans = cfg.iter_span_rows().collect::<Vec<_>>();
    for ((row, col), span) in spans {
        if row >= target_row {
            cfg.set_row_span((row, col), 1);
            cfg.set_row_span((row + 1, col), span);
        }
    }
}