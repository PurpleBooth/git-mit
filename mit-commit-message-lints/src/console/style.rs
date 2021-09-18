use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS,
    presets::UTF8_FULL,
    Attribute,
    Cell,
    ContentArrangement,
    Table,
};
use console::style;

use crate::{lints::Lints, mit::Authors};

pub fn success(success: &str, tip: &str) {
    println!(
        "{}\n\n{}",
        style(success).green().bold(),
        style(tip).italic()
    );
}

pub fn problem(error: &str, tip: &str) {
    eprintln!("{}\n\n{}", style(error).red().bold(), style(tip).italic());
}

pub fn warning(warning: &str, tip: Option<&str>) {
    eprintln!("{}", style(warning).yellow().bold());

    if let Some(tip) = tip {
        eprintln!("\n{}", style(tip).italic());
    }
}

pub fn to_be_piped(output: &str) {
    println!("{}", output);
}

pub fn lint_table(list: &Lints, enabled: &Lints) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Lint", "Status"]);

    let rows: Table = list.clone().into_iter().fold(table, |mut table, lint| {
        table.add_row(vec![
            lint.name(),
            if enabled.clone().into_iter().any(|x| x == lint) {
                "enabled"
            } else {
                "disabled"
            },
        ]);
        table
    });

    println!("{}", rows);
}

#[must_use]
pub fn author_table(authors: &Authors) -> String {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec!["Initial", "Name", "Email", "Signing Key"]);

    let rows: Table = authors
        .clone()
        .into_iter()
        .fold(table, |mut table, (initial, author)| {
            table.add_row(vec![
                Cell::new(initial),
                Cell::new(author.name()),
                Cell::new(author.email()),
                if let Some(key) = author.signingkey() {
                    Cell::new(key)
                } else {
                    Cell::new("None".to_string()).add_attributes(vec![Attribute::Italic])
                },
            ]);
            table
        });

    format!("{}", rows)
}
