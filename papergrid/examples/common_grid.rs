use papergrid::{
    config::{AlignmentHorizontal, Borders, Indent, Sides},
    dimension::Estimate,
    grid::common::{CommonConfig, CommonGrid, ExactDimension},
    records::IterRecords,
};

fn main() {
    let cfg = generate_table_config();

    let data = [
        ["Papergrid", "is a library", "for print tables", "!"],
        [
            "Just like this",
            "NOTICE",
            "that multiline is not supported",
            "H\ne\nl\nl\no",
        ],
    ];
    let records = IterRecords::new(data, 4, None);

    let mut dim = ExactDimension::default();
    dim.estimate(records, &cfg);

    let grid = CommonGrid::new(records, &dim, &cfg);

    println!("{grid}");
}

fn generate_table_config() -> CommonConfig {
    const STYLE: Borders<char> = Borders {
        top: Some('-'),
        top_left: Some('+'),
        top_right: Some('+'),
        top_intersection: Some('+'),
        bottom: Some('-'),
        bottom_left: Some('+'),
        bottom_right: Some('+'),
        bottom_intersection: Some('+'),
        horizontal: Some('-'),
        left_intersection: Some('+'),
        right_intersection: Some('+'),
        vertical: Some('|'),
        left: Some('|'),
        right: Some('|'),
        intersection: Some('+'),
    };

    let mut cfg = CommonConfig::default();
    cfg.set_borders(STYLE);
    cfg.set_alignment_horizontal(AlignmentHorizontal::Center);
    cfg.set_padding(Sides::new(
        Indent::spaced(1),
        Indent::spaced(1),
        Indent::spaced(0),
        Indent::spaced(0),
    ));

    cfg
}
