use cli_table::{Table, Row, Cell};
use cli_table::format::*;
use crate::waypoints::List;

macro_rules! row {
    ($($name:expr, $fmt:expr;)*) => {
        Row::new(vec![
            $(Cell::new($name, $fmt),)*
        ])
    }
}

pub fn print(list: List, group: Option<&str>) {
    match group {
        // print_group
        Some(g) => {
            let mut rows = vec![
                row!{
                    "name", head_format();
                    &g, head_format();
                }
            ];
            for w in &list.0 {
                let mut r = vec![
                    row!{
                        &w.name, Default::default();
                        &w.path, Default::default();
                    }
                ];
                rows.append(&mut r)
            }
            Table::new(rows, table_format())
                .print_stdout()
                .expect("error printing table")
        }
        // print_base
        None => {
            let mut rows = vec![
                row!{
                    "name", head_format();
                    "path", head_format();
                    "group", head_format();
                }
            ];
            for w in &list.0 {
                let g = match &w.group {
                    Some(g) => g,
                    None => ""
                };
                let mut r = vec![
                    row!{
                        &w.name, Default::default();
                        &w.path, Default::default();
                        &g, Default::default();
                    }
                ];
                rows.append(&mut r)
            }
            Table::new(rows, table_format())
                .print_stdout()
                .expect("error printing table")
        }
    }
    // build rows

}

fn head_format() -> CellFormat {
    CellFormat::builder()
        .bold(true)
        .foreground_color(Some(Color::Cyan))
        .justify(Justify::Center)
        .build()
}

fn table_format() -> TableFormat {
    TableFormat::new(border_style(), seperator_style())
}

fn border_style() -> Border {
    Border::builder()
        .top(Some(HorizontalLine::new('┌', '┐','┬','─')))
        .bottom(Some(HorizontalLine::new('└', '┘', '┴', '─')))
        .left(Some(VerticalLine::new('│')))
        .right(Some(VerticalLine::new('│')))
        .build()
}

fn seperator_style() -> Separator {
    Separator::builder()
        .column(Some(VerticalLine::new('│')))
        .title(Some(HorizontalLine::new('├', '┤','┼','─')))
        .build()
}
