// An example of slices in Rust
fn main() {
    let mut s = String::from("Hello world!");

    let index = first_word_index(&s); // index = 5

    println!("The index of the space is {index}.");

    // Since 'index' has no actual relation to 's', this doesn't affect 'index':
    s.clear(); // s is now an empty string ("")

    if index == 5 {
        println!("Index doesn't care about s.");
    }

    // Re-declare s
    let s = String::from("Hello world!");

    let first_word = first_word_slice(&s);

    // s.clear(); // ERROR: The string has already been immutably borrowed and will be used later!

    println!("The first word is: {first_word}");
}

// Write a function that takes a string of words separated by spaces and returns the first word it finds in that string. 
// If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.

// This function returns the index of the first space it finds, or the whole string if no space is found.
fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// Returns a slice (like a pointer collection) of the given string, in this case the first word.
fn first_word_slice(s: &String) -> &str { // "&str" is the "string slice" type
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // Removing the starting or ending number means "Start from the beginning" or "Go to the end" respectively
        }
    }

    &s[..]
}


