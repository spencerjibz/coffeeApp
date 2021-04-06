use clap::{App, SubCommand};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use pad::*;
mod utils;
use utils::*;

fn main() {
    let matches = App::new("Coffee App")
        .version("0.1.0")
        .about("Simple command app")
        .subcommand(
            SubCommand::with_name("list")
                .alias("ls")
                .about("List the coffee menu"),
        )
        .subcommand(
            SubCommand::with_name("order")
                .alias("o")
                .about("Order a coffee"),
        );

    // handle matches

    //

    let sub_commands = matches.get_matches().subcommand;

    match sub_commands {
        None => {
            // matches.print_long_help();
            println!(
                "Coffee App 0.1.0
Simple command app

USAGE:
    Coffee App [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help     Prints this message or the help of the given subcommand(s)
    list     List the coffee menu
    order    Order a coffee"
            )
        }
        Some(command) => {
            if command.name == "list" {
                println!("COFFEE MENU");
                println!("----------------");

                for e in coffee() {
                    println!(
                        "{} /{}",
                        e.name.with_exact_width(10),
                        e.price.with_exact_width(10)
                    )
                }
            } else if command.name == "order" {
                //  prompt with Order options
                let selections: Vec<_> = coffee()
                    .iter()
                    .map(|cof| format!("{} ({})", cof.name, cof.price))
                    .collect();
                let coffee_selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("choose your coffee type")
                    .default(0)
                    .items(&selections)
                    .interact()
                    .unwrap();
                // prompt about the sugar level
                let sugar_levels: Vec<_> = sugar()
                    .iter()
                    .map(|sug| format!("{} ({})", sug.name, sug.spoons))
                    .collect();
                let sugar_selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("choose your sugar level")
                    .default(0)
                    .items(&sugar_levels)
                    .interact()
                    .unwrap();
                // confirm where you want decaf our not
                let decaf = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Do you prefer your coffee to be decaf?")
                    .interact();

                // ask if the wanted the coffee cold
                let cold = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Do you prefer your coffee to be cold?")
                    .interact();
                // questions about the vessel

                let vessels = vec!["Mug", "Cup", "Takeway package"];
                let vessel_selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("What  would you like your coffee to be served in ")
                    .default(0)
                    .items(&vessels)
                    .interact()
                    .unwrap();
                // stirred preference
                let stirrer = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Do you prefer your coffee with a stirrer?")
                    .interact();
                // Result
                println!(" YOUR ORDER");
                println!("-----------------------------------------------------");
                println!(
                    "{}{}",
                    "CoffeeType:".with_exact_width(30).truecolor(131, 131, 131),
                    selections[coffee_selection]
                );
                println!(
                    "{}{}",
                    "Sugar Level:".with_exact_width(30).truecolor(131, 131, 131),
                    sugar_levels[sugar_selection]
                );
                println!(
                    "{}{}",
                    "Decaf:".with_exact_width(30).truecolor(131, 131, 131),
                    decaf.unwrap()
                );
                println!(
                    "{}{}",
                    "Cold:".with_exact_width(30).truecolor(131, 131, 131),
                    cold.unwrap()
                );
                println!(
                    "{}{}",
                    "Served In:".with_exact_width(30).truecolor(131, 131, 131),
                    vessels[vessel_selection]
                );
                println!(
                    "{}{}",
                    "with stirrer".with_exact_width(30).truecolor(131, 131, 131),
                    stirrer.unwrap()
                );

                println!("-----------------------------------------------------");
                println!("Enjoy your order!")
            } else {
                // done next then
            }
        }
    }
}
