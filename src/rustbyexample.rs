/*
 * Task 1.0
 */
pub fn hello_world(){
    println!("Hello World");
}

/*
 * Task 1.2.1
 */
pub fn formatted_print(){
    let x = 5;
    println!("x = {}", x);
    println!("This is arg0: {0} this is arg1: {1} here comes arg0 again: {0}", "a0", "a1");

    println!("Base 10               : {}",   69420); 
    println!("Base 2 (binary)       : {:b}", 69420); 
    println!("Base 8 (octal)        : {:o}", 69420); 
    println!("Base 16 (hexadecimal) : {:x}", 69420); 
    println!("Base 16 (hexadecimal) : {:X}", 69420); 
}

/*
 * Task 1.2.2
 */
pub fn debug_print(){
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Person<'a>{
        name: &'a str,
        age: u8
    }

    let name = "Peter";
    let age = 29;
    let peter = Person{name, age};
    println!("Using the debug to print person struct: \n{:#?}", peter);

}
