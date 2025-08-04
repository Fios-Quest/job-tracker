use partially::Partial;
use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Default)]
pub struct IncompletePartialErrors {
    errors: Vec<String>,
}

impl IncompletePartialErrors {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            errors: Vec::with_capacity(capacity),
        }
    }

    pub fn field_error<S: Display>(field: S) -> Self {
        let mut errors = IncompletePartialErrors::with_capacity(1);
        errors.push(format!("Partial Check failed on field `{field}`"));
        errors
    }

    pub fn get_errors(&self) -> &Vec<String> {
        &self.errors
    }

    pub fn push<S: ToString>(&mut self, error: S) {
        self.errors.push(error.to_string())
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}

impl From<IncompletePartialErrors> for Result<(), IncompletePartialErrors> {
    fn from(value: IncompletePartialErrors) -> Self {
        match value.is_empty() {
            true => Ok(()),
            false => Err(value),
        }
    }
}

impl From<Vec<String>> for IncompletePartialErrors {
    fn from(errors: Vec<String>) -> Self {
        Self { errors }
    }
}

impl fmt::Display for IncompletePartialErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Partial was incomplete; ")?;
        let mut iter = self.errors.iter().map(String::as_str).peekable();
        let first_message = iter.next().unwrap_or("but no errors returned");
        write!(f, "{first_message}")?;
        while let Some(message) = iter.next() {
            match iter.peek() {
                None => write!(f, ", and ")?,
                Some(_) => write!(f, ", ")?,
            }
            write!(f, "{message}")?;
        }
        Ok(())
    }
}

impl Error for IncompletePartialErrors {}

pub trait CheckPartialComplete: Partial {
    fn check_complete(&self) -> Result<(), IncompletePartialErrors>;
}

macro_rules! impl_is_partial_complete_optional_name_only {
    ($storable:ty) => {
        impl CheckPartialComplete for $storable {
            fn check_complete(&self) -> Result<(), IncompletePartialErrors> {
                let mut errors = IncompletePartialErrors::with_capacity(1);
                // Name must be present and not empty
                match self.name.as_ref().map(|name| !name.is_empty()) {
                    Some(true) => {}
                    Some(false) => errors.push("`name` is empty"),
                    None => errors.push("`name` is missing"),
                }
                errors.into()
            }
        }
    };
}
pub(crate) use impl_is_partial_complete_optional_name_only;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incomplete_partial_error_to_result_no_errors() {
        let error = IncompletePartialErrors::with_capacity(0);
        let result: Result<_, _> = error.into();
        assert!(result.is_ok())
    }

    #[test]
    fn test_incomplete_partial_error_to_result_single_errors() {
        let mut error = IncompletePartialErrors::with_capacity(1);
        error.push("Single error");
        let result: Result<_, _> = error.into();
        assert!(result.is_err())
    }

    #[test]
    fn test_incomplete_partial_error_no_errors() {
        let error = IncompletePartialErrors { errors: vec![] };
        assert_eq!(
            format!("{error}"),
            "Partial was incomplete; but no errors returned"
        );
    }

    #[test]
    fn test_incomplete_partial_error_single() {
        let error = IncompletePartialErrors {
            errors: vec!["Single error".to_string()],
        };
        assert_eq!(format!("{error}"), "Partial was incomplete; Single error");
    }

    #[test]
    fn test_incomplete_partial_error_double() {
        let error = IncompletePartialErrors {
            errors: vec!["First error".to_string(), "Second error".to_string()],
        };
        assert_eq!(
            format!("{error}"),
            "Partial was incomplete; First error, and Second error"
        );
    }

    #[test]
    fn test_incomplete_partial_error_triple() {
        let error = IncompletePartialErrors {
            errors: vec![
                "First error".to_string(),
                "Second error".to_string(),
                "Third error".to_string(),
            ],
        };
        assert_eq!(
            format!("{error}"),
            "Partial was incomplete; First error, Second error, and Third error"
        );
    }

    #[test]
    fn test_field_error() {
        let error = IncompletePartialErrors::field_error("name");
        assert_eq!(
            format!("{error}"),
            "Partial was incomplete; Partial Check failed on field `name`"
        );
    }

    #[test]
    fn test_get_errors() {
        let errors = vec![
            "First error".to_string(),
            "Second error".to_string(),
            "Third error".to_string(),
        ];
        let error = IncompletePartialErrors {
            errors: errors.clone(),
        };
        assert_eq!(error.get_errors(), &errors);
    }

    #[test]
    fn test_push() {
        let mut error = IncompletePartialErrors::with_capacity(2);
        error.push("First error");
        error.push("Second error");
        assert_eq!(
            error.get_errors(),
            &vec!["First error".to_string(), "Second error".to_string()]
        );
    }

    #[test]
    fn test_from_errors() {
        let errors = vec![
            "First error".to_string(),
            "Second error".to_string(),
            "Third error".to_string(),
        ];
        let error = IncompletePartialErrors::from(errors.clone());
        assert_eq!(error.get_errors(), &errors);
    }
}
