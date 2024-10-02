
enum Direction {
    Left,
    Right,
}
enum Shapes{
    Circle(f32),
    Square(f32),
    Rectangle(f32, f32),
}
fn calculate_area(shape:Shapes)->i8{
    return 0
}
fn move_around(direction: Direction){
    println!("I am at left")

}



fn main() {
    //Structs
    let user = User{
        active:true,
        username: String::from("Suraj"),
        email: String::from("surajangurajnaidu@gmail.com"),
        sign_in_count: 3,
    };
    if user.active {
        println!("{} email is {}", user.username, user.email);
    }
    //Implements
    let res = Rectan{
        width:30,
        height:30,
    };
    println!("Area of the rectangle with width {} and height {} is {}", res.width, res.height, res.area());
    println!("Perimeter is {}",res.perimeter());
    //Enum
    let my_direction = Direction::Right;
    move_around(my_direction);

    let circle = Shapes::Circle(200.0);
    let square = Shapes::Square(200.0);
    let rect = Shapes::Rectangle(200.0, 200.0);
    calculate_area(circle);





}
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Rectan{
    width: u32,
    height: u32,
}
impl Rectan{
    fn area(&self)->u32{
        return self.width * self.height;
    }
    fn perimeter(&self)->u32{
        return (self.width + self.height)*2;
    }
}
