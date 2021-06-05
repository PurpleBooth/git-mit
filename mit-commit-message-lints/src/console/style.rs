use crate::lints::Lints;
use comfy_table::Table;
use console::style;

pub fn problem(error: &str, tip: &str) {
    eprintln!("{}\n\n{}", style(error).red().bold(), style(tip).italic());
}

pub fn warning(warning: &str) {
    eprintln!("{}", style(warning).yellow().bold());
}

pub fn to_be_piped(output: &str) {
    println!("{}", output);
}

pub fn lint_table(list: &Lints, enabled: &Lints) {
    let mut table = Table::new();
    table.set_header(vec!["Lint", "Status"]);

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
