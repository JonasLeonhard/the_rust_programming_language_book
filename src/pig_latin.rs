/// Convert Strings to pig latin.
/// The first consonant of each word is moved to the end of the word and “ay” is added,
/// so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added
/// to the end instead (“apple” becomes “apple-hay”).
/// Keep in mind the details about UTF-8 encoding!
pub fn convert_to_piglatin(mut string: String) -> String {
    let first = string.chars().next().unwrap(); // gets the first char

    match first {
        'a' | 'e' | 'i' | 'o' | 'u' => string += "-hay",
        _ => {
            string.replace_range(..1, "");
            string.push(first); // string += &first.to_string();
            string += "-ay"; // string = format!("{}-{}", string, "ay");
        }
    }
    //string.replace_range(string.len()-1..string.len(), "="); //replace last char with '='
    string
}

// /// Convert str literals to pig latin.
// /// The first consonant of each word is moved to the end of the word and “ay” is added,
// /// so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added
// /// to the end instead (“apple” becomes “apple-hay”).
// /// Keep in mind the details about UTF-8 encoding!
// pub fn convert_str_to_piglatin(str: &str) -> String {
//     let mut string = String::from(str); // get a String from str
//     let first = str.chars().next().unwrap(); // gets the first char of a str literal

//     // string.replace_range(string.len()-1..string.len(), "heyo");

//     match first {
//         'a' | 'e' | 'i' | 'o' | 'u' => string += "-hay",
//         _ => {
//             string.replace_range(..1, "");
//             string += &first.to_string();
//             string += "-ay";
//         }
//     }
//     //string.replace_range(string.len()-1..string.len(), "=");
//     string
// }
