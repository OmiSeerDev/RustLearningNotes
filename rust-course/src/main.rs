fn main() {
    println!("Hello, Omar! Way to go!");
    let age: u8 = 28;
    // int Varibles can be changed to i and a number of bytes
    // (i8, i16, i32, i64, i128, i256) or u (unsigned).

    // Por defecto las variables son inmutables para mutarlas deben llevar mut
    let mut times: &str = "years";

     println!("Hi I'm Omar and I'm {} {} old.", times, age);

     times = "centuries";
     println!("{}" ,times);

     println!("Please insert your name: ");
     let mut name: String = String::new();
     std::io::stdin().read_line(&mut name).unwrap();

     println!("Congratulations, {}. You've made your first input log. 
     Insert a number: ", name);
     let mut firstNumber: String = String::new();
     std::io::stdin().read_line(&mut firstNumber).unwrap();

     let mut firstNumber_int: u32 = firstNumber.trim().parse().unwrap();
     print!("Your input number is: {}", firstNumber_int);

}
