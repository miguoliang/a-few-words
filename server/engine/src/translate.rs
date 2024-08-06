use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use super::error::Error;

pub enum Language {
    Chinese,
    English,
}

impl Language {
    fn to_str(&self) -> &str {
        match self {
            Self::Chinese => "zh",
            Self::English => "en",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TranslationResponse {
    data: TranslationData,
}

#[derive(Debug, Serialize, Deserialize)]
struct TranslationData {
    translations: Vec<TranslatedText>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TranslatedText {
    #[serde(rename = "translatedText")]
    translated_text: String,
}

pub async fn translate_text(
    api_key: &str,
    text: &str,
    source_lang: Language,
    target_lang: Language,
) -> Result<String, Error> {
    let client = Client::new();
    let url = format!(
        "https://translation.googleapis.com/language/translate/v2?key={}",
        api_key
    );

    let request_body = json!({
        "q": text,
        "source": source_lang.to_str(),
        "target": target_lang.to_str(),
        "format": "text"
    });

    let response = client
        .post(&url)
        .json(&request_body)
        .send()
        .await?
        .json::<TranslationResponse>()
        .await?;

    Ok(response.data.translations[0].translated_text.clone())
}
