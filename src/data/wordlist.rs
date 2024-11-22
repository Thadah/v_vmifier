use gloo::storage::{Storage, LocalStorage};
use gloo::net::http::Request;
use std::error::Error;

const WORDLIST_KEY: &str = "cached_wordlist";
const WORDLIST_URL: &str = "https://raw.githubusercontent.com/kkrypt0nn/wordlists/main/wordlists/languages/english.txt";

/// Fetch the wordlist from localStorage or the remote URL.
pub async fn get_wordlist() -> Result<Vec<String>, Box<dyn Error>> {
    // Attempt to retrieve the wordlist from localStorage
    if let Ok(cached) = LocalStorage::get::<Vec<String>>(WORDLIST_KEY) {
        return Ok(cached);
    }

    // Fetch the wordlist from the remote URL
    let response = Request::get(WORDLIST_URL)
        .send()
        .await?
        .text()
        .await?;

    // Split the fetched text into lines and collect into a Vec<String>
    let words: Vec<String> = response
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    // Cache the wordlist in localStorage for future use
    LocalStorage::set(WORDLIST_KEY, &words)?;

    Ok(words)
}
