use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FieldError {
    pub field: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AppError {
    Validation {
        errors: Vec<FieldError>,
        message: String,
    },
    Blocked {
        code: String,
        message: String,
    },
    NotFound {
        message: String,
    },
    Message {
        message: String,
    },
}

impl AppError {
    pub fn message(msg: impl Into<String>) -> Self {
        Self::Message {
            message: msg.into(),
        }
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound {
            message: msg.into(),
        }
    }

    pub fn blocked(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self::Blocked {
            code: code.into(),
            message: msg.into(),
        }
    }

    pub fn validation_errors(&self) -> Option<&[FieldError]> {
        match self {
            Self::Validation { errors, .. } => Some(errors),
            _ => None,
        }
    }
}

impl From<String> for AppError {
    fn from(message: String) -> Self {
        Self::Message { message }
    }
}

impl From<&str> for AppError {
    fn from(message: &str) -> Self {
        Self::Message {
            message: message.to_string(),
        }
    }
}

#[derive(Default)]
pub struct ValidationBuilder {
    errors: Vec<FieldError>,
}

impl ValidationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(
        &mut self,
        field: impl Into<String>,
        code: impl Into<String>,
        message: impl Into<String>,
    ) {
        self.errors.push(FieldError {
            field: field.into(),
            code: code.into(),
            message: message.into(),
        });
    }

    pub fn required_len(&mut self, field: &str, raw: &str, min: usize, max: usize) -> String {
        let trimmed = raw.trim().to_string();
        let len = trimmed.chars().count();
        if len == 0 {
            self.push(field, "required", format!("{field} is required."));
        } else if len < min {
            self.push(
                field,
                "too_short",
                format!("{field} must be at least {min} characters."),
            );
        } else if len > max {
            self.push(
                field,
                "too_long",
                format!("{field} must be at most {max} characters."),
            );
        }
        trimmed
    }

    pub fn optional_max_len(
        &mut self,
        field: &str,
        raw: Option<&str>,
        max: usize,
    ) -> Option<String> {
        let trimmed = raw.map(str::trim).unwrap_or("");
        if trimmed.is_empty() {
            return None;
        }
        if trimmed.chars().count() > max {
            self.push(
                field,
                "too_long",
                format!("{field} must be at most {max} characters."),
            );
        }
        Some(trimmed.to_string())
    }

    pub fn finish(self) -> Result<(), AppError> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            let message = self
                .errors
                .iter()
                .map(|e| e.message.as_str())
                .collect::<Vec<_>>()
                .join(" ");
            Err(AppError::Validation {
                errors: self.errors,
                message,
            })
        }
    }
}

pub fn is_valid_email(value: &str) -> bool {
    if value.chars().any(char::is_whitespace) {
        return false;
    }
    let Some((local, domain)) = value.split_once('@') else {
        return false;
    };
    if local.is_empty() || domain.is_empty() || domain.contains('@') {
        return false;
    }
    let labels: Vec<&str> = domain.split('.').collect();
    if labels.len() < 2 {
        return false;
    }
    let Some(tld) = labels.last() else {
        return false;
    };
    if tld.len() < 2 || !tld.bytes().all(|b| b.is_ascii_alphabetic()) {
        return false;
    }
    labels[..labels.len() - 1].iter().all(|label| {
        !label.is_empty()
            && label
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-')
    })
}

pub fn is_valid_http_url(value: &str) -> bool {
    if value.chars().any(char::is_whitespace) {
        return false;
    }
    let rest = if let Some(r) = value.strip_prefix("https://") {
        r
    } else if let Some(r) = value.strip_prefix("http://") {
        r
    } else {
        return false;
    };
    let host = rest.split(['/', '?', '#']).next().unwrap_or("");
    !host.is_empty()
}

pub fn escape_like_pattern(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len() + 4);
    for ch in raw.trim().to_lowercase().chars() {
        if matches!(ch, '%' | '_' | '\\') {
            out.push('\\');
        }
        out.push(ch);
    }
    format!("%{out}%")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_email_and_http_url() {
        assert!(is_valid_email("dr@cabinet-medical.fr"));
        assert!(!is_valid_email("invalid"));
        assert!(!is_valid_email("dr@localhost"));
        assert!(!is_valid_email("dr @cabinet.fr"));

        assert!(is_valid_http_url("https://cabinet.fr"));
        assert!(is_valid_http_url("http://example.com/path"));
        assert!(!is_valid_http_url("ftp://example.com"));
        assert!(!is_valid_http_url("https://"));
    }

    #[test]
    fn escapes_sql_like_wildcards() {
        assert_eq!(escape_like_pattern(" 100%_a\\b "), "%100\\%\\_a\\\\b%");
    }
}
