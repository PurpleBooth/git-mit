use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("The details of the author of this commit are stale")]
#[diagnostic(
code(common::mit::lib::authors::try_from_str::unparsable),
help("Can you confirm who's currently coding? It's nice to get and give the right credit. You can fix this by running `git mit` then the initials of whoever is coding for example: `git mit bt` or `git mit bt se`"),
)]
pub struct StaleAuthorError {}

#[derive(Error, Debug, Diagnostic)]
#[error("could not find initial")]
#[diagnostic(help("{help}"))]
pub struct UnknownAuthor {
    help: String,
    #[source_code]
    original_given_initials: String,
    given_initials: Vec<String>,
    missing_initials: Vec<String>,
}

impl UnknownAuthor {
    #[must_use]
    pub fn new(given_initials: &[String], missing_initials: Vec<String>) -> UnknownAuthor {
        let mut tips = vec![
            "To see a summary of your configured authors run",
            "`git mit-config mit generate`",
            "To add a new author run",
            "`git mit-config mit set eg \"Egg Sample\" egg.sample@example.com`",
        ];

        if missing_initials.contains(&"config".to_string()) {
            tips.push("Did you mean `git mit-config`");
        }

        if missing_initials.contains(&"relates-to".to_string()) {
            tips.push("Did you mean `git mit-relates-to`");
        }

        if missing_initials.contains(&"install".to_string()) {
            tips.push("Did you mean `git mit-install`");
        }

        let help: String = tips.join(" ");

        UnknownAuthor {
            given_initials: given_initials.to_owned(),
            original_given_initials: given_initials.join(" "),
            missing_initials,
            help,
        }
    }
}
