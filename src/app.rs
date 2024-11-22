// src/app.rs

use rand::seq::SliceRandom;
use yew::prelude::*;
use crate::data::{replacements, wordlist};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

#[function_component(App)]
pub fn app() -> Html {
    let song_titles = use_state(|| Vec::new()); // State for randomized song titles
    let user_input = use_state(|| "".to_string()); // State for user input
    let transformed_output = use_state(|| "".to_string()); // State for transformed output
    let wordlist_state = use_state(|| Vec::new()); // State for the wordlist

    {
        let wordlist_state = wordlist_state.clone();
        use_effect(move || {
            // Spawn a local task to fetch the wordlist asynchronously
            spawn_local(async move {
                match wordlist::get_wordlist().await {
                    Ok(words) => wordlist_state.set(words),
                    Err(err) => {
                        log::error!("Error fetching wordlist: {:?}", err);
                        // Optionally set a default wordlist or handle the error
                        wordlist_state.set(vec![
                            "default".to_string(),
                            "wordlist".to_string(),
                            "fallback".to_string(),
                        ]);
                    }
                }
            });
            // No cleanup necessary
            || ()
        });
    }

    // Function to generate randomized song titles
    let generate_titles = {
        let song_titles = song_titles.clone();
        let wordlist = wordlist_state.clone();
        Callback::from(move |_| {
            let wordlist = (*wordlist).clone();
            if wordlist.is_empty() {
                log::warn!("Wordlist is empty. Cannot generate song titles.");
                return;
            }

            let mut rng = rand::thread_rng();
            let randomized_titles: Vec<String> = (0..5) // Generate 5 titles
                .map(|_| {
                    let word1 = wordlist.choose(&mut rng).unwrap_or(&"default".to_string()).clone();
                    let word2 = wordlist.choose(&mut rng).unwrap_or(&"title".to_string()).clone();
                    format!("{} {}", word1, word2)
                })
                .collect();
            song_titles.set(randomized_titles);
        })
    };

    // Function to transform input into V/Vm style
    let transform_input = {
        let user_input = user_input.clone();
        let transformed_output = transformed_output.clone();
        let wordlist = wordlist_state.clone();

        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>();
            let value = input.value();

            user_input.set(value.clone());

            let mut rng = rand::thread_rng();
            let transformed: String = value
                .chars()
                .map(|c| {
                    replacements::REPLACEMENTS
                        .get(&c)
                        .and_then(|vec| vec.choose(&mut rng))
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| c.to_string())
                })
                .collect();

            // Append random word from the wordlist
            let wordlist = (*wordlist).clone();
            if wordlist.is_empty() {
                log::warn!("Wordlist is empty. Cannot append random suffix.");
                transformed_output.set(transformed.clone());
                return;
            }

            let random_suffix = wordlist.choose(&mut rng).unwrap_or(&"suffix".to_string()).clone();
            let final_transformed = format!("{} {}", transformed, random_suffix);

            transformed_output.set(final_transformed);
        })
    };

    html! {
        <main>
            <h1>{ "V/Vm-ifier with Dynamic Wordlist" }</h1>
            <button onclick={generate_titles}>{ "Generate Randomized Song Titles" }</button>
            <ul>
                { for (*song_titles).iter().map(|title| html! { <li>{ title }</li> }) }
            </ul>
            <input
                type="text"
                placeholder="Enter text to V/Vm-ify"
                oninput={transform_input.clone()}
            />
            <p>{ "Transformed output: " }{ (*transformed_output).clone() }</p>
        </main>
    }
}
