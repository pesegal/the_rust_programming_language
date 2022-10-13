fn main() {
    let _s = "hello"; //String Literal

    let mut s2 = String::from("hello");
    s2.push_str(", world!");

    println!("{s2}");

    let x = s2;
    // let y = s2; wont work move error

    // Stack values are fine however since they are copied efficiently;
    let a = 1;
    let b = a;
    let c = a;

    // functions can take and return ownership
    let mut s3 = String::from("My String");
    s3 = take_and_give_back(s3);

    // but it can be a pain if we don't want to give ownership if not needed
    let mut s3 = String::from("My String Gotta Come Back");
    let (size, s3) = calculate_len_and_return_ownership_hard_way(s3);
    println!("{s3} | {size}");

    // this is were references come in handy
    let mut s4 = String::from("My String doesn't get sent in the first place");
    let size2 = calculate_len_using_ref(&s4);
    println!("{s4} | {size2}");

    // References are immutable by default but can me made mutable

    let mut s5 = String::from("My String ref will be ");
    add_more(&mut s5);
    println!(s5)


}

fn add_more(p0: &mut String) {
    p0.push_str("added to by this method.")
}

fn calculate_len_using_ref(p0: &String) -> usize {
    p0.len()
}

fn calculate_len_and_return_ownership_hard_way(p0: String) -> (usize, String) {
    let len = p0.len();
    (len, p0)
}


fn take_and_give_back(s: String) -> String {
    println!("This took and is going to give back ownership of = {s}");
    s
}