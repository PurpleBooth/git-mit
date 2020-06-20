use tinytemplate::error::Error;

/// Format a string as uppercase
///
/// # Errors
///
/// Errors if the value isn't string or null
pub fn format_upper(value: &serde_json::Value, output: &mut String) -> Result<(), Error> {
    match value {
        serde_json::Value::Null => Ok(()),
        serde_json::Value::String(s) => {
            output.push_str(&s.to_uppercase());
            Ok(())
        }
        _ => Err(Error::GenericError {
            msg: "Expected a printable value but found array or object.".to_string(),
        }),
    }
}
