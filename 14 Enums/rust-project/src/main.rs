enum Direction {
    North,
    East,
    South,
    West,
}
fn move_around(direction: Direction) {
    // implements logic to move a character around
}


// Define an enum called Shape
enum Shape {
    Circle(f64),  // Variant with associated data (radius)
    Square(f64),  // Variant with associated data (side length)
    Rectangle(f64, f64),  // Variant with associated data (width, height)
}
// Function to calculate area based on the shape
fn calculate_area(shape: Shape) -> f64 {
    // calculates the area of the shape 
    return 0
}

fn main() {
    let my_direction = Direction::North;
    let new_direction = my_direction; // No error, because Direction is Copy
    move_around(new_direction);

    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);
}

