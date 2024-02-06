use reqwest::{Client, Url};

pub fn is_file_path(input: &str) -> bool {
    std::path::Path::new(input).is_file()
}

pub fn is_url(input: &str) -> bool {
    Url::parse(input).is_ok()
}

pub async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    response.json().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    #[cfg(target_family = "unix")]
    fn test_is_file_path_unix() {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_file_path = temp_file.path().to_string_lossy().into_owned();

        assert_eq!(is_file_path(&temp_file_path), true);
        assert_eq!(is_file_path("/nonexistent/file.txt"), false);
        assert_eq!(is_file_path("http://example.com"), false);
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_is_file_path_windows() {
        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let temp_file_path = temp_file.path().to_string_lossy().into_owned();

        assert_eq!(is_file_path(&temp_file_path), true);
        assert_eq!(is_file_path(r"C:\nonexistent\file.txt"), false);
        assert_eq!(is_file_path("http://example.com"), false);
    }

    #[test]
    fn test_is_url() {
        assert_eq!(is_url("http://example.com"), true);
        assert_eq!(is_url("https://example.com/path"), true);
        assert_eq!(is_url("ftp://example.com"), true);
        assert_eq!(is_url("/path/to/file.txt"), false);
        assert_eq!(is_url("invalid_url"), false);
    }
}
