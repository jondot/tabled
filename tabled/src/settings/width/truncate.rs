//! This module contains [`Truncate`] structure, used to decrease width of a [`Table`]s or a cell on a [`Table`] by truncating the width.

use std::{borrow::Cow, iter, marker::PhantomData, ops::Deref};

use papergrid::util::string::{string_width_multiline_tab, string_width_tab};

use crate::{
    grid::{config::GridConfig, grid_projection::GridProjection},
    records::{EmptyRecords, ExactRecords, Records, RecordsMut},
    settings::{
        measurement::Measurement,
        peaker::{Peaker, PriorityNone},
        width::Width,
        CellOption, TableOption,
    },
    tables::table::TableDimension,
};

use super::util::{cut_str, get_table_widths, get_table_widths_with_total, replace_tab};

/// Truncate cut the string to a given width if its length exceeds it.
/// Otherwise keeps the content of a cell untouched.
///
/// The function is color aware if a `color` feature is on.
///
/// Be aware that it doesn't consider padding.
/// So if you want to set a exact width you might need to use [`Padding`] to set it to 0.
///    
/// ## Example
///
/// ```
/// use tabled::{object::Segment, Width, Modify, Table};
///
/// let table = Table::new(&["Hello World!"])
///     .with(Modify::new(Segment::all()).with(Width::truncate(3)));
/// ```
///
/// [`Padding`]: crate::Padding
#[derive(Debug)]
pub struct Truncate<'a, W = usize, P = PriorityNone> {
    width: W,
    suffix: Option<TruncateSuffix<'a>>,
    _priority: PhantomData<P>,
}

#[derive(Debug)]
struct TruncateSuffix<'a> {
    text: Cow<'a, str>,
    limit: SuffixLimit,
    #[cfg(feature = "color")]
    try_color: bool,
}

impl Default for TruncateSuffix<'_> {
    fn default() -> Self {
        Self {
            text: Cow::default(),
            limit: SuffixLimit::Cut,
            #[cfg(feature = "color")]
            try_color: false,
        }
    }
}

/// A suffix limit settings.
#[derive(Debug, Clone, Copy)]
pub enum SuffixLimit {
    /// Cut the suffix.
    Cut,
    /// Don't show the suffix.
    Ignore,
    /// Use a string with n chars instead.
    Replace(char),
}

impl<W> Truncate<'static, W>
where
    W: Measurement<Width>,
{
    /// Creates a [`Truncate`] object
    pub fn new(width: W) -> Truncate<'static, W> {
        Self {
            width,
            suffix: None,
            _priority: PhantomData::default(),
        }
    }
}

impl<'a, W, P> Truncate<'a, W, P> {
    /// Sets a suffix which will be appended to a resultant string.
    ///
    /// The suffix is used in 3 circamstances:
    ///     1. If original string is *bigger* than the suffix.
    ///        We cut more of the original string and append the suffix.
    ///     2. If suffix is bigger than the original string.
    ///        We cut the suffix to fit in the width by default.
    ///        But you can peak the behaviour by using [`Truncate::suffix_limit`]
    pub fn suffix<S: Into<Cow<'a, str>>>(self, suffix: S) -> Truncate<'a, W, P> {
        let mut suff = self.suffix.unwrap_or_default();
        suff.text = suffix.into();

        Truncate {
            width: self.width,
            suffix: Some(suff),
            _priority: PhantomData::default(),
        }
    }

    /// Sets a suffix limit, which is used when the suffix is too big to be used.
    pub fn suffix_limit(self, limit: SuffixLimit) -> Truncate<'a, W, P> {
        let mut suff = self.suffix.unwrap_or_default();
        suff.limit = limit;

        Truncate {
            width: self.width,
            suffix: Some(suff),
            _priority: PhantomData::default(),
        }
    }

    #[cfg(feature = "color")]
    /// Sets a optional logic to try to colorize a suffix.
    pub fn suffix_try_color(self, color: bool) -> Truncate<'a, W, P> {
        let mut suff = self.suffix.unwrap_or_default();
        suff.try_color = color;

        Truncate {
            width: self.width,
            suffix: Some(suff),
            _priority: PhantomData::default(),
        }
    }
}

impl<'a, W, P> Truncate<'a, W, P> {
    /// Priority defines the logic by which a truncate will be applied when is done for the whole table.
    ///
    /// - [`PriorityNone`] which cuts the columns one after another.
    /// - [`PriorityMax`] cuts the biggest columns first.
    /// - [`PriorityMin`] cuts the lowest columns first.
    ///
    /// [`PriorityMax`]: crate::peaker::PriorityMax
    /// [`PriorityMin`]: crate::peaker::PriorityMin
    pub fn priority<PP: Peaker>(self) -> Truncate<'a, W, PP> {
        Truncate {
            width: self.width,
            suffix: self.suffix,
            _priority: PhantomData::default(),
        }
    }
}

impl Truncate<'_, (), ()> {
    pub fn truncate_text(text: &str, width: usize, tab_width: usize) -> Cow<'_, str> {
        let text = replace_tab(text, tab_width);

        match text {
            Cow::Borrowed(text) => truncate_text(text, width, "", false),
            Cow::Owned(text) => match truncate_text(&text, width, "", false) {
                Cow::Borrowed(_) => Cow::Owned(text),
                Cow::Owned(text) => Cow::Owned(text),
            },
        }
    }
}

impl<W, P, R> CellOption<R> for Truncate<'_, W, P>
where
    W: Measurement<Width>,
    R: Records + ExactRecords + RecordsMut<String>,
    for<'a> &'a R: Records,
{
    fn change(&mut self, records: &mut R, cfg: &mut GridConfig, entity: papergrid::config::Entity) {
        let truncate_width = self.width.measure(&*records, cfg);

        let mut width = truncate_width;
        let mut suffix = Cow::Borrowed("");

        if let Some(x) = self.suffix.as_ref() {
            let (s, w) = make_suffix(x, width, cfg.get_tab_width());
            suffix = s;
            width = w;
        };

        let count_rows = records.count_rows();
        let count_columns = records.count_columns();

        let save_suffix_color = need_suffix_color_preservation(&self.suffix);

        for pos in entity.iter(count_rows, count_columns) {
            let text = records.get_cell(pos).as_ref();

            let cell_width = string_width_multiline_tab(text, cfg.get_tab_width());
            if truncate_width >= cell_width {
                continue;
            }

            let text = if width == 0 {
                if truncate_width == 0 {
                    Cow::Borrowed("")
                } else {
                    Cow::Borrowed(suffix.deref())
                }
            } else {
                // todo: Think about it.
                //       We could eliminate this allocation if we would be allowed to cut '\t' with unknown characters.
                //       Currently we don't do that.
                let text = replace_tab(text, cfg.get_tab_width());
                Cow::Owned(truncate_text(&text, width, &suffix, save_suffix_color).into_owned())
            };

            records.set(pos, text.into_owned());
        }
    }
}

fn need_suffix_color_preservation(suffix: &Option<TruncateSuffix<'_>>) -> bool {
    #[cfg(not(feature = "color"))]
    {
        false
    }
    #[cfg(feature = "color")]
    {
        suffix.as_ref().map_or(false, |s| s.try_color)
    }
}

fn make_suffix<'a>(
    suffix: &'a TruncateSuffix<'_>,
    width: usize,
    tab_width: usize,
) -> (Cow<'a, str>, usize) {
    let suffix_length = string_width_tab(&suffix.text, tab_width);
    if width > suffix_length {
        return (Cow::Borrowed(suffix.text.as_ref()), width - suffix_length);
    }

    match suffix.limit {
        SuffixLimit::Ignore => (Cow::Borrowed(""), width),
        SuffixLimit::Cut => {
            let suffix = cut_str(&suffix.text, width);
            (suffix, 0)
        }
        SuffixLimit::Replace(c) => {
            let suffix = Cow::Owned(iter::repeat(c).take(width).collect());
            (suffix, 0)
        }
    }
}

impl<W, P, R> TableOption<R, TableDimension<'static>> for Truncate<'_, W, P>
where
    W: Measurement<Width>,
    P: Peaker,
    R: Records + ExactRecords + RecordsMut<String>,
    for<'a> &'a R: Records,
{
    fn change(
        &mut self,
        records: &mut R,
        cfg: &mut GridConfig,
        dims: &mut TableDimension<'static>,
    ) {
        if records.count_rows() == 0 || records.count_columns() == 0 {
            return;
        }

        let width = self.width.measure(&*records, cfg);
        let (widths, total) = get_table_widths_with_total(&*records, cfg);
        if total <= width {
            return;
        }

        let suffix = self.suffix.as_ref().map(|s| TruncateSuffix {
            text: Cow::Borrowed(&s.text),
            limit: s.limit,
            #[cfg(feature = "color")]
            try_color: s.try_color,
        });

        let widths = truncate_total_width(records, cfg, widths, total, width, P::create(), suffix);

        dims.set_widths(widths);
    }
}

fn truncate_total_width<P: Peaker, R: Records + ExactRecords + RecordsMut<String>>(
    records: &mut R,
    cfg: &mut GridConfig,
    mut widths: Vec<usize>,
    total: usize,
    width: usize,
    priority: P,
    suffix: Option<TruncateSuffix<'_>>,
) -> Vec<usize>
where
    for<'a> &'a R: Records,
{
    let count_rows = records.count_rows();
    let count_columns = records.count_columns();

    let min_widths = get_table_widths(EmptyRecords::new(count_rows, count_columns), cfg);

    decrease_widths(&mut widths, &min_widths, total, width, priority);

    let points = get_decrease_cell_list(cfg, &widths, &min_widths, (count_rows, count_columns));

    let mut truncate = Truncate::new(0);
    truncate.suffix = suffix;
    for ((row, col), width) in points {
        truncate.width = width;
        CellOption::change(&mut truncate, records, cfg, (row, col).into());
    }

    widths
}

fn truncate_text<'a>(
    content: &'a str,
    width: usize,
    suffix: &str,
    _suffix_color_try_keeping: bool,
) -> Cow<'a, str> {
    let content = cut_str(content, width);

    if suffix.is_empty() {
        return content;
    }

    #[cfg(feature = "color")]
    {
        if _suffix_color_try_keeping {
            if let Some(block) = ansi_str::get_blocks(&content).last() {
                if block.has_ansi() {
                    let style = block.style();
                    Cow::Owned(format!(
                        "{}{}{}{}",
                        content,
                        style.start(),
                        suffix,
                        style.end()
                    ))
                } else {
                    let mut content = content.into_owned();
                    content.push_str(suffix);
                    Cow::Owned(content)
                }
            } else {
                let mut content = content.into_owned();
                content.push_str(suffix);
                Cow::Owned(content)
            }
        } else {
            let mut content = content.into_owned();
            content.push_str(suffix);
            Cow::Owned(content)
        }
    }

    #[cfg(not(feature = "color"))]
    {
        let mut content = content.into_owned();
        content.push_str(suffix);
        Cow::Owned(content)
    }
}

fn get_decrease_cell_list(
    cfg: &GridConfig,
    widths: &[usize],
    min_widths: &[usize],
    shape: (usize, usize),
) -> Vec<((usize, usize), usize)> {
    let gp = GridProjection::with_shape(cfg, shape);

    let mut points = Vec::new();
    (0..shape.1).for_each(|col| {
        (0..shape.0)
            .filter(|&row| gp.is_cell_visible((row, col)))
            .for_each(|row| {
                let (width, width_min) = match cfg.get_span_column((row, col)) {
                    Some(span) => {
                        let width = (col..col + span).map(|i| widths[i]).sum::<usize>();
                        let min_width = (col..col + span).map(|i| min_widths[i]).sum::<usize>();
                        let count_borders = count_borders(cfg, col, col + span, shape.1);
                        (width + count_borders, min_width + count_borders)
                    }
                    None => (widths[col], min_widths[col]),
                };

                if width >= width_min {
                    let padding = cfg.get_padding((row, col).into());
                    let width = width.saturating_sub(padding.left.size + padding.right.size);

                    points.push(((row, col), width));
                }
            });
    });

    points
}

fn decrease_widths<F>(
    widths: &mut [usize],
    min_widths: &[usize],
    total_width: usize,
    mut width: usize,
    mut peeaker: F,
) where
    F: Peaker,
{
    let mut empty_list = 0;
    for col in 0..widths.len() {
        if widths[col] == 0 || widths[col] <= min_widths[col] {
            empty_list += 1;
        }
    }

    while width != total_width {
        if empty_list == widths.len() {
            break;
        }

        let col = match peeaker.peak(min_widths, widths) {
            Some(col) => col,
            None => break,
        };

        if widths[col] == 0 || widths[col] <= min_widths[col] {
            continue;
        }

        widths[col] -= 1;

        if widths[col] == 0 || widths[col] <= min_widths[col] {
            empty_list += 1;
        }

        width += 1;
    }
}

fn count_borders(cfg: &GridConfig, start: usize, end: usize, count_columns: usize) -> usize {
    let gp = GridProjection::new(cfg).count_columns(count_columns);

    (start..end).skip(1).filter(|&i| gp.has_vertical(i)).count()
}
