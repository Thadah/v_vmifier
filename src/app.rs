use crate::data::{replacements, wordlist};
use gloo_console::log;
use rand::seq::SliceRandom;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use rand::Rng;

const MAX_SONG_NUMBER: usize = 20;
const MIN_WORD_NUMBER: usize = 1;
const MAX_WORD_NUMBER: usize = 4;

#[function_component(App)]
pub fn app() -> Html {
    let song_titles = use_state(Vec::new); // State for randomized song titles
    let song_number = use_state(|| MAX_SONG_NUMBER); // State for user input
    let min_word_number = use_state(|| MIN_WORD_NUMBER); // State for user input
    let max_word_number = use_state(|| MAX_WORD_NUMBER); // State for user input
    let user_input = use_state(String::new); // State for user input
    let transformed_output = use_state(String::new); // State for transformed output
    let wordlist_state = use_state(Vec::new); // State for the wordlist
    let is_wordlist_loaded = use_state(|| false); // State to track if wordlist is loaded

    // Load the wordlist when the component is mounted
    {
        let wordlist_state = wordlist_state.clone();
        let is_wordlist_loaded = is_wordlist_loaded.clone();
        use_effect_with(
            &(),
            move |_| {
                load_wordlist(wordlist_state.clone(), is_wordlist_loaded.clone());
            },
        );
    }

    let generate_titles = generate_song_titles(&song_number, &min_word_number, &max_word_number, song_titles.clone(), wordlist_state.clone());
    let transform_input = transform_input(user_input.clone(), transformed_output.clone(), wordlist_state.clone());
    
    html! {
        <main>
            <h1>{ "V/Vm-ifier" }</h1>

            // Indicate loading status
            if !*is_wordlist_loaded {
                <p>{ "Loading wordlist..." }</p>
            }

            <div id="input-container">
                <div id="input-tooltips">
                    <p>{ "Number of song titles" }</p>
                    <p>{ "Minimum words per title" }</p>
                    <p>{ "Maximum of words per title" }</p>
                </div>
                <div id="input-fields">
                    <input id="song-number-input" class="user-input" type="number" min="1" max={MAX_SONG_NUMBER.to_string()} oninput={update_number_input(song_number, MAX_SONG_NUMBER)} value={song_number.to_string()} />
                    <input id="song-min-words-input" class="user-input" type="number" placeholder="Min words per title" min="1" max={MAX_WORD_NUMBER.to_string()} oninput={update_min_word_number(min_word_number.clone(), max_word_number.clone(), MAX_WORD_NUMBER)} value={min_word_number.to_string()}/>
                    <input id="song-max-words-input" class="user-input" type="number" placeholder="Max words per title" min="1" max={MAX_WORD_NUMBER.to_string()} oninput={update_max_word_number (min_word_number.clone(), max_word_number.clone(), MAX_WORD_NUMBER)} value={max_word_number.to_string()}/>
                </div>
                <div id="input-generator">
                    <button onclick={generate_titles} disabled={!*is_wordlist_loaded}>
                        { "Generate Randomized Song Titles" }
                    </button>
                </div>
            </div>

            <div id="album">
                <div id="left-text">
                    <p class="text-white">{ "V/Vm Test Records : Made In Northern England." }</p>
                </div>
                <div id="images">
                    <img src="images/belce.png" alt="Belce the Cat" />
                    <img src="images/zuibaku.png" alt="It's just a bomb" />
                    <img src="images/piracy.png" alt="It's (not) a Crime" />
                    <img src="images/blue.png" alt="Blue the Cat" />
                    <img src="images/getaway.png" alt="Black Monday" />
                    <img src="images/aphextwinhl2.png" alt="Come on we all thought the same" />
                </div>
                <div id="main">
                    <div id="main-top">
                        <ul>
                            <li class="text-red text-title">{ "V/Vm" }</li>
                            <li class="text-red text-subtitle">{ "\"HelpAphexTwin4.0\"" }</li>
                        </ul>
                        <ol class="numbered-list">
                            { for (*song_titles).iter().map(|title| html! { <li class="text-green">{ title }</li> }) }
                        </ol>
                    </div>
                    <ul id="main-bottom">
                        <li class="text-yellow">{ "Designed^Coded^Debugged^Deployed" }</li>
                        <li class="text-yellow">{ "by Thadah/aichan in The North Of Basque Country." }</li>
                        <li class="text-red">{ "\"Mi capacidad de atención es menor que la de uno de esos" }</li>
                        <li class="text-red">{ "peces de pecera... Ya sabes, los naranjas.\"" }</li>
                        <li class="text-white text-author">{ "NY4κOTecH - Copyleft Thadah/aichan Test Soft. 2024." }</li>
                    </ul>
                </div>
                <div id="right-text" class="text-white">{ "V/Vm - \"HelpAphexTwin4.O\" HATO4" }</div>
            </div>
            
            <input class="user-input" type="text" placeholder="Enter text to V/Vm-ify" oninput={transform_input.clone()}/>

            <p>{ "Transformed output: " }</p>
            <div id="output">
                <p id="transformed-output" class="text-green">{ (*transformed_output).clone() }</p>
            </div>
        </main>
    }
}

fn load_wordlist(wordlist_state: UseStateHandle<Vec<String>>, is_wordlist_loaded: UseStateHandle<bool>) {
    spawn_local(async move {
        log!("Loading wordlist...");

        match wordlist::get_wordlist().await {
            Ok(words) => {
                wordlist_state.set(words);
                is_wordlist_loaded.set(true);
                log!("Wordlist successfully loaded and set.");
            }
            Err(err) => {
                log!(
                    "Error fetching wordlist: ",
                    &JsValue::from_str(&err.to_string())
                );
                // Optionally set a default wordlist or handle the error
                wordlist_state.set(vec![
                    "default".to_string(),
                    "wordlist".to_string(),
                    "fallback".to_string(),
                ]);
                is_wordlist_loaded.set(true); // Considered loaded even if set to default
            }
        }
    });
}

fn create_randomized_songlist(wordlist: &Vec<String>, song_number: usize, min_word_number: usize, max_word_number: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let randomized_titles: Vec<String> = (0..song_number)
        .map(|_| {
            // Randomly select a word count between min and max (inclusive)
            let word_count = if min_word_number == max_word_number {
                min_word_number
            } else {
                rng.gen_range(min_word_number..=max_word_number)
            };
            // Generate words for the title
            let words: Vec<String> = (0..word_count)
                .map(|_| {
                    wordlist
                        .choose(&mut rng)
                        .unwrap_or(&"default".to_string())
                        .clone()
                })
                .map(|word| transform_text(&word))
                .collect();
            words.join(" ")
        })
        .collect();
    randomized_titles
}

fn transform_text(input: &String) -> String {
    let mut rng = rand::thread_rng();
    let transformed: String = input.chars().map(|c| {
        let mut s = c.to_string();
        // Random chance to transform the character
        if rng.gen_bool(0.4) {
            if let Some(replacements) = replacements::REPLACEMENTS.get(&c) {
                // Pick replacement randomly
                if let Some(replacement) = replacements.choose(&mut rng) {
                    s = replacement.to_string();
                }
            }
        }
        // Random chance to convert to uppercase
        if rng.gen_bool(0.4) {
            s = s.to_uppercase();
        }
        s
    }).collect();
    transformed
}

fn add_random_string(input: String, wordlist: &Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let mut result = input.clone();
    if rng.gen_bool(0.2) {
        let random_word = wordlist.choose(&mut rng).unwrap_or(&"word".to_string()).clone();
        if rng.gen_bool(0.5) {
            result = format!("{} {}", random_word, input);
        } else {
            result = format!("{} {}", input, random_word);
        }
    }
    result
}

fn update_number_input(song_number: UseStateHandle<usize>, max: usize) -> Callback<InputEvent> {
    // Callback to handle input changes
    let song_number_input = {
        let song_number = song_number.clone();
        Callback::from(move |e: InputEvent| {
            // Extract the value from the input event
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<usize>() {
                // Ensure the number is between 1 and 20
                let clamped = value.clamp(1, max);
                song_number.set(clamped);
            }
        })
    };
    song_number_input
}

fn generate_song_titles(song_number: &UseStateHandle<usize>, min_word_number: &UseStateHandle<usize>, max_word_number: &UseStateHandle<usize>, song_titles: UseStateHandle<Vec<String>>, wordlist_state: UseStateHandle<Vec<String>>) -> Callback<MouseEvent> {
    // Function to generate randomized song titles
    let generate_titles = {
        let song_titles = song_titles.clone();
        let wordlist = wordlist_state.clone();
        let song_count = song_number.clone();
        let min_word_count = min_word_number.clone();
        let max_word_count = max_word_number.clone();
        Callback::from(move |_| {
            let wordlist = (*wordlist).clone();
            if wordlist.is_empty() {
                log!("Wordlist is empty. Cannot generate song titles.");
                return;
            }

            let randomized_titles = create_randomized_songlist(&wordlist, *song_count, *min_word_count, *max_word_count);
            song_titles.set(randomized_titles);
        })
    };
    generate_titles
}

fn transform_input(user_input: UseStateHandle<String>, transformed_output: UseStateHandle<String>, wordlist_state: UseStateHandle<Vec<String>>) -> Callback<InputEvent> {
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
    
            // Randomly decide to append or prepend a word with a low chance
            let wordlist = (*wordlist).clone();
            let mut final_string = value.clone();
            
            final_string = add_random_string(final_string, &wordlist);

            let transformed = transform_text(&final_string);
    
            transformed_output.set(transformed);
        })
    };
    transform_input
}

fn update_min_word_number(
    min_word_number: UseStateHandle<usize>,
    max_word_number: UseStateHandle<usize>,
    max_allowed: usize,
) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        if let Ok(value) = input.value().parse::<usize>() {
            let clamped = value.clamp(1, max_allowed);
            min_word_number.set(clamped);
            if clamped > *max_word_number {
                max_word_number.set(clamped);
            }
        }
    })
}

fn update_max_word_number(
    min_word_number: UseStateHandle<usize>,
    max_word_number: UseStateHandle<usize>,
    max_allowed: usize,
) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        if let Ok(value) = input.value().parse::<usize>() {
            let clamped = value.clamp(1, max_allowed);
            max_word_number.set(clamped);
            if clamped < *min_word_number {
                min_word_number.set(clamped);
            }
        }
    })
}