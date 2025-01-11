use regex::Regex;
use std::fs;
use std::io::{self, Write};
use std::process::{Command, Stdio};

fn validate_youtube_url(url: &str) -> bool {
    Regex::new(r"^(https?://)?(www\.)?(youtube\.com|youtu\.be)/.+$")
        .unwrap()
        .is_match(url)
}

fn get_user_input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_default();
    input.trim().to_owned()
}

fn download_video(url: &str, format: &str) {
    let output_dir = "uploads";

    if fs::create_dir_all(output_dir).is_err() {
        eprintln!("Failed to create output directory: {}", output_dir);
        return;
    }

    let mut command = Command::new("yt-dlp");

    command.arg(url);
    command
        .arg("-o")
        .arg(format!("{}/%(title)s.%(ext)s", output_dir));

    if format == "mp3" {
        command.args(&["--extract-audio", "--audio-format", "mp3"]);
    } else {
        command.args(&["-f", "bestvideo+bestaudio"]);
    }

    command.stdout(Stdio::null());
    command.stderr(Stdio::null());

    println!("Waiting for download to complete...");

    let status = command.status();

    match status {
        Ok(status) if status.success() => println!("Download completed successfully!"),
        Ok(_) => eprintln!("Download failed."),
        Err(e) => eprintln!("Error starting yt-dlp: {}", e),
    }
}

fn main() {
    let url = loop {
        let url = get_user_input("Enter the YouTube URL: ");
        if validate_youtube_url(&url) {
            break url;
        } else {
            println!("Invalid YouTube URL. Please try again.");
        }
    };

    let format = loop {
        let format =
            get_user_input("Do you want to download as MP3 or MP4? (type 'mp3' or 'mp4'): ")
                .to_lowercase();
        if format == "mp3" || format == "mp4" {
            break format;
        } else {
            println!("Invalid format. Please type 'mp3' or 'mp4'.");
        }
    };

    download_video(&url, &format);
}
