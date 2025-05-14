use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("{}", "
 ██████╗ ██╗   ██╗███████╗███████╗███████╗     ██████╗  █████╗ ███╗   ███╗███████╗
██╔════╝ ██║   ██║██╔════╝██╔════╝██╔════╝    ██╔════╝ ██╔══██╗████╗ ████║██╔════╝
██║  ███╗██║   ██║█████╗  ███████╗███████╗    ██║  ███╗███████║██╔████╔██║█████╗  
██║   ██║██║   ██║██╔══╝  ╚════██║╚════██║    ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝  
╚██████╔╝╚██████╔╝███████╗███████║███████║    ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗
 ╚═════╝  ╚═════╝ ╚══════╝╚══════╝╚══════╝     ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝
                                                                                  
".green());

    println!("{}", "Welcome to the Number Guessing Game!".bold().green());
    println!("\nChoose difficulty:\n1. Easy (1–50)\n2. Medium (1–100)\n3. Hard (1–500)");
    print!("Enter choice (1/2/3): ");
    io::stdout().flush().unwrap();

    let mut difficulty_input = String::new();
    io::stdin()
        .read_line(&mut difficulty_input) 
        .expect("Failed to read line");
    let difficulty = difficulty_input.trim();

    let (max_number, max_attempts) = match difficulty {
        "1" => (50, 20),
        "2" => (100, 15),
        "3" => (500, 10),
        _ => {
            println!("{}", "Invalid input, Defaulting to medium".yellow());
            (100, 15)
        }
    };

    let secret_number = rand::thread_rng().gen_range(1..=max_number);
    println!("{} {} {}", "🔢 I'm thinking of a number between 1 and".green(), max_number.to_string().green().bold(), "... can you guess it?".green());
    println!("You have {} attempts.\n", max_attempts.to_string().cyan().bold());

    let mut attempts = 0;

    while attempts < max_attempts {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        let mut guess_input = String::new();
        io::stdin().read_line(&mut guess_input).unwrap();

        let guess: u32 = match guess_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "⚠️  Invalid input. Please enter a number.".red());
                continue;
            }
        };

        attempts += 1;
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} (Attempt {}/{})", "Too small! 📉".yellow(), attempts, max_attempts),
            Ordering::Greater => println!("{} (Attempt {}/{})", "Too big! 📈".yellow(), attempts, max_attempts),
            Ordering::Equal => {
                println!(
                "{} {} {} 🎉",
                "Correct! You guessed the number in".green().bold(),
                attempts.to_string().green().bold(),
                "tries.".green().bold());
                
                break;            
            }
        }

        if attempts == max_attempts {
            println!(
                "{} {}",
                "💀 Game over! The number was".red().bold(),
                secret_number.to_string().red().bold()
            );
        }

    }

    println!("{}", "\nThanks for playing! 🕹️".bright_green());
  
}

