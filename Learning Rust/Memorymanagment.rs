fn main() {
    //Memory management in Rust
    //Mutability Jargon
    let mut x = String::from("Suraj");
    x.push_str(" hi");
    println!("hello {}",x);
    //stack vs heap
    stack_fn();
    heap_fn();
    update_string();

}
fn stack_fn(){
    let a = 10;
    let b = 20;
    let c= a+b;
    println!("Stack function a is {} and b is {} and c is {}",a,b,c);

}
fn heap_fn(){
    let s1 = String::from("hello");
    let s2: String = String::from(" world");
    let combined = format!("{}{}", s1, s2);
    println!("Heap functional {}", combined);
}
fn update_string(){
    let mut s = String::from("hello");
    println!("{}", s);
    println!("Capacity {}, Length {}, pointer: {:p}",s.capacity(),s.len(),s.as_ptr());
    s.push_str(" world");
    println!("{}", s);
}


