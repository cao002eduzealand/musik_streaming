use std::fs::{self, File};
use std::io::{self, BufReader};
use std::sync::{Arc, Mutex};
use rodio::{Decoder};

pub struct Song {
    pub number: usize,
    pub name: String,
}

pub fn list_songs() -> Result<Vec<Song>, String> {
    let dir = fs::read_dir("music/")
        .map_err(|_| "Kunne ikke finde 'music/' mappen".to_string())?;

    let mut names: Vec<String> = dir
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".mp3") { Some(name) } else { None }
        })
        .collect();

    if names.is_empty() {
        panic!("Ingen .mp3 filer fundet i 'music/' mappen");
    }

    names.sort();

    Ok(names.into_iter().enumerate().map(|(i, name)| Song {
        number: i + 1,
        name: name.trim_end_matches(".mp3").to_string(),
    }).collect())
}

pub fn find_song<'a>(songs: &'a Vec<Song>, number: usize) -> Option<&'a Song> {
    songs.iter().find(|s| s.number == number)
}

pub fn play_song(song: &Song, connected: Arc<Mutex<bool>>) -> Result<(), String> {
    let path = format!("music/{}.mp3", song.name);

    let file = File::open(&path)
        .map_err(|_| format!("Kunne ikke finde filen: {}", path))?;

    let (_stream_keep, stream_handle) = rodio::OutputStream::try_default()
        .map_err(|_| "Kunne ikke åbne lydenheden".to_string())?;

    let sink = rodio::Sink::try_new(&stream_handle)
        .map_err(|_| "Kunne ikke oprette afspiller".to_string())?;

    let source = Decoder::new(BufReader::new(file))
        .map_err(|_| "kunne ikke afkode MP3-filen".to_string())?;

    sink.append(source);


    println!("Afspiller: {}", song.name);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  [P]  Pause");
    println!("  [R]  Genoptag afspilning");
    println!("  [Q]  Stop og afslut");
    println!("  [D]  Simuler afbrudt internetforbindelse");
    println!("  [C]  Simuler genoprettet internetforbindelse");
    println!("  [+]  Skru op for lydstyrken");
    println!("  [-]  Skru ned for lydstyrken");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    loop {
        if sink.empty() {
            println!("Sang færdig");
            break;
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_uppercase().as_str() {
            "P" => {
                sink.pause();
                println!("Sat på pause.");
            }
            "R" => {
                let is_connected = *connected.lock().unwrap();
                if is_connected {
                    sink.play();
                    println!("Genoptager...");
                } else {
                    println!("Ingen forbindelse! Genopret forbindelsen først med [C]");
                }
            }
            "Q" => {
                sink.stop();
                println!("Stoppet.");
                break;
            }
            "D" => {
                simulate_internet(Arc::clone(&connected), false);
                sink.pause();
            }
            "C" => {
                simulate_internet(Arc::clone(&connected), true);
            }
            "+" => {
                let vol = sink.volume();
                sink.set_volume((vol + 0.1).min(1.0));
                println!("Lydstyrke: {:.0}%", sink.volume() * 100.0);
            }
            "-" => {
                let vol = sink.volume();
                sink.set_volume((vol - 0.1).max(0.0));
                println!("Lydstyrke: {:.0}%", sink.volume() * 100.0);
            }
            _ => println!("Ukendt kommando. Brug P, R, Q, D, C, + eller -"),

        }
    }

    Ok(())
}

pub fn simulate_internet(connected: Arc<Mutex<bool>>, status: bool) {
    let mut conn = connected.lock().unwrap();
    *conn = status;

    if status {
        println!("Forbindelse genoprettet! Tryk [R] for at genoptage.");
    } else {
        println!("Forbindelse afbrudt! musik sat på pause.");
    }
}