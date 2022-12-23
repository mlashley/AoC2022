fn main() {
    let mut s = String::from("Fuckyou");
    println!("{}",s);
    s = (&s[2..]).to_string();
    println!("{}",s);
}
