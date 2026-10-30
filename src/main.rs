use core::num;
/*
By: Aum markandey
Date: 2026-10-30
Program Details: a program to tell you weather you will be on the next
*/
use std::io;
fn getinput() -> String {
     let mut input = String::new();
          
    
     io::stdin().read_line(&mut input).expect("Failed to read line");
     input
}
fn main() {
    

    let spot_in_line: i32 = getinput().trim().parse().expect("Please type a number!");
   
        let number_of_Car: i32 = getinput().trim().parse().expect("Please type a number!");
    ;
        let People_Per_Car: i32 = getinput().trim().parse().expect("Please type a number!");
    let nexttrain =(number_of_Car*People_Per_Car)-spot_in_line;
    if nexttrain>=0{
        println!("yes");
    } else {
                println!("no");
    }



    
}
