//! Validation utilities for the Venice AI API SDK

/// Validate that a string is not empty
pub fn validate_non_empty_string(value: &str, field_name: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("{} cannot be empty", field_name));
    }
    Ok(())
}

/// Validate that a number is within a range
pub fn validate_number_range<T: PartialOrd + std::fmt::Display>(
    value: T,
    min: T,
    max: T,
    field_name: &str,
) -> Result<(), String> {
    if value < min || value > max {
        return Err(format!(
            "{} must be between {} and {}, got {}",
            field_name, min, max, value
        ));
    }
    Ok(())
}

/// Validate that a vector is not empty
pub fn validate_non_empty_vec<T>(value: &[T], field_name: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err(format!("{} cannot be empty", field_name));
    }
    Ok(())
}

/// Validate that a string matches a regex pattern
#[cfg(feature = "regex")]
pub fn validate_regex_match(value: &str, pattern: &str, field_name: &str) -> Result<(), String> {
    use regex::Regex;
    
    let regex = Regex::new(pattern).map_err(|e| {
        format!("Invalid regex pattern for {}: {}", field_name, e)
    })?;
    
    if !regex.is_match(value) {
        return Err(format!(
            "{} must match pattern {}, got {}",
            field_name, pattern, value
        ));
    }
    
    Ok(())
}