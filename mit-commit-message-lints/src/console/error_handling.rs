//! Setup miette's pretty stack trace handling

use std::env;

use miette::GraphicalTheme;

/// Setup miette's pretty stack trace handling
///
/// We also have a secret env `DEBUG_PRETTY_ERRORS` that disables all fancy
/// output for the specdwon tests
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
