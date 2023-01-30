fn display( s: &String) {
    println!("{s}");
}

fn append(s: &mut String, appended: &String) {
    s.push_str(appended);
}

fn bad_substring(s: &String, end_index: usize) -> &str {
    &s[0..end_index]
}

fn main() {
    let x = 3;
    let mut s = String::from(if x > 5 { "Hello" } else { "World" });
    display(&s);
    let to_append = String::from(" Hello");
    append(&mut s, &to_append);
    let test = bad_substring(&s, 2);
    append(&mut s, &to_append);
    println!("{test}");
    display(&s);
}
