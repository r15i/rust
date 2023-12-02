
//no garbage , no allocate and free memory
//
//rust = memory managed with ownership
//key no feature of ownership will slow down the program
//HARD CONCEPT
//
//
//
//stack and heap
//pushing to the stack is faster because the heap has to find a place to 
//insert data
//
//
//OWNERSHIP RULES 
//
//
//1 EACH VALUE IN RUST HAS AN OWNER
//
//2 THERE CAN BE ONE OWNER AT A TIME
//
//3 WHEN THE OWNER GOES OUT OF SCOPE THE VALUE WILL BE DROPPED
//
//
//

fn main() {

    //s not in scope 
    // s in scope


    let mut s = String::from("hello");

    s.push_str(", pushed");

    print!("{}",s);


}
