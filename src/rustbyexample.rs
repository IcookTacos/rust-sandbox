pub fn hello_world(){
    println!("Hello World");
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

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

pub fn primitive_scalars(){
    let unsigned_integer: u8 = 20;
    let signed_integer: i8 = -20;
    let largest_unsinged_integer: u8 = 255;
    println!("Unsigned 8 bit integer           : {}", unsigned_integer);
    println!("Signed 8 bit integer             : {}", signed_integer);
    println!("Largest unsinged 8 bit integer   : {}", largest_unsinged_integer);

    let mut infered_type = 12;
    print!("infered_type have type: ");
    print_type_of(&infered_type);

    infered_type = -5;
    print!("infered_type have type: ");
    print_type_of(&infered_type);
}

pub fn primitive_compounds(){
    
}
