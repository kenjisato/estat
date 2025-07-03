use estat::EstatClient;
use mockito::{Matcher, Server};
use serde_json::json;

#[test]
fn test_get_stats_list_success() {
    let mut server = Server::new();

    let mock_response = json!({
        "GET_STATS_LIST": {
            "RESULT": {
                "STATUS": 0,
                "ERROR_MSG": "正常に終了しました。"
            },
            "DATALIST_INF": {
                "NUMBER": 1,
                "RESULT_INF": [{
                    "@id": "0000010101",
                    "STAT_NAME": {"@code": "00200521", "$": "国勢調査"},
                    "GOV_ORG": {"@code": "00200", "$": "総務省"},
                    "STATISTICS_NAME": "平成27年国勢調査",
                    "TITLE": {"@no": "001", "$": "人口等基本集計"},
                    "CYCLE": "-",
                    "SURVEY_DATE": "201510",
                    "OPEN_DATE": "20160226",
                    "SMALL_AREA": 0,
                    "MAIN_CATEGORY": {"@code": "02", "$": "人口・世帯"}
                }]
            }
        }
    });

    let mock = server
        .mock("GET", "/rest/3.0/app/json/getStatsList")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("appId".into(), "test_api_key".into()),
            Matcher::UrlEncoded("statsCode".into(), "00200521".into()),
            Matcher::UrlEncoded("limit".into(), "10".into()),
            Matcher::UrlEncoded("lang".into(), "J".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response.to_string())
        .create();

    let client = EstatClient::with_base_url("test_api_key".to_string(), &server.url());
    let result = client.get_stats_list("00200521", 10);

    mock.assert();
    assert!(result.is_ok());
}

#[test]
fn test_get_stats_list_api_error() {
    let mut server = Server::new();

    let error_response = json!({
        "GET_STATS_LIST": {
            "RESULT": {
                "STATUS": 100,
                "ERROR_MSG": "アプリケーションIDが指定されていません。"
            }
        }
    });

    let mock = server
        .mock("GET", "/rest/3.0/app/json/getStatsList")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("appId".into(), "test_api_key".into()),
            Matcher::UrlEncoded("statsCode".into(), "00200521".into()),
            Matcher::UrlEncoded("limit".into(), "10".into()),
            Matcher::UrlEncoded("lang".into(), "J".into()),
        ]))
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(error_response.to_string())
        .create();

    let client = EstatClient::with_base_url("test_api_key".to_string(), &server.url());
    let result = client.get_stats_list("00200521", 10);

    mock.assert();
    assert!(result.is_ok());
}

#[test]
fn test_get_stats_list_network_error() {
    let mut server = Server::new();

    let mock = server 
        .mock("GET", "/rest/3.0/app/json/getStatsList")
        .match_query(Matcher::AllOf(vec![
            Matcher::UrlEncoded("appId".into(), "test_api_key".into()),
            Matcher::UrlEncoded("statsCode".into(), "00200521".into()),
            Matcher::UrlEncoded("limit".into(), "10".into()),
            Matcher::UrlEncoded("lang".into(), "J".into()),
        ]))
        .with_status(500)
        .create();

    let client = EstatClient::with_base_url("test_api_key".to_string(), &server.url());
    let result = client.get_stats_list("00200521", 10);

    mock.assert();
    assert!(result.is_err());
}