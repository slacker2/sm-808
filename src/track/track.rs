extern crate time;

use sequence::sequence::Sequence;
use std::convert::AsRef;
use std::io;
use std::io::prelude::*;
use std::process::Command;
use std::thread;

pub struct Track {
    time_signature: u32,
    bpm: u32,
    title: String,
    sequences: Vec<Sequence>,
}

fn parse_for_sound(sound: &String) -> String {
    match sound.as_ref() {
        "kick+snare" => String::from_str("boom click"),
        "kick"       => String::from_str("boom"),
        "snare"      => String::from_str("click"),
        "hihat"      => String::from_str("ting"),
        _            => String::from_str("")
    }
}

fn say_sound(sound: &String) {
    let parsed_sound = parse_for_sound(&sound);
    let output = Command::new("say")
                         .arg(parsed_sound)
                         .output()
                         .unwrap_or_else(|e| { panic!("Failed to execute process: {}", e) });

    output.stdout;
}

impl Track {
    pub fn new(bpm: u32, title: &str) -> Track {
        Track { time_signature: 4, bpm: bpm, title: title.to_string(), sequences: vec![] }
    }

    pub fn add_sequence(&mut self, s: Sequence) {
        self.sequences.push(s);
    }

    pub fn play(&self) {
        let compiled_track = self.compile_track();
        // ((60/bpm) * time_signature) / 8
        // Can potentially lose precision here if we're going to use f32 and u32
        let step_time = ((((60f32/(self.bpm as f32))*(self.time_signature as f32))/8f32)*1000f32) as u32;
        let mut step_count: u32 = 0;

        print!("Playing track: {}", self.title.to_string());

        for step in compiled_track.iter() {
            if step_count % 8 == 0 {
                println!("");
                print!("|");
            }
            print!("{}|", step);
            io::stdout().flush().unwrap();
            say_sound(&step);
            step_count = step_count + 1;
            thread::sleep_ms(step_time);
        }

        println!("");
    }

    fn compile_track(&self) -> Vec<String> {
        // Probably don't want to use usize
        let mut longest_pattern: u32 = 0;

        // Find the longest sequence, since that's how long the track will be
        // (as specified in Assumption 3)
        for s in self.sequences.iter() {
            if s.num_steps() > longest_pattern { longest_pattern = s.num_steps(); }
        }

        let mut track: Vec<String> = Vec::with_capacity(longest_pattern as usize);

        for _ in 0..longest_pattern {
            track.push("_".to_string());
        }

        for s in self.sequences.iter() {
            for i in 0..longest_pattern {
                let casted_i = i as usize;
                if s.hit(i) {
                    if &track[casted_i][..] == "_" {
                        track[casted_i] = s.get_sample();
                    } else {
                        track[casted_i].push_str("+");
                        track[casted_i].push_str(&s.get_sample()[..]);
                    }
                }
            }
        }

        track
    }
}
