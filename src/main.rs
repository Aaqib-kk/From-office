#[allow(dead_code)]
use std::f32;
use std::io;
fn main() {
    
    loop { 
    println!( "\n\t\t*** Simple Calculator ***\n\nEnter the Function you want to perform \n1) Addition\n2) Subtraction\n3) Multiplication\n4) Division\n5) Exponent\n6) Enter 0 to Quit");
    let mut choice = taking_input();
    let choice = convert_f32(&mut choice);

    if choice != 10.0 && choice != 0.0
    {
        println!("Please Enter you numbers\nEnter First Number ");
        let mut x = taking_input();
        let num1 = convert_f32(&mut x);
        if num1 == 10.0 {
            println!("\n*** PLEASE ENTER NUMBER AGAIN ***\n");
            continue;
        }

        println!("Enter Second Number");
        let mut y = taking_input();
        let num2 = convert_f32(&mut y);
        if num2 == 10.0 {
            println!("\n** PLEASE ENTER NUMBER AGAIN ***\n");
            continue;
        }
        
        match choice {
            1.0 => println!("Addition result is {}", num1 + num2),
            2.0 => println!("Subtraction result is {}", num1 - num2),
            3.0 => println!("Division result is {}", num1 / num2),
            4.0 => println!("Multiplication result is {}", num1 * num2),
            5.0 => println!("Exponent result is {}", num1.powf(num2)),
            _   => println!("Invalid number is added"),
        }
    }
    else if choice == 10.0 {
        continue;
        }
    else if choice == 0.0 {
        break;
        } 
    }
}

fn taking_input() -> String {
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("failed");
    num1
}

fn convert_f32(x: &mut String) -> f32 {
    
    match x.trim().parse::<f32>()
        {
        Ok(t) =>  { return t }
        Err(_t) => {  println!("You have entered invalid number!, Please Try Agin");
                    return 10.0 }
    }
}
