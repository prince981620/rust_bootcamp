// define an enum called shape
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64)
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14 * radius*radius,
        Shape::Square(widht) => widht*widht,
        Shape::Rectangle(width,height ) => width*height
    }
}
fn main(){
    let circle =  Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    println!("Area of circle: {}",calculate_area(circle));
    println!("Area of square: {}",calculate_area(square));
    println!("Area of rectangle: {}",calculate_area(rectangle));
}