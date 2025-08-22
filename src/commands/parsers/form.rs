use std::path::PathBuf;

use crate::util::strip_quotes::strip_matching_quotes;

#[derive(Debug, Clone)]
pub struct FormData {
    pub key: String,
    pub file_name: Option<String>,
    pub file_path: Option<PathBuf>,
    pub str_value: Option<String>,
}

/// Validates a given string and returns a FormData struct if possible.
/// name=value
///     name must not be empty after trim
///     value is taken literally
/// File field:
///   name=@PATH[;filename=NAME]
///     `@PATH` means read from a file; `@-` means read from stdin.
///     Options are semicolon-separated, any order; allowed keys: `filename`.
///     Each option may appear at most once; unknown keys are an error.
///     Quotes around PATH or option values are allowed and are stripped if they match
/// TODO:
///     Validate MIME field
///     Allow STDIN to function
pub fn parse_form(form: &str) -> Result<FormData, String> {
    let (mut name, rhs) = form
        .split_once('=')
        .ok_or_else(|| "Form must be in `key=value` form")?;

    let rhs_stripped = rhs.trim_start().strip_prefix('@');
    name = name.trim();

    if name.is_empty() {
        return Err("Form field name may not be empty".to_string());
    }

    let mut form_data = FormData {
        key: name.to_string(),
        file_name: None,
        file_path: None,
        str_value: None,
    };

    // File field
    if let Some(stripped) = rhs_stripped {
        let inner_contents = stripped.split_once(';');

        if let Some((file_path, keys)) = inner_contents {
            if keys.is_empty() {
                return Err("Dangling ';' found after file path".to_string());
            }

            let path_trimmed = strip_matching_quotes(file_path.trim());
            if path_trimmed.is_empty() {
                return Err("File path after '@' is empty".to_owned());
            }
            form_data.file_path = Some(PathBuf::from(path_trimmed.to_string()));

            for seg in keys.split(';').filter(|s| !s.trim().is_empty()) {
                let (seg_key, seg_value) = seg
                    .split_once('=')
                    .ok_or_else(|| "Options must be in `key=value` form")?;

                let seg_key_formatted = seg_key.trim().to_lowercase();
                let seg_value_trimmed = strip_matching_quotes(seg_value.trim());

                match seg_key_formatted.as_ref() {
                    "filename" => {
                        if seg_value_trimmed.is_empty() {
                            return Err("filename= must have a value".to_string());
                        }

                        if form_data.file_name != None {
                            return Err("Duplicates of `filename=` are not allowed".to_string());
                        }

                        form_data.file_name = Some(seg_value_trimmed.to_string());
                    }
                    unknown => {
                        return Err(format!(
                            "Unkown form option '{unknown}' (allowed: filename)"
                        ));
                    }
                }

                // if path_trimmed == "-" && form_data.file_name.is_none() {
                //     form_data.file_name = Some("stdin".to_string());
                // }
            }
        } else {
            let path_only = strip_matching_quotes(stripped.trim());
            if path_only.is_empty() {
                return Err("File path after '@' is empty".to_owned());
            }
            form_data.file_path = Some(PathBuf::from(path_only.to_string()));

            // if path_only == "-" && form_data.file_name.is_none() {
            //     form_data.file_name = Some("stdin".to_string());
            // }
        }
    } else {
        // Text field
        form_data.str_value = Some(rhs.to_string());
    }

    Ok(form_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_text_form() {
        let form = parse_form("key=value").unwrap();
        assert_eq!(form.key, "key");
        assert_eq!(form.str_value.as_deref(), Some("value"));
        assert!(form.file_name.is_none());
        assert!(form.file_path.is_none());
    }

    #[test]
    fn test_text_value_keeps_everything_after_first_equals() {
        let form = parse_form("k=a=b=c").unwrap();
        assert_eq!(form.key, "k");
        assert_eq!(form.str_value.as_deref(), Some("a=b=c"));
    }

    #[test]
    fn test_text_value_preserves_leading_spaces() {
        // rhs is taken literally (no trimming for text fields)
        let form = parse_form("k=  value").unwrap();
        assert_eq!(form.key, "k");
        assert_eq!(form.str_value.as_deref(), Some("  value"));
    }

    #[test]
    fn test_multipart_form_path_only() {
        let form = parse_form("key=@/tmp/file.bin").unwrap();
        assert_eq!(form.key, "key");
        assert_eq!(form.file_path.unwrap(), PathBuf::from("/tmp/file.bin"));
        assert!(form.file_name.is_none());
        assert!(form.str_value.is_none());
    }

    #[test]
    fn test_multipart_form_with_filename() {
        let form = parse_form("key=@/tmp/file.bin;filename=random.bin").unwrap();
        assert_eq!(form.key, "key");
        assert_eq!(form.file_name.unwrap(), "random.bin");
        assert_eq!(form.file_path.unwrap(), PathBuf::from("/tmp/file.bin"));
        assert!(form.str_value.is_none());
    }
}
