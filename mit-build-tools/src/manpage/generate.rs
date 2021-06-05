use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use clap::App;
use serde::Serialize;
use tinytemplate::TinyTemplate;

use crate::manpage::formatters;

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

/// # Panics
///
/// Will panic if it can't render the template
pub fn generate(app: &App, out_dir: &Path, md_template: &str) {
    let mut tt = TinyTemplate::new();
    let manpage_template = fs::read_to_string(md_template).unwrap();
    tt.set_default_formatter(&formatters::format_escape);
    tt.add_template("man", &manpage_template).unwrap();
    tt.add_formatter("upper", formatters::format_upper);
    tt.add_formatter("escape", formatters::format_escape);
    tt.add_formatter("unescape", tinytemplate::format_unescaped);

    let context = Context::new(app);

    let rendered = tt.render("man", &context).unwrap();
    let mut file =
        File::create(&out_dir.join(format!("{}.man.md", app.get_bin_name().unwrap()))).unwrap();
    file.write_all(rendered.as_ref()).unwrap();
}

impl<'a> Context {
    fn new(app: &App) -> Context {
        Context {
            bin: existing::existing(app, "{bin}"),
            version: existing::existing(app, "{version}"),
            author: existing::existing(app, "{mit}"),
            usage: existing::existing(app, "{usage}"),
            all_args: existing::existing(app, "{all-args}"),
            unified: existing::existing(app, "{unified}"),
            flags: existing::existing(app, "{flags}"),
            options: existing::existing(app, "{options}"),
            positionals: existing::existing(app, "{positionals}"),
            subcommands: existing::existing(app, "{subcommands}"),
            after_help: existing::existing(app, "{after-help}"),
            before_help: existing::existing(app, "{before-help}"),
            about: app.get_about().unwrap().into(),
        }
    }
}

mod existing {
    use clap::App;

    pub(crate) fn existing(app: &App, variable: &'static str) -> String {
        let mut copy = app.clone().help_template(variable);

        let mut version_buffer: Vec<u8> = vec![];
        copy.write_help(&mut version_buffer).unwrap();

        String::from_utf8(version_buffer).unwrap().trim_end().into()
    }
}
