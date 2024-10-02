fn main() {
    // Transfer of owners in Rust
    let mut my_string = String::from("hello");
    my_string = takes_ownership(my_string);
    // Borrowing the reference in Rust
    let s2 = &my_string;
    let s3 = &my_string;
    let s4= &my_string;
    println!("{}", my_string); // This line would cause a compile error because ownership has been moved.
    browsing(&mut my_string);
    println!("{}", my_string);
    //Mutable Reference
    let mut name = String::from("Suraj");
    name.push_str(" Naidu");
    println!("{}", name);
    update(&mut name);
    println!("{}", name);
    let s2 = &mut name;


}

fn takes_ownership(some_string: String)-> String {
    println!("{}", some_string); // `some_string` now owns the data.
    return some_string
}
fn browsing(some_string: &String){
    println!("{}", some_string);
}
fn update(some_string: &mut String){
    some_string.push_str("  1");
    println!("{}", some_string);

}