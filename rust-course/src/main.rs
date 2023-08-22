use regex::Regex;

fn main (){
 //Set regex
//(\d+)\s?\+\s?(\d+)
let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
let re_sus = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

 //Get user data
 println!("Please. Insert your expression.\n");
 let mut expression: String = String::new();
std::io::stdin().read_line(&mut expression).unwrap();
 //Use operators
//Multiplication
loop {
    // Iterative capture of regex values.  
 let caps = re_mult.captures(expression.as_str());
 
 if caps.is_none() {
     break;
 }
 
 let caps = caps.unwrap();
 
 let cap_expression = caps.get(0).unwrap().as_str();
 let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
 let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
 
 let product = left_value * right_value;
 expression = expression.replace(cap_expression, &product.to_string());
 
 }

// Addition
loop {
   // Iterative capture of regex values.  
let caps = re_add.captures(expression.as_str());

if caps.is_none() {
    break;
}

let caps = caps.unwrap();

let cap_expression = caps.get(0).unwrap().as_str();
let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
println!("{:?} left:{} right:{}", caps, left_value, right_value);

let addition = left_value + right_value;
expression = expression.replace(cap_expression, &addition.to_string());

}

loop {
    // Iterative capture of regex values.  
 let caps = re_sus.captures(expression.as_str());
 
 if caps.is_none() {
     break;
 }
 
 let caps = caps.unwrap();
 
 let cap_expression = caps.get(0).unwrap().as_str();
 let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
 let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
 println!("{:?} left:{} right:{}", caps, left_value, right_value);
 
 let sustraction = left_value - right_value;
 expression = expression.replace(cap_expression, &sustraction.to_string());
 
 }
 //Set result
 println!("{expression}");

}