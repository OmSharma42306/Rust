// enums 
#[derive(Debug)]
enum Direction{
    North,
    South,
    East,
    West,
}

fn main(){
    let my_direction = Direction::East;
    
    move_around(my_direction);
    
}

fn move_around(direction:Direction){
    println!("{:?}",direction);

}



/*
Define an enum called TrafficLight with variants: Red, Yellow, Green.

Write a function action(light: TrafficLight) that uses match to print:

"Stop" for Red

"Ready" for Yellow

"Go" for Green

Create an enum called Result<T, E> and implement a function divide(a, b) that returns Ok(result) or Err("Divide by zero").
*/
#[derive(Debug)]
#[derive(PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green
}

fn action(light : TrafficLight){
    if(light == TrafficLight :: Red){
        println!("{:?}",light)
    }
    
}

fn main(){
    let my_light : TrafficLight = TrafficLight::Green;

    action(my_light);
}