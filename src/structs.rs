// Structs - Used to create custom data types

// Traditional Struct
// struct Color {
//   red: u8,
//   green: u8,
//   blue: u8,
// }

// Tuple Struct
// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    } // constructor

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    } // NOTE: &self is referencing the struct of impl - Person

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    } // NOTE: since this is a setter (change something in the struct), add mut

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    } // NOTE: does not mutate any struct state, so no need mut
} // NOTE: like class ~ish | ref: https://doc.rust-lang.org/std/keyword.impl.html

// struct w/ generics
struct Rectangle<T, U> {
    length: T,
    height: U,
}

// trait - tie struct to function
trait Shape {
    fn new(length: f32, width: f32) -> Self; // return self indicates that it is a constructor
    fn area(&self) -> f32;
} // like a solidity interface

struct Rectangle2 {
    length: f32,
    width: f32,
}
struct Circle {
    length: f32,
    width: f32,
}

impl Shape for Rectangle2 {
    fn new(length: f32, width: f32) -> Rectangle2 {
        return Rectangle2 { length, width };
    } // constructor

    fn area(&self) -> f32 {
        return self.length * self.width;
    }
}

const PI: f32 = 3.214;
impl Shape for Circle {
    fn new(length: f32, width: f32) -> Circle {
        return Circle { length, width };
    } // constructor

    fn area(&self) -> f32 {
        return (self.length / 2.0).powf(2.0) * PI;
    }
}

pub fn run() {
    // let mut c = Color {
    //   red: 255,
    //   green: 0,
    //   blue: 0,
    // };

    // c.red = 200;

    // println!("Color: {} {} {}", c.red, c.green, c.blue);

    //
    //

    // let mut c = Color(255, 0, 0);

    // c.0 = 200;

    // println!("Color: {} {} {}", c.0, c.1, c.2);

    //
    //

    let mut p = Person::new("Mary", "Doe");
    println!("Person {}", p.full_name());
    p.set_last_name("Williams");
    println!("Person {}", p.full_name());
    println!("Person Tuple {:?}", p.to_tuple());

    let rec = Rectangle {
        length: 4,
        height: 7,
    }; // generic struct usage

    let rec2: Rectangle2 = Shape::new(10.0, 15.0);
    let cir: Circle = Shape::new(10.9, 8.9);
    println!("Rec Area {}", rec2.area());
    println!("Cir Area {}", cir.area());
}
