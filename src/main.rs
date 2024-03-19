use std::collections::HashMap;

fn find_pairs(data: String) -> HashMap<String, usize> {
    // find most common pair
    let token = data.into_bytes();
    let mut pairs: HashMap<String, usize> = HashMap::new();
    for (i, b) in token.iter().enumerate() {
        if i == token.len() - 1 {
            break;
        }
        let key = format!("{}, {}", *b, token[i + 1]);
        let value = pairs.entry(key).or_insert(0);
        *value += 1;
    }
    pairs
}

fn main() {
    // let text = "Ｕｎｉｃｏｄｅ! 🅤🅝🅘🅒🅞🅓🅔‽ 🇺‌🇳‌🇮‌🇨‌🇴‌🇩‌🇪! 😄 The very name strikes fear and awe into the hearts of programmers worldwide. We all know we ought to “support Unicode” in our software (whatever that means—like using wchar_t for all the strings, right?). But Unicode can be abstruse, and diving into the thousand-page Unicode Standard plus its dozens of supplementary annexes, reports, and notes can be more than a little intimidating. I don’t blame programmers for still finding the whole thing mysterious, even 30 years after Unicode’s inception.";
    let text = "aaabdaaabac";
    let pairs = find_pairs(text.to_string());
}
