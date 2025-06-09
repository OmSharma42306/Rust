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