use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref REPLACEMENTS: HashMap<char, Vec<&'static str>> = {
        let mut map = HashMap::new();
        
        map.insert('a', vec!["ä", "4", "á"]);
        map.insert('b', vec!["ß", "8"]);
        map.insert('c', vec!["¢", "©"]);
        map.insert('d', vec!["ð", "đ"]);
        map.insert('e', vec!["ë", "3", "é"]);
        map.insert('f', vec!["ƒ"]);
        map.insert('g', vec!["9", "ğ"]);
        map.insert('h', vec!["ĥ", "#"]);
        map.insert('i', vec!["¡", "1", "í"]);
        map.insert('j', vec!["ĵ"]);
        map.insert('k', vec!["κ", "₭"]);
        map.insert('l', vec!["1", "|"]);
        map.insert('m', vec!["м", "₥"]);
        map.insert('n', vec!["ñ", "и"]);
        map.insert('o', vec!["0", "ø", "ö", "ó"]);
        map.insert('p', vec!["ρ", "¶"]);
        map.insert('q', vec!["ʠ"]);
        map.insert('r', vec!["я", "®"]);
        map.insert('s', vec!["$", "š", "§"]);
        map.insert('t', vec!["+", "†", "7"]);
        map.insert('u', vec!["ü", "ú"]);
        map.insert('v', vec!["√"]);
        map.insert('w', vec!["ω", "ŵ"]);
        map.insert('x', vec!["×", "✕"]);
        map.insert('y', vec!["¥", "ÿ"]);
        map.insert('z', vec!["ƶ", "2", "ž"]);

        map
    };
}
