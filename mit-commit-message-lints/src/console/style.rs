use comfy_table::Table;
use console::style;

use crate::lints::Lints;

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
