//! Visual styling for the output

use std::fmt::Display;

use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS,
    presets::UTF8_FULL,
    Attribute,
    Cell,
    ContentArrangement,
    Table,
};
use miette::{Diagnostic, GraphicalReportHandler, Severity};
use mit_lint::Lints;
use thiserror::Error;

use crate::mit::Authors;

/// Print a advice using our error handler tool
///
/// # Panics
///
/// Panics on a format failure. This should be impossible
pub fn success(success: &str, tip: &str) {
    let mut out = String::new();
    GraphicalReportHandler::default()
        .render_report(
            &mut out,
            &Success {
                success: success.to_string(),
                help: Some(tip),
            },
        )
        .unwrap();
    println!("{}", out);
}

#[derive(Error, Debug)]
#[error("{success}")]
struct Success<'a> {
    success: String,
    help: Option<&'a str>,
}

impl Diagnostic for Success<'_> {
    fn severity(&self) -> Option<Severity> {
        Some(Severity::Advice)
    }

    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        self.help
            .map(|x| Box::new(x.to_string()) as Box<dyn Display>)
    }
}

#[derive(Error, Debug)]
#[error("{warning}")]
struct Warning<'a> {
    warning: String,
    help: Option<&'a str>,
}

impl Diagnostic for Warning<'_> {
    fn severity(&self) -> Option<Severity> {
        Some(Severity::Warning)
    }

    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        self.help
            .map(|x| Box::new(x.to_string()) as Box<dyn Display>)
    }
}

/// Print a warning using our error handler tool
///
/// # Panics
///
/// Panics on a format failure. This should be impossible
pub fn warning(warning: &str, tip: Option<&str>) {
    let mut out = String::new();
    GraphicalReportHandler::default()
        .render_report(
            &mut out,
            &Warning {
                warning: warning.to_string(),
                help: tip,
            },
        )
        .unwrap();
    eprintln!("{}", out);
}

/// Print entirely undecorated to stdout
pub fn to_be_piped(output: &str) {
    println!("{}", output);
}

/// Print a table of lints
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

/// Print a table of authors
#[must_use]
pub fn author_table(authors: &Authors<'_>) -> String {
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
                author.signingkey().map_or_else(
                    || Cell::new("None".to_string()).add_attributes(vec![Attribute::Italic]),
                    Cell::new,
                ),
            ]);
            table
        });

    format!("{}", rows)
}
