#[derive(Debug)] // required for printing a struct directly
struct ExampleStruct {
    string_field: String,
    second_string_field: String,

    /*
        You can (and, in fact, need) to explicity declare integers - 
        whether they are signed (u/i - unsigned/signed)
        and their size (8,16,32,64,128 bytes - or 'size' for architecture used)
        ex. u64, i16, usize.
    */

    integer_field: isize,

    // comma at the end is not a mistake!
}

fn simple_concat(first_string: String, second_string: String) -> String {
    let output_string = format!("{} | {}", first_string, second_string);
    output_string

    // return output_string;
    // you return something by NOT adding a semilocon, or by using return
}

use std::io; // for stdio

fn main() {

    /* 
        Some printing examples
    */ 


    // A simple print
    println!("A simple print");

    // Print with an argument, way #1
    println!("{} {} {} {}", "Print", "with", "an", "argument");

    // Print with an argument, way #2
    println!("{3} {2} {1} {0}", "Print", "with", "an", "argument");

    // Print with named arguments (nice!)
    println!("{arg1} {arg2} {arg3} {arg4}", 
        arg1="Print", 
        arg2="with", 
        arg3="named", 
        arg4="arguments");

    // Print with formatting values
    println!("{arg1} = {arg1:b} in binary!", arg1=8);
    println!("{0} = {0:#X} in hex!", 255); // #x = ff, #X = FF (capitalised)
    println!("{0} = {0:#06X} in hex!", 255);

    

    /*  Format cheat sheet:
        :b -> binary


    */

    // Pretty printing!

    let pretty_print_string = ["Pretty", "printing", "is pretty!"];
    println!("{:#?}", pretty_print_string);

    // Decimals

    let decimal = 3.1415;
    println!("Examples of decimal for {}:", decimal);
    println!("1: {:.1}", decimal);
    println!("3: {:.3}", decimal);
    println!("5: {:.5}", decimal);
    println!("7: {:.7}", decimal);


    // Structs example

    let struct_test = ExampleStruct {
        second_string_field: String::from("String argument #1"), // don't have to be in order
        string_field: "String argument #2".to_string(),
        integer_field: 42,
    };
    // again, end with a comma

    // printing directly, with the Debug trait
    println!("Directly printing a struct: {:#?}", struct_test);

    // printing by accessing fields
    println!("Printing by accessing fields: {}", struct_test.string_field);

    // printing by function
    println!("Printing by function: {}", 
        simple_concat(String::from("First string"), "second string".to_string())
        );


}
