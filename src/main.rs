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

    loop {
        println!("\nTilgængelige sange:");

        for song in &songs {
            println!("{}: {}", song.number, song.name);
        }
        println!("\nTryk 'q for afslutte programmet");
        println!("\nSkriv nummeret på sangen du vil afspille:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.eq_ignore_ascii_case("q") {
            break;
        }
        let valgt: usize = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Fejl: Indtast venligst et tal");
                continue;
            }
        };

        let song = match find_song(&songs, valgt) {
            Some(s) => s,
            None => {
                eprintln!("Fejl: Ingen sang med nummer {}", valgt);
                continue;
            }
        };

        let connected = Arc::new(Mutex::new(true));

        if let Err(e) = play_song(song, connected) {
            eprintln!("fejl under afspilning: {}", e);
        }
    }
}