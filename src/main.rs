// src/main.rs
use clap::Parser;
use std::path::PathBuf;               // <-- to było brakujące

/// KeyOut-RS – wirtualna klawiatura ekranowa dla Linuksa w konsoli
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Ścieżka do pliku konfiguracyjnego
    #[arg(long)]
    config: Option<PathBuf>,

    /// Uruchom jako daemon
    #[arg(long)]
    daemon: bool,
}

fn main() {
    let args = Args::parse();

    println!("Witaj w KeyOut-RS!");
    println!("Daemon mode: {}", args.daemon);
    if let Some(config_path) = args.config {
        println!("Config file: {}", config_path.display());
    } else {
        println!("Brak pliku konfiguracyjnego – używamy domyślnego");
    }

    // Tu później będzie właściwa logika
    println!("Naciśnij Ctrl+C żeby wyjść");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
