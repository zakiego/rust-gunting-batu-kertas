use rand::Rng;
use std::io::{self, Write};

const OPTION: [&str; 3] = ["Gunting", "Batu", "Kertas"];

fn main() {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0..OPTION.len());
    let computer_input: &str = OPTION[random];
    let player_input = get_player_input();

    let result = determine_winner(&player_input, computer_input);

    println!("\nComputer: {}", computer_input);
    println!("Player: {}", player_input);

    println!("\n{}", result);
}

fn get_player_input() -> String {
    let mut input = String::new();

    println!("Pilihan yang ada:");
    for i in 0..OPTION.len() {
        println!("{}. {}", i + 1, OPTION[i]);
    }
    print!("\nLu pilih yang mana? ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    input = input.trim().to_string();

    if !OPTION.contains(&&*input) {
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

#[test]
fn test_determine_winner() {
    assert_eq!(determine_winner("Gunting", "Batu"), "Lu kalah wkwk 🤪");
    assert_eq!(
        determine_winner("Gunting", "Kertas"),
        "Hebat juga, lu menang 🫡🥳"
    );
    assert_eq!(determine_winner("Kertas", "Gunting"), "Lu kalah wkwk 🤪");
    assert_eq!(
        determine_winner("Kertas", "Batu"),
        "Hebat juga, lu menang 🫡🥳"
    );
    assert_eq!(determine_winner("Batu", "Kertas"), "Lu kalah wkwk 🤪");
    assert_eq!(
        determine_winner("Batu", "Gunting"),
        "Hebat juga, lu menang 🫡🥳"
    );
    assert_eq!(determine_winner("Batu", "Batu"), "Yah, seri 😐");
    assert_eq!(determine_winner("Gunting", "Gunting"), "Yah, seri 😐");
    assert_eq!(determine_winner("Kertas", "Kertas"), "Yah, seri 😐");

    assert_eq!(determine_winner("Aaaaaa", "Aaaa"), "Ada error, check deh");
}
