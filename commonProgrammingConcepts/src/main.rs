fn main() {
    
    println!("data types");
    //statistically typed language 
    //need to know at compile time the type
    //
    //
    
    //don't need to type annotation shown in the code 
    //need to write u32
    let guess: u32 = "42".parse().expect("Not a number");

    //scalar types
    //
    //scalar types 
    //intgersi isize, usize (signed and unsigned)
    //
    //number literals 
    //
    //DECIMAL   98_222
    //HEX       0xff
    //Octal     0o77
    //Binary    0b1111_0000
    //Byte(only for u8 ) b'A'


    //Integer overflow behaviour
    //1 rust check for integer overflow that cause to panic at runtime if 
    //  the behaviour occurs
    //2 when compiling with with --release flag rust does not include checks for integer overflow
    //
    //if overglow occurs rust performs 2 complement wrapping 
    //1 it become circular 
    //2 explicitly handle with some methods

    //floating point
    //
    //types f32,f34
    //


    //numeric operations
    //
    //basic mathematical operations 
    //sum + 
    
    //
    //multiplication *
    
    //truncated
    //division /
    
    //remainder %


    //BOOLEAN 
    //let t = true;
    //let t :bool = false;

    //char IS UNICODE NOT ASCII
    //let z = 'z';
    //let z: char = 'Z';
    //


    //TUPLE TYPE
    //
    //let tup: (i32,f64,u8) = (500,6.4,1);
    //
    //let tup =  (500,6.4,1);
    
    //index to the tuple
    //let five = tup.0;
    //

    //array
    //type of array and length
    //let a:[i32;5] =[1,2,3,4,5];
    //
    //implicit 
    //let month = ["1","2"]
    

    // an array with 5 istance of 3
    //let a = [3,5] 
    //
    //
    
    //invalid index 
    //end in a runtime error at the point of using an invalid value in the indexing operation 
    //the program exited with an error message and didn't execute the final println!
    //
    //rust check at runtime the length
    //and exit
    //

    




    println!(" ");

}
