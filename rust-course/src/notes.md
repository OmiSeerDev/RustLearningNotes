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
<p>Rust has no null types. If there's a type that is not known to be defined
it should be declared as Option<T> where T is the expected type. 
</p>


<p>Errors are datatypes. It can be used as Option<T, E>
Writing unwrap () displays the type if exists or the error.
</p>

<h3> Adding a cargo crate </h3>
<p>
    The keyword for using binaries and creates is <i>use</i>, 
    and it must need to be added to the cargo.toml file 
    in the depdendencies section
</p>
