#[macro_use]
//imports crates. written by me and partner
mod solvestring;
use crate::solvestring::solvestring::solve_string;
fn main(){
    let inputs = ["9+2", "9-2", "9*2", "9/2", "9--2", "9*5+(6-5)"];
    for input in &inputs{
        solve_string(input.to_string());
    }
}