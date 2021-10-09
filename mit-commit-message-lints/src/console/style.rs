use std::{env, fmt::Display, io};

use clap::App;
use clap_generate::{generate, Generator};
use comfy_table::{
    modifiers::UTF8_ROUND_CORNERS,
    presets::UTF8_FULL,
    Attribute,
    Cell,
    ContentArrangement,
    Table,
};
use miette::{Diagnostic, GraphicalReportHandler, GraphicalTheme, Severity};
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

/// Prints completions to stdout
pub fn print_completions<G: Generator>(app: &mut App) {
    generate::<G, _>(app, app.get_name().to_string(), &mut io::stdout());
}

pub fn miette_install() {
    miette::set_panic_hook();
    if env::var("DEBUG_PRETTY_ERRORS").is_ok() {
        miette::set_hook(Box::new(|_| {
            Box::new(
                miette::MietteHandlerOpts::new()
                    .force_graphical(true)
                    .terminal_links(false)
                    .graphical_theme(GraphicalTheme::unicode_nocolor())
                    .build(),
            )
        }))
        .expect("failed to install debug print handler");
    }
}
