// 🎲 Dice Roller (Random number + match)

fn run_dice_roller(user_number:i32 , rng : &mut ThreadRng){
    let mut v : Vec<i32> = (1..100).collect();
    v.shuffle(rng);
    
    let r= v.choose(rng);
    
    println!("{:?},{}",r,user_number);
    
    match r {
        Some(&random_value) => {
            println!("{:?}{:?}",random_value,user_number);
            
            if user_number == random_value{
                println!("You Gueesed it Right");
            }else{
                println!("Yot Guessed it Wrong!");
            }
        }
        None =>{
             println!("⚠️ Failed to choose a number from the list.");
        }
        
    }

}

use rand::prelude::*;

fn main(){
let mut rng = rand::rng();

run_dice_roller(3,&mut rng);

}