use charabia::Tokenize;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct TokenInfo {
    original: String,
    lemma: String,
    kind: String,
    script: String,
    language: String,
    char_start: usize,
    char_end: usize,
}

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn tokenize(text: &str) -> String {
    let tokens: Vec<TokenInfo> = text
        .reconstruct()
        .map(|(orig, token)| {
            let kind = if token.is_word() {
                "Word".to_string()
            } else if token.is_stopword() {
                "StopWord".to_string()
            } else if token.is_separator() {
                match token.separator_kind() {
                    Some(charabia::SeparatorKind::Hard) => "HardSeparator".to_string(),
                    Some(charabia::SeparatorKind::Soft) => "SoftSeparator".to_string(),
                    None => "Separator".to_string(),
                }
            } else {
                "Unknown".to_string()
            };

            TokenInfo {
                original: orig.to_string(),
                lemma: token.lemma().to_string(),
                kind,
                script: format!("{:?}", token.script),
                language: token
                    .language
                    .map(|l| format!("{:?}", l))
                    .unwrap_or_default(),
                char_start: token.char_start,
                char_end: token.char_end,
            }
        })
        .collect();

    serde_json::to_string(&tokens).unwrap()
}
