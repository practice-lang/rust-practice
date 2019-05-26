pub fn run() {
 //Print to Console
 println!("Hello World");
 //Basic Formatting
 println!("Number: {}", 1);
 println!("{} is from {}", "Brad", "Mass");
 //Positional Argument
 println!("{0} is form {1} and {0} likes to {2}", 
 "Brad", "Mass", "code");
 //Named Arguments
 println!("{name} likes to play {Activity}", 
 name="John", 
 Activity="Baseball"
 );
//Placeholder traits
 println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
//Placeholder for debug trait
 println!("{:?}", (23, true, "hello"));
 //Basic math
 println!("10 + 10 = {}", 10 + 10);
}