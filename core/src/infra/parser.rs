use crate::domain::request::Request;
use anyhow::{Context, Result};

pub fn parse_http_file(content: &str) -> Result<Request> {
    let mut lines = content.lines();

    // 1. Parse Method and URL
    let first_line = lines.next().context("Empty file")?;
    let mut parts = first_line.split_whitespace();
    let method = parts.next().context("Missing method")?;
    let url = parts.next().context("Missing URL")?;

    let mut request = Request::new(method, url);

    // 2. Parse Headers
    let mut in_body = false;
    let mut body_lines = Vec::new();

    for line in lines {
        if in_body {
            body_lines.push(line);
            continue;
        }

        if line.trim().is_empty() {
            in_body = true;
            continue;
        }

        if let Some((key, value)) = line.split_once(':') {
            request.headers.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    // 3. Set Body
    if !body_lines.is_empty() {
        request.body = Some(body_lines.join("\n"));
    }

    Ok(request)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_get() {
        let content = "GET https://example.com/api";
        let req = parse_http_file(content).unwrap();
        assert_eq!(req.method, "GET");
        assert_eq!(req.url, "https://example.com/api");
        assert!(req.body.is_none());
    }

    #[test]
    fn test_parse_post_with_headers_and_body() {
        let content = r#"POST https://api.com/users
Content-Type: application/json
Authorization: Bearer token

{
    "name": "foo"
}"#;
        let req = parse_http_file(content).unwrap();
        assert_eq!(req.method, "POST");
        assert_eq!(req.headers.get("Content-Type").unwrap(), "application/json");
        assert_eq!(req.body.unwrap(), "{\n    \"name\": \"foo\"\n}");
    }
}
