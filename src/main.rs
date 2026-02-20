use std::io;
use std::sync::{Arc, Mutex};
use musik_streaming::{list_songs, find_song, play_song};

fn main() {
    let songs = match list_songs() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Fejl: {}", e);
            return;
        }
    };

    println!("Tilgængelige sange:");
    for song in &songs {
        println!("{}: {}", song.number, song.name);
    }

    println!("\nSkriv nummeret på sangen du vil afspille:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let valgt: usize = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Fejl: Indtast venligst et tal!");
            return;
        }
    };

    let song = match find_song(&songs, valgt) {
        Some(s) => s,
        None => {
            eprintln!("Fejl: Ingen sang med nummer {}!", valgt);
            return;
        }
    };

    let connected = Arc::new(Mutex::new(true));

    if let Err(e) = play_song(song, connected) {
        eprintln!("Fejl under afspilning: {}", e);
    }
}