fn main() {
    let mut s = String::from("Hello there!");
    s.push_str(" What's for dinner?");
    println!("s = {s}");
    let s2 = s.clone(); // must clone here, or s would move to s2 (go away)
    println!("now s = {s} and s2 = {s2}");
    change(&mut s);
    println!("{s}");
    println!("vvv Slice Stuff vvv");

    let sentence = String::from("This is s string.");
    println!("{}", first_word(&sentence));
}

fn change(s: &mut String) {
    s.push_str(" I'm hungry!")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        print!("{i}:{item}.");
        if item == b' ' {
            println!();
            return &s[..i]
        }
    }
    println!("\nCouldn't find a space.");
    &s[..]
}
