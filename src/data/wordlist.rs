use gloo::storage::{LocalStorage, Storage};
use gloo_console::log;
use gloo_net::http::Request;
use std::error::Error;

const WORDLIST_KEY: &str = "cached_wordlist";
const WORDLIST_URL: &str = "https://raw.githubusercontent.com/kkrypt0nn/wordlists/main/wordlists/languages/english.txt";

/// Retrieve the wordlist from localStorage or fetch from the URL.
pub async fn get_wordlist() -> Result<Vec<String>, Box<dyn Error>> {
    log!("get_wordlist start");

    // Attempt to retrieve the wordlist from localStorage
    if let Ok(cached) = LocalStorage::get::<Vec<String>>(WORDLIST_KEY) {
        log!("Wordlist loaded from cache.");
        return Ok(cached);
    }

    log!("Wordlist not found in cache. Fetching from URL.");

    // Fetch the wordlist from the URL
    let response = Request::get(WORDLIST_URL).send().await?;

    if !response.ok() {
        return Err(format!("Failed to fetch wordlist. Status: {}", response.status()).into());
    }

    let text = response.text().await?;

    // Process the wordlist content
    let words: Vec<String> = text
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    log!("Processed wordlist with {} words.", words.len());

    // Cache the wordlist in localStorage for future use
    LocalStorage::set(WORDLIST_KEY, &words)?;
    log!("Wordlist cached in localStorage.");

    log!("get_wordlist end.");
    Ok(words)
}
