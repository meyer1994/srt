use std::fs::File;
use std::iter::Iterator;
use std::io::{BufRead, BufReader};

use clap::Clap;


#[derive(Debug, Clap)]
struct Args {
    #[clap(short, long)]
    file: String,

    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,
}


#[derive(Debug)]
struct Subtitle {
    id: u32,
    time: Vec<String>,
    text: Vec<String>,
}

#[derive(Debug)]
struct Srt {
    filename: String
}

impl Srt {
    fn new(filename: String) -> Srt {
        return Srt { filename: filename }
    }

    fn iter(&self) -> Iterator<Subtitle> {

    }
}

fn main() {
    let args: Args = Args::parse();

    let srt: Srt = Srt::new(args.file);


    // let mut subs: Vec<Subtitle> = Vec::new();

    // loop {
    //     let line = lines.next();
    //     if line.is_none() {
    //         break;
    //     }

    //     let line = line.unwrap();
    //     if line.is_empty() {
    //         continue;
    //     }

    //     // Id
    //     let id: u32 = line.parse::<u32>().unwrap();

    //     // Time
    //     let time = lines.next().unwrap();
    //     let time: Vec<String> = time.split("-->")
    //         .map(|s| s.trim().to_string())
    //         .collect();

    //     // Text
    //     let mut text: Vec<String> = Vec::new();
    //     loop {
    //         let line = lines.next();
    //         if line.is_none() {
    //             break;
    //         }
    //         let line = line.unwrap();

    //         if line.is_empty() {
    //             break;
    //         }
    //         text.push(line);
    //     }

    //     let sub = Subtitle { id: id, time: time, text: text };
    //     subs.push(sub);
    // }
}
