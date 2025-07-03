use estat::EstatClient;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_estat_client_new() {
    let api_key = "test_api_key".to_string();
    let client = EstatClient::new(api_key.clone());
    assert_eq!(client.api_key, api_key);
}

#[test]
fn test_load_api_key_from_file() {
    let temp_file = NamedTempFile::new().unwrap();
    let test_api_key = "test_api_key_from_file";
    fs::write(temp_file.path(), test_api_key).unwrap();

    let contents = fs::read_to_string(temp_file.path()).unwrap();
    let api_key = contents.trim().to_string();

    assert_eq!(api_key, test_api_key);
}

#[test]
fn test_load_api_key_empty_file() {
    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), "").unwrap();

    let contents = fs::read_to_string(temp_file.path()).unwrap();
    let api_key = contents.trim().to_string();

    assert!(api_key.is_empty());
}

#[test]
fn test_load_api_key_whitespace_trimming() {
    let temp_file = NamedTempFile::new().unwrap();
    let test_api_key = "  test_api_key_with_whitespace  \n";
    fs::write(temp_file.path(), test_api_key).unwrap();

    let contents = fs::read_to_string(temp_file.path()).unwrap();
    let api_key = contents.trim().to_string();

    assert_eq!(api_key, "test_api_key_with_whitespace");
}
