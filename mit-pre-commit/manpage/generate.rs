use serde::Serialize;

#[derive(Serialize)]
struct Context {
    bin: String,
    version: String,
    about: String,
    usage: String,
    author: String,
    all_args: String,
    unified: String,
    flags: String,
    options: String,
    positionals: String,
    subcommands: String,
    after_help: String,
    before_help: String,
}

use crate::manpage::formatters;
use clap::App;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tinytemplate::{format_unescaped, TinyTemplate};

pub fn generate(app: &App, out_dir: &PathBuf) {
    let mut tt = TinyTemplate::new();
    let manpage_template = fs::read_to_string("docs/manpage.template.md").unwrap();
    tt.set_default_formatter(&format_unescaped);
    tt.add_template("man", &manpage_template).unwrap();
    tt.add_formatter("upper", formatters::format_upper);

    let context = Context::new(&app);

    let rendered = tt.render("man", &context).unwrap();
    let mut file =
        File::create(&out_dir.join(format!("{}.man.md", env!("CARGO_PKG_NAME")))).unwrap();
    file.write_all(rendered.as_ref()).unwrap();
}

impl<'a> Context {
    fn new(app: &App) -> Context {
        Context {
            bin: existing::existing(&app, "{bin}"),
            version: existing::existing(&app, "{version}"),
            author: existing::existing(&app, "{author}"),
            usage: existing::existing(&app, "{usage}"),
            all_args: existing::existing(&app, "{all-args}"),
            unified: existing::existing(&app, "{unified}"),
            flags: existing::existing(&app, "{flags}"),
            options: existing::existing(&app, "{options}"),
            positionals: existing::existing(&app, "{positionals}"),
            subcommands: existing::existing(&app, "{subcommands}"),
            after_help: existing::existing(&app, "{after-help}"),
            before_help: existing::existing(&app, "{before-help}"),
            about: app.get_about().unwrap().into(),
        }
    }
}

mod existing {
    use clap::App;

    pub(crate) fn existing(app: &App, variable: &str) -> String {
        let mut copy = app.clone().help_template(variable);

        let mut version_buffer: Vec<u8> = vec![];
        copy.write_help(&mut version_buffer).unwrap();

        String::from_utf8(version_buffer.to_vec())
            .unwrap()
            .trim_end()
            .into()
    }
}
