//struct or structure is a custom data type that lets you 
//packacke together and name multiple related values that make up 
//a meaningful group



//defining and instantiating Structs
struct User{
    active: bool,
    username:String, 
    email:String,
    sign_in_count:u64

}

// buld and return an object
fn build_user(email: String,username :String)-> User{
    User{
        active: true,
        username,
        email,
        sign_in_count:1,
    }

}


fn main() {

    let user1 = User{
        active: true,
        username: String::from("some123"),
        email: String::from("some@xxx.com"),
        sign_in_count:1,
    };
    

    user1.email = String::from("another@email.com");
    

    //often useful to create a new instance of a struct that 
    //includes most of the values from another instace but changes some
    //struct update Syntax does this
    
    let user2 = User{
        active: user1.active,
        username: user1.username,
        email: String::from("yyyy@xxxx.com"),
        sign_int_count: user1.sign_in_count,
    };


    let user2 = User{
        email: String::from("another@example.com"),
        ..user1
     };

    

}








