// It is like React, you need to import the libraries that you're using
use clap::{command, Arg, ArgMatches, Command};

fn main() {
    // Command is a library that makes us to use the things that he can do
    let match_result: ArgMatches = command!()
    // U can call the subcommand inside of the command
    .subcommand(
        Command::new("register-person")
            .arg(
                Arg::new("firstname")
                .short('f')
                .long("first-name")
                .aliases(["fname", "firstname"])
                .required(true)
                .help("The person's first name!")
                // .conflicts_with("lastname")
            )
            .arg(
                Arg::new("lastname")
                .short('l')
                .long("last-name")
                .aliases(["lname", "lastname"])
                .require_equals(true)
                .help("This argument takes the person's last name!")
            )
        )
    .subcommand(
        Command::new("register-pet")
            .arg(
            Arg::new("pet-name")
            .long("pet-name")
            .short('n')
            .aliases(["pname", "petname"])
            .require_equals(true)
            )
    )
    .about("This application register's people with their doctor's office.")
    .arg(
        Arg::new("fluffy")
        .long("fluffy")
        .help("Is the person wearing a fluffy coat or not?")
    )
    .get_matches();

    // println!("Pet name: {}", match_result.get_one::<String>("pet-name").unwrap_or(&"No pet name found".to_string()))
    // let pet_args = match_result.subcommand_matches("register-pet");
    // println!("Does pet name exist? {}", pet_args.unwrap().get_one::<String>("pet-name").unwrap());

    let person_args = match_result.subcommand_matches("register-person").unwrap();
    let fname = person_args.get_one::<String>("firstname").unwrap();
    let lname = person_args.get_one::<String>("lastname").unwrap();

    println!("First name: {} Last Name: {}", fname, lname);

}
