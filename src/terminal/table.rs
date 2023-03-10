use comfy_table::modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS};
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, CellAlignment, Color, Table};

pub fn render_table(headers: Vec<String>, rows: Vec<Vec<String>>) -> String {
    let header_cells: Vec<Cell> = headers
        .into_iter()
        .map(|h| {
            Cell::new(h)
                .add_attribute(Attribute::Bold)
                .fg(Color::DarkMagenta)
                .set_alignment(CellAlignment::Center)
        })
        .collect();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .apply_modifier(UTF8_SOLID_INNER_BORDERS);

    table.set_header(header_cells);
    for row in rows {
        table.add_row(row);
    }

    format!("{table}")
}
