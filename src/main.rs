fn main() {
    println!("the Green light is: {}", TrafficLight::Green.time());
    println!("the Red light is: {}", TrafficLight::Red.time());
    println!("the yellow light is: {}", TrafficLight::Yellow.time());


    let vec = vec![10, 20, 30, u32::MAX];
    match int_vector_sum(vec) {
        Some(x) => println!("final result is: {}", x),
        None => println!("final result is: None"),
    }

    let triangle = Triangle { w: 10.0, h: 5.0 };
    print_area(&triangle);
    let rectangle = Rectangle { w: 10.0, h: 5.0 };
    print_area(&rectangle);
    let circle = Circle { r: 5.0 };
    print_area(&circle);
}

// Task 1:
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl Time for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Green => 50,
            TrafficLight::Red => 40,
            _ => 10,
        }
    }
}

pub trait Time {
    fn time(&self) -> u8;
}

// Task 2:
fn int_vector_sum(vec: Vec<u32>) -> Option<u32> {
    let sum: &mut u32 = &mut 0;
    for item in vec.iter() {
        match sum.checked_add(*item) {
            Some(x) => {
                *sum = x;
            },
            None => {
                return None;
            },
        }
    }
    return Some(*sum)
}

// Task 3:
fn print_area<T: Area>(shape: &T) {
    println!("The area of shape is: {}", shape.calculate_area());
}

pub trait Area {
    fn calculate_area(&self) -> f32;
}

struct Triangle {
    w: f32,
    h: f32,
}

impl Area for Triangle{
    fn calculate_area(&self)-> f32 {
        self.w * self.h / 2.0
    }
}

struct Rectangle {
    w: f32,
    h: f32,
}

impl Area for Rectangle{
    fn calculate_area(&self)-> f32 {
        self.w * self.h
    }
}

struct Circle {
    r: f32,
}

impl Area for Circle{
    fn calculate_area(&self)-> f32 {
        self.r * self.r * std::f32::consts::PI
    }
}
