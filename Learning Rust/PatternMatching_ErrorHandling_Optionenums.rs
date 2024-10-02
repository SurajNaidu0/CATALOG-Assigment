use std::fs;
use std::path::Path;

enum Shape{
    Circle(i32),
    Square(i32),
    Rectangle(i32,i32),
}
// Pattern Matching
fn calculate_area(shape:Shape)->i32{
    let res = match shape{
        Shape::Circle(radius) => radius * radius,
        Shape::Square(size) => size * size,
        Shape::Rectangle(width, height) => width * height,
    };
    return res;
}
//Struct generic
struct Name<Y,N>{
    first_name:Y,
    last_name:Y,
    age:N,
    weight:N,


}
//Result enum
enum Result<A,B>{
    Ok(A),
    Err(B),
}
//Option Enum
// enum Option<T>{
//     None,
//     Some(T),
// }
fn main() {
    let circle = Shape::Circle(50);
    println!("{}",calculate_area(circle));
    //generic struct
    let person = Name{
        first_name:String::from("Suraj"),
        last_name:String::from("Naidu"),
        age:18,
        weight:70,
    };
    println!("{}",person.first_name);
    println!("{}",person.last_name);
    println!("{}",person.age);
    println!("{}",person.weight);
    //Error handling
    let res = fs::read_to_string(&Path::new("./README.md"));
    match res{
        Ok(text) => println!("{}",text),
        Err(e) => println!("{}",e),
    }
    let res1= find_first_a(String::from("Suraj Anguraj"));
    match res1 {
        Some(x) => println!("{}",x),
        None => println!("not found"),
    }




}
fn find_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

