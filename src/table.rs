use crate::waypoints::List;
use cli_table::format::*;
use cli_table::{Cell, Row, Table};

macro_rules! row {
    ($($name:expr, $fmt:expr;)*) => {
        Row::new(vec![
            $(Cell::new($name, $fmt),)*
        ])
    }
}

pub fn print_all(list: List) {
    let mut rows = vec![row! {
        "name", head_format();
        "group", head_format();
        "path", head_format();
    }];
    for w in &list.0 {
        let g = match &w.group {
            Some(g) => g,
            None => "",
        };
        rows.push(row! {
            &w.name, Default::default();
            &g, Default::default();
            &w.path, Default::default();
        })
    }
    Table::new(rows, table_format())
        .print_stdout()
        .expect("error printing table")
}

pub fn print_group(list: List, group: &str) {
    let mut rows = vec![row! {
        "name", head_format();
        group, head_format();
    }];
    for w in &list.0 {
        rows.push(row! {
            &w.name, Default::default();
            &w.path, Default::default();
        })
    }
    Table::new(rows, table_format())
        .print_stdout()
        .expect("error printing table")
}

pub fn print_groupless(list: List) {
    let mut rows = vec![row! {
        "name", head_format();
        "path", head_format();
    }];
    for w in &list.0 {
        let mut r = vec![row! {
            &w.name, Default::default();
            &w.path, Default::default();
        }];
        rows.append(&mut r)
    }
    Table::new(rows, table_format())
        .print_stdout()
        .expect("error printing table")
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
        .top(Some(HorizontalLine::new('┌', '┐', '┬', '─')))
        .bottom(Some(HorizontalLine::new('└', '┘', '┴', '─')))
        .left(Some(VerticalLine::new('│')))
        .right(Some(VerticalLine::new('│')))
        .build()
}

fn seperator_style() -> Separator {
    Separator::builder()
        .column(Some(VerticalLine::new('│')))
        .title(Some(HorizontalLine::new('├', '┤', '┼', '─')))
        .build()
}
