fn main() {
    // Slices are a references to elements in a collection. They do not have ownership

    /* Example:
        Take a string, and return the first word split by spaces, if no spaces return the word.
     */
    let s = String::from("hello world it's cool");

    // Examples of slices
    let _hello = &s[0..5];
    let _world = &s[6..11];

    let first = first_word(&s);
    let second = second_word(&s);
    println!("The first word in: {s}, is {first} and the second is {second}.");

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] // Slice shortcut for entire reference. Works the same as &s
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut first_space_index: usize = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && first_space_index == 0 {
            first_space_index = i;
        } else if item == b' ' && first_space_index != 0 {
            return &s[first_space_index+1..i]
        }
    }

    return if first_space_index != 0 {
        &s[first_space_index + 1..]
    } else {
        &s
    }
}