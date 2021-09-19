use std::{error::Error, process};

use console::style;
use mit_commit::CommitMessage;
use mit_lint::{Code, Problem};

pub fn unparsable_author(parse_err: &dyn Error) {
    super::style::problem(
        "Unable to parse the author config",
        &format!(
            "You can fix this by correcting the file so it's parsable\n\nYou can see a parsable \
             example by running:\ngit mit-config mit example\n\nHere's the technical details, \
             that might help you track down the source of the problem\n\n{}",
            parse_err
        ),
    );
    std::process::exit(Code::UnparsableAuthorFile as i32);
}

pub fn initial_not_matched_to_author(initials_without_authors: &[&str]) {
    let mut tips = vec![
        "To see a summary of your configured authors run",
        "git mit-config mit generate",
        "To add a new author run",
        "git mit-config mit set eg \"Egg Sample\" egg.sample@example.com",
    ];

    if initials_without_authors.contains(&"config") {
        tips.push("Did you mean \"git mit-config\"");
    }

    if initials_without_authors.contains(&"relates-to") {
        tips.push("Did you mean \"git mit-relates-to\"");
    }

    if initials_without_authors.contains(&"install") {
        tips.push("Did you mean \"git mit-install\"");
    }

    super::style::problem(
        &format!(
            "Could not find the initials {}.",
            initials_without_authors.join(", ")
        ),
        &tips.join("\n\n"),
    );
    std::process::exit(Code::InitialNotMatchedToAuthor as i32);
}

pub fn stale_author() {
    crate::console::style::problem(
        "The details of the author of this commit are stale",
        "Can you confirm who's currently coding?\n\nIt's nice to get and give the right \
         credit.\n\nYou can fix this by running `git mit` then the initials of whoever is coding \
         for example:\ngit mit bt\ngit mit bt se\n",
    );

    process::exit(Code::StaleAuthor as i32);
}

fn format_lint_problems(
    original_message: &CommitMessage,
    lint_problems: Vec<Problem>,
) -> Option<(String, Code)> {
    let (_, message_and_code) = lint_problems.into_iter().fold(
        (original_message, None),
        |(commit_message, output), problem| {
            (
                commit_message,
                match output {
                    Some((existing_output, _)) => Some((
                        {
                            let error = style(problem.error()).red().bold();
                            let tip = style(problem.tip()).italic();

                            format!("{}\n\n{}\n\n{}", existing_output, error, tip)
                        },
                        *(problem.code()),
                    )),
                    None => Some((
                        {
                            let error = style(problem.error()).red().bold();
                            let tip = style(problem.tip()).italic();

                            format!(
                                "{}\n\n---\n\n{}\n\n{}",
                                String::from(commit_message.clone()),
                                error,
                                tip
                            )
                        },
                        *(problem.code()),
                    )),
                },
            )
        },
    );
    message_and_code
}

pub fn lint_problem(commit_message: &CommitMessage, lint_problems: Vec<Problem>, clipboard: bool) {
    let output = format_lint_problems(commit_message, lint_problems);

    if let Some((message, exit_code)) = output {
        display_lint_err_and_exit(&message, exit_code, clipboard);
    }
}

fn display_lint_err_and_exit(commit_message: &str, exit_code: Code, clipboard: bool) {
    eprintln!("{}", commit_message);

    if clipboard {
        eprintln!(
            "\n{}",
            style("Your previous commit message has been copied to the clipboard")
                .bold()
                .blue()
        );
    }

    std::process::exit(exit_code as i32);
}
