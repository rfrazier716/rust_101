fn display( s: &String) {
    println!("{s}");
}

fn append(s: &mut String, appended: String) {
    s.push_str(&appended);
}

fn main() {
    let x = 3;
    let mut s = String::from(if x > 5 { "Hello" } else { "World" });
    display(&s);
    append(&mut s, String::from("Hello"));
    display(&s);
}
