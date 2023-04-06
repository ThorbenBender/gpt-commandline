use std::io;
use reqwest::Client;

enum Commands {
    Hello,
    Bye,
    Time,
    Unknown
}

enum Result {
    Ok(),
    Error(),
}



fn identify_command(input: &String) -> Commands {
    return match input.trim().to_lowercase().as_str() {
        "hello" => Commands::Hello,
        "bye" => Commands::Bye,
        "time" => Commands::Time,
        _ => Commands::Unknown
    };
}



async fn make_request() {
    let client = Client::new();
    let res = client.get("https://cat-fact.herokuapp.com/facts").send().await.expect("Failed to get data...").text().await.expect("failed to load payload");
    println!("{}", res);
}


fn execute_command(command: Commands) {
    match command {
        Commands::Bye => println!("Goodbye"),
        Commands::Hello => println!("Hi, how was your day?"),
        Commands::Time => println!("Sorry, i dont know the time."),
        Commands::Unknown => println!("Sorry the command doesnt exist!!!")
    }
}


#[tokio::main]
async fn main() {
    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input).expect("Could not read line...");
        let command = identify_command(&input);
        execute_command(command);
        make_request().await;
        input = String::new();
    }
}