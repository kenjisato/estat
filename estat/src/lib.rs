use reqwest::blocking::Client;
use serde_json::Value;
use std::fs;

pub struct EstatClient {
    pub api_key: String,
    base_url: String,
    client: Client,
}

impl EstatClient {

    pub fn new(api_key: String) -> Self {
        Self::with_base_url(api_key, "https://api.e-stat.go.jp")     
    }

    pub fn with_base_url(api_key: String, base_url: &str) -> Self {
        Self {
            api_key,
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    pub fn get_stats_list(&self, stats_code: &str, limit: usize) -> Result<Value, reqwest::Error> {
        let url = format!("{}/rest/3.0/app/json/getStatsList", self.base_url);
        let params = [
            ("appId", self.api_key.as_str()),
            ("statsCode", stats_code),
            ("limit", &limit.to_string()),
            ("lang", "J"),
        ];

        let res = self.client.get(url).query(&params).send()?.json::<Value>()?;
        Ok(res)
    }

    pub fn load_api_key_from_default_location() -> Result<String, Box<dyn std::error::Error>> {
        let mut path = dirs::home_dir().ok_or("ホームディレクトリが見つかりません")?;
        path.push(".estat");

        if !path.exists() {
            return Err(format!("~/.estat が見つかりません（パス: {:?}）", path).into());
        }

        let contents = fs::read_to_string(&path)?;
        let api_key = contents.trim().to_string();

        if api_key.is_empty() {
            return Err("~/.estat に API キーが空です".into());
        }

        Ok(api_key)
    }
}
