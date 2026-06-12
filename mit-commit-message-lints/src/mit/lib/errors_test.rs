use super::errors::*;
use miette::Diagnostic;
use serde::ser;

#[test]
fn serialize_authors_error_has_serialise_diagnostic_code() {
    let ser_err: toml::ser::Error = ser::Error::custom("test error");
    let err = SerializeAuthorsError::from(ser_err);
    let code = err.code().unwrap().to_string();
    assert_eq!(
        code,
        "mit_commit_message_lints::mit::lib::authors::serialise_authors_error",
        "SerializeAuthorsError should have code 'serialise_authors_error', got: {code}"
    );
}

#[test]
fn deserialize_authors_error_has_deserialise_diagnostic_code() {
    let err = DeserializeAuthorsError {
        src: String::new(),
        toml_span: (0usize, 0usize).into(),
        yaml_span: (0usize, 0usize).into(),
        yaml_message: String::new(),
        toml_message: String::new(),
    };
    let code = err.code().unwrap().to_string();
    assert_eq!(
        code,
        "mit_commit_message_lints::mit::lib::authors::deserialise_authors_error",
        "DeserializeAuthorsError should have code 'deserialise_authors_error', got: {code}"
    );
}
