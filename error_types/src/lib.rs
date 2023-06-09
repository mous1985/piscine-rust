pub use chrono::{Utc, NaiveDate};

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        let date = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        FormError {
            form_values: (field_name, field_value),
            date,
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name,
            last_name,
            birth,
            birth_location,
            password,
        }
    }

    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        let mut errors = Vec::new();

        if self.first_name.is_empty() {
            let err = FormError::new(String::from("first_name"), self.first_name.clone(), String::from("No user name"));
            return Err(err);
        } else {
            errors.push("Valid first name");
        }

        if self.password.len() < 8 {
            let err = FormError::new(String::from("password"), self.password.clone(), String::from("At least 8 characters"));
            return Err(err);
        }

        let has_alphabetic = self.password.chars().any(|c| c.is_alphabetic());
        let has_numeric = self.password.chars().any(|c| c.is_numeric());
        let has_non_alphanumeric = self.password.chars().any(|c| !c.is_alphanumeric());

        if !has_alphabetic || !has_numeric || !has_non_alphanumeric {
            let err = FormError::new(String::from("password"), self.password.clone(), String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)"));
            return Err(err);
        }

        errors.push("Valid password");

        Ok(errors)
    }
}

pub fn create_date_from_string(date_str: &str) -> Result<NaiveDate, chrono::format::ParseError> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
}
