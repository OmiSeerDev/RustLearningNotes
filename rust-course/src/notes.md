<h1> Some notes regarding the language syntax </h1>
<p>
    
    //Main function
    fn main() {
    println!("Hello, Omar! Way to go!");
    let age: u8 = 28;

</p> 

<p>
    Integer variables can be changed to i and a number of bytes
     (i8, i16, i32, i64, i128, i256) or u (unsigned).
     Por defecto las variables son inmutables para mutarlas deben llevar <i>mut</i>.
     
</p>

    let mut times: &str = "years";

     println!("Hi I'm Omar and I'm {} {} old.", times, age);

     times = "centuries";
     println!("{}" ,times);
<h3> Doing input insertion of Strings </h3>
<p>
    
     println!("Please insert your name: ");
     let mut name: String = String::new();
     std::io::stdin().read_line(&mut name).unwrap();

     println!("Congratulations, {}. You've made your first input log. 
     Insert a number: ", name);
     let mut firstNumber: String = String::new();
     std::io::stdin().read_line(&mut firstNumber).unwrap();
     
</p>
<h3> Doing input insertion of numbers </h3>
<p>
    
     let mut firstNumber_int: u32 = firstNumber.trim().parse().unwrap();
</p>    
<h3> Condicionals </h3>
<p>
    
     if firstNumber_int > 18 {
        println!("Woow! you're an adult");
    } else {
         println!("Hey! you're underage!");
     }
</p>    
<h3> Loops </h3>
<p>
    
     let mut counter: u8 = 0;
     loop {
         counter = counter+1;
         println!("{counter}\n");
         if counter >= 9 {
            break;
        }
     }

     loop {
        println!("To exit write stop.");
        let mut select: String = String::new(); 
        std::io::stdin().read_line(&mut select).unwrap();
    // Is needed to clean the String input
        select = select.trim().to_string();
        if select == "stop" {
        break;
        }
     }
    }

</p>
<h3> Some type annotations </h3>

## There are 4 types of primitives
<ul>
    <li>int: that can be set to the bytes and if has a sign or not. i and u.</li>
    <li>float: that can be f32 and f64 (equivalent to float and double in C.)</li>
    <li>bool : True or false</li>
    <li>char: defined using 4 bytes, can get any unicode char and requires ' ' </li>
</ul>

## There are secondary types
<p>Tuples: Immutable series of data that can have different types within. </p>
<code>
 let one_tuple: (u32, u32,u32, f32) = (200, 10, 4, -32.1); 
</code>

<p>Arrays: Immutable series of data that cannot have different types within. </p>
<code>
 let array: [u32;3] = [200, 0, 24, 32]; 
 //One can access to elements is through the indexes.
</code>

<p>Vector: Immutable series of data that cannot have different types within. </p>
<code>
 let vector: Vec<u32>> = Vec::new();
  vector = [200, 0, 24, 32]; 
 //One can access to elements is through the indexes.

//Adding elements using:
 vector.push(element);

let mut friends: Vec<&str> = Vec::new();
friends.push("Omi");
friends.push("Santa");
friends.pop();
print!("{:?}", friends);
</code>

<p>
Rust has no null types. If there's a type that is not known to be defined
it should be declared as Option<T> where T is the expected type. 
</p>


<p>
Errors are datatypes. It can be used as Option<T, E>
Writing unwrap () displays the type if exists or the error.
</p>

<h3>Functions</h3>
<p>
Functions are used to shorten and modularize code.
</p>
<code>
fn add_one (num:u32){
   num = num +  1;
}

// Return types are set by using -> 

fn add_two_numbers (numA:u32 ,numB:u32) -> u32 {
    return numA + numB;
}
</code>

<h3> Adding a cargo crate </h3>
<p>
    The keyword for using binaries and creates is <i>use</i>, 
    and it must need to be added to the cargo.toml file 
    in the depdendencies section
</p>
