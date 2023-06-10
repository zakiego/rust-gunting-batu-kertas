use rand::Rng;
use std::io::{self, Write};

fn main() {
    let option: [&str; 3] = ["Gunting", "Batu", "Kertas"];

    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0..3);
    let computer_input: &str = option[random];
    let player_input = get_player_input();

    let result = determine_winner(&player_input, computer_input);

    println!("\nComputer: {}", computer_input);
    println!("Player: {}", player_input);

    println!("\n{}", result);
}

fn get_player_input() -> String {
    let option: [&str; 3] = ["Gunting", "Batu", "Kertas"];

    let mut input = String::new();

    println!("Pilihan yang ada:");
    for i in 0..option.len() {
        println!("{}. {}", i + 1, option[i]);
    }
    print!("\nLu pilih yang mana? ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input = input.trim().to_string();

    if !option.contains(&&*input) {
        println!("Pilihan lu salah, masukin yang bener! 😡\n");
        return get_player_input();
    }

    return input;
}

fn determine_winner(player: &str, computer: &str) -> &'static str {
    let result;

    if player == computer {
        result = "Yah, seri 😐";
        return result;
    }

    let player_lose = "Lu kalah wkwk 🤪";
    let player_win = "Hebat juga, lu menang 🫡🥳";

    result = match (player, computer) {
        ("Gunting", "Batu") => player_lose,
        ("Gunting", "Kertas") => player_win,
        ("Kertas", "Gunting") => player_lose,
        ("Kertas", "Batu") => player_win,
        ("Batu", "Kertas") => player_lose,
        ("Batu", "Gunting") => player_win,
        _ => "Ada error, check deh",
    };

    return result;
}
