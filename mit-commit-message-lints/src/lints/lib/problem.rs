use crate::console::exit::Code;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Problem {
    error: String,
    tip: String,
    code: Code,
}

impl Problem {
    #[must_use]
    pub fn new(error: String, tip: String, code: Code) -> Problem {
        Problem { error, tip, code }
    }

    #[must_use]
    pub fn code(&self) -> &Code {
        &self.code
    }

    #[must_use]
    pub fn error(&self) -> &str {
        &self.error
    }

    #[must_use]
    pub fn tip(&self) -> &str {
        &self.tip
    }
}

#[cfg(test)]
mod tests {

    use crate::{console::exit::Code, lints::Problem};

    #[test]
    fn test_has_error() {
        let problem = Problem::new("Some error".into(), "".into(), Code::NotConventionalCommit);
        assert_eq!(problem.error(), "Some error");
    }

    #[test]
    fn test_has_has_tip() {
        let problem = Problem::new("".into(), "Some tip".into(), Code::NotConventionalCommit);
        assert_eq!(problem.tip(), "Some tip");
    }

    #[test]
    fn test_has_has_code() {
        let problem = Problem::new("".into(), "".into(), Code::NotConventionalCommit);
        assert_eq!(problem.code(), &Code::NotConventionalCommit);
    }
}
