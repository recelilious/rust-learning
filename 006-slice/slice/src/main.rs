fn main() {
    let mut s = String::from("Hello, World!");
    let sl = &s[..5];
    println!("{s}");
    println!("{sl}");

    s.push_str(" Hi.");
    println!("{s}");
}
