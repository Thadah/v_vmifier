use gloo::storage::{Storage, LocalStorage};
use std::error::Error;
use gloo_console::log;

const WORDLIST_KEY: &str = "cached_wordlist";

// Embed the contents of 'english.txt' at compile time
const ENGLISH_TXT: &str = include_str!("../../english.txt");

/// Retrieve the wordlist from localStorage or use the embedded 'english.txt'.
pub fn get_wordlist() -> Result<Vec<String>, Box<dyn Error>> {
    log!("get_wordlist start");

    // Attempt to retrieve the wordlist from localStorage
    if let Ok(cached) = LocalStorage::get::<Vec<String>>(WORDLIST_KEY) {
        log!("Wordlist loaded from cache.");
        return Ok(cached);
    }

    log!("Wordlist not found in cache. Using embedded 'english.txt'.");

    // Process the embedded 'english.txt' content
    let words: Vec<String> = ENGLISH_TXT
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
