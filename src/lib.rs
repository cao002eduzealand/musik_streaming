use std::fs::{self, File};
use std::io::{self, BufReader};
use std::sync::{Arc, Mutex};
use rodio::{Decoder};

pub struct Song {
    pub number: usize,
    pub name: String,
}

pub fn list_songs() -> Result<Vec<Song>, String> {
    let dir = fs::read_dir("src/music")
        .map_err(|_| "Kunne ikke finde 'music/' mappen!".to_string())?;

    let mut names: Vec<String> = dir
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".mp3") { Some(name) } else { None }
        })
        .collect();

    if names.is_empty() {
        return Err("Ingen .mp3 filer fundet i 'music/' mappen!".to_string());
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
    let path = format!("src/music/{}.mp3", song.name);

    let file = File::open(&path)
        .map_err(|_| format!("Kunne ikke finde filen: {}", path))?;

    let _stream = rodio::OutputStreamBuilder::open_default_stream()
        .map_err(|_| "Kunne ikke åbne lydenheden!".to_string())?;

    let sink = rodio::Sink::connect_new(&_stream.mixer());

    let source = Decoder::new(BufReader::new(file))
        .map_err(|_| "Kunne ikke afkode MP3-filen!".to_string())?;

    sink.append(source);


    println!("Afspiller: {}", song.name);
    println!("Kommandoer: [P] Pause  [R] Genoptag  [Q] Stop  [D] Afbryd forbindelse  [C] Genopret forbindelse");

    loop {
        if sink.empty() {
            println!("Sang færdig!");
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
                    println!("Ingen forbindelse! Genopret forbindelsen først med [C].");
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
            _ => println!("Ukendt kommando. Brug P, R, Q, D eller C."),
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
        println!("Forbindelse afbrudt! Musik sat på pause.");
    }
}