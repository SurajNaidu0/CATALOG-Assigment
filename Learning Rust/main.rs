fn main() {
    // Assigning a variable
    let x = -5;
    let y:u32 = 6000;
    let z:f32 = 700.567;
    // Print Statement
    println!("x:{}",x);
    print!("x:{},y:{},z:{}",x,y,z);
    println!("Hello, world!");
    // Boolean operations
    let is_true = true;
    let is_false = false;
    if is_true{
        print!("It is true");
    }else{
        print!("It is false");
    }
    let is_male = true;
    let mut is_above_18 = false;
    is_above_18 = true;
    if is_male && is_above_18{
        println!("Male");
    }
    //String
    let name: &str = "suraj";
    let name: String = String::from("Suraj");
    println!("Hello {}",name);
    let char1 = name.chars().nth(0);
    print!("{}",char1.unwrap()); // unwraping the option
    //Conditionals and Loops
    let is_even = true;
    if is_even{
        println!("Even");
    }else if !is_even{
        println!("Odd");
    }else{
        println!("Its in a quantum state")
    }
    //For loop
    for i in 0..11{
        println!("{}",i);
    }
    //Iterating String Array Maps
    let sentence: String= String::from("hi my name is suraj");
    let first_word = get_first_word(sentence);
    println!("The first word is {}", first_word);
    println!("the addition of 7+9 is {}",add(7,9));

}
//Creating Functions
fn get_first_word(sentence: String)-> String {
    let mut ans : String = String::from("");
    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char == ' '{
            break;
        }
    }
    return ans;
}
fn add(a:i32, b:i32)->i32{
    return a+b;
}
