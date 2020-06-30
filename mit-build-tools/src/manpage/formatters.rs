use tinytemplate::error::Error;
use tinytemplate::format_unescaped;

/// Format a string as uppercase
///
/// # Errors
///
/// Errors if the value isn't string or null
pub fn format_upper(value: &serde_json::Value, output: &mut String) -> Result<(), Error> {
    let mut string_value = String::new();
    format_escape(value, &mut string_value)?;

    output.push_str(&string_value.to_uppercase());

    Ok(())
}

/// Escape special markdown sequences
///
/// # Errors
///
/// Errors if the value isn't string or null
pub fn format_escape(value: &serde_json::Value, output: &mut String) -> Result<(), Error> {
    let mut string_value = String::new();
    format_unescaped(value, &mut string_value)?;

    output.push_str(
        &string_value
            .replace("[", "\\[")
            .replace("]", "\\]")
            .replace("(", "\\(")
            .replace(")", "\\)")
            .replace("`", "\\`")
            .replace("<", "\\<")
            .replace(">", "\\>"),
    );

    Ok(())
}
