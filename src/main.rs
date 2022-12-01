use std::io;

fn main() {

    println!("Enter a string for conversion");

    // create string from input
    let mut arg = String::new();
    io::stdin().read_line(&mut arg).expect("invalid input");
  
    // print result
    println!("{}", pl_cc(arg.trim().to_string()));
}


fn pl_cc(mut arg: String) -> String {

    // vowel list for iterating
    let vowel = vec!['a', 'e', 'i', 'o', 'u'];

    // exceptions 
    if arg.len() < 3 {
        return "invalid string, minimum length of 3 chars".to_string();
    };

    // defines first and second character
    let (c1, c2) = ( arg.chars().nth(0).unwrap(), arg.chars().nth(1).unwrap());

    let mut tye = (false, false);
   
    for x in 0..vowel.len() {
        if c1 == vowel[x] { tye.0 = true};
        if c2 == vowel[x] { tye.1 = true};
    };

    let latin = match tye {
        // type one 
        (false, true) => { arg.remove(0);  format!("{}{}", (arg), c1)},
        // type two
        (false, false)   => { arg.remove(0);arg.remove(0); format!("{}{}{}", arg, c1, c2)},
        // type 3
        (_, _) => {arg},
    };

    format!("{latin}ay")
}
