use std::io;

fn main() {

    // input 

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("could not input string");

    let input = input.trim();


    // output

    let out = pl(input.to_string());

    println!("{}", out);
}

pub fn pl(input: String) -> String {

    // vowels 
    let vowel = vec!['a', 'e', 'i', 'o', 'u'];

    // first 3 characters 
    let char1 = input.chars().nth(0).unwrap();
    let char2 = input.chars().nth(1).unwrap();
    let char3 = input.chars().nth(2).unwrap();
    

    // initalizing tye variable 
    let mut tye = 0;

    // determines if the string is type one two or three

    for x in 0..vowel.len() {

        if char1 == vowel[x] {
            tye = 3;
        
        }else if char2 == vowel[x] {
            tye = 1;

        }else if char3 == vowel[x] {
            tye = 2;
    
        };

    };

    println!("{}", tye);
    // sends type and the input string to the converter
    return convert(input, tye)
}

pub fn convert(input: String, tye: i32) -> String {

    // initalizing first and second variable
    // piglatin words are split into three parts, the two that are switched around and a suffix at
    // the end
    let mut first = String::new();
    let mut second = String::new();

    // based on the type specified, the algorithim manipulates the string to convert it to piglatin
    if tye == 1 {

        for x in 0..input.chars().count() - 1 {
            first = first + &(input.chars().nth(x + 1).unwrap().to_string());
        };
        second = input.chars().nth(0).unwrap().to_string();

        return first + &second + "ay";


    }else if tye == 2 {
         for x in 0..input.chars().count() -2  {
            first = first + &(input.chars().nth(x + 2).unwrap().to_string());
        };
        second = input.chars().nth(0).unwrap().to_string() + &input.chars().nth(1).unwrap().to_string();

        return first + &second + "ay";


    }else if tye == 3 {
       
        let returner = input + "yay";
        return returner

    }else {
        
        return "invalid type".to_string()
    };
}
