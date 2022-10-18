use std::collections::HashMap;

fn main() {
    println!("Common Collections");

    // Vectors can contain any number of values
    let _v = vec![1,2,3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);


    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let fourth: Option<&i32> = v.get(3);
    match fourth {
        Some(third) => println!("The fourth element is {}", third),
        None => println!("There is no fourth element."),
    }


    // Iterate
    let v2 = vec![1,2,3,4,5,6];

    for i in &v2 {
        println!("{i}");
    }

    // Iterate and mutate
    let mut v_mut = vec![100,32,57];

    for i in &mut v_mut {
        // The star (*) is the dereference operator.
        *i += 50;
    }

    println!("{v_mut:?}");

    // Strings!
    let mut s = String::new();

    // like vectors you can add stuff;
    let s1 = "bar";
    s.push_str(s1);
    println!("{s}");

    // Or characters
    s.push('!');
    println!("{s}");

    // Concat
    let first_string = String::from("Hello, ");
    let second_string = String::from("world!");

    let hello_world = first_string + &second_string;
    println!("{hello_world}");

    let third_string = "Haha";
    // Or use the format macro
    let hello_world2 = format!("{}-{}", third_string, second_string);
    println!("{hello_world2}");

    // Iterate over a string with either characters or bytes;
    let s3 = "Hello, World!";
    for x in s3.chars() {
        println!("{x}");
    }

    for b in s3.bytes() {
        println!("{b}");
    }

    // Hash Maps

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Iterate over hash maps

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // TODO: Rust Exercises go here!





}
