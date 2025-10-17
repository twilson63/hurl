use regex::Regex;

pub fn is_valid_url(url: &str) -> bool {
    let url_regex =
        Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").expect("Invalid regex for URL validation");
    url_regex.is_match(url)
}

pub fn extract_status_code(response: &str) -> Option<u16> {
    let status_regex = Regex::new(r"HTTP/\d\.\d (\d{3})").expect("Invalid regex for status code");
    status_regex
        .captures(response)
        .and_then(|caps| caps.get(1))
        .and_then(|m| m.as_str().parse().ok())
}

pub fn format_duration(millis: u128) -> String {
    if millis < 1000 {
        format!("{}ms", millis)
    } else {
        format!("{:.2}s", millis as f64 / 1000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_url() {
        assert!(is_valid_url("https://example.com"));
        assert!(is_valid_url("http://example.com/path"));
        assert!(!is_valid_url("not a url"));
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(500), "500ms");
        assert_eq!(format_duration(1500), "1.50s");
    }
}
