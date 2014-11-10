
use sequence::sequence::Sequence;
use std::io::stdio;
use std::io::Timer;
use std::time::Duration;

pub struct Track {
    time_signature: uint,
    bpm: uint,
    title: String,
    sequences: Vec<Sequence>,
}

pub trait Playable {
    fn play(&self);
}

impl Playable for Track {
    fn play(&self) {
        println!("Playing track");
    }
}

impl Track {
    pub fn new(bpm: uint, title: String) -> Track {
        Track { time_signature: 4, bpm: bpm, title: title, sequences: vec![] }
    }

    pub fn add_sequence(&mut self, s: Sequence) {
        self.sequences.push(s);
    }

    pub fn play(&self) {
        let compiled_track = self.compile_track();
        // ((60/bpm)*time_signature)/8
        let step_time = ((((60f32/(self.bpm as f32))*(self.time_signature as f32))/8f32)*1000f32) as i64;
        let mut step_count: int = 0;
        let mut timer = Timer::new().unwrap();

        print!("Playing track: {}", self.title);

        for step in compiled_track.iter() {
            if step_count % 8 == 0 {
                println!("");
                print!("|");
            }
            print!("{}|",step);
            stdio::flush();
            step_count = step_count + 1;
            timer.sleep(Duration::milliseconds(step_time));
        }
        println!("");
    }

    fn compile_track(&self) -> Vec<String> {
        let mut longest_pattern = 0;

        // find the longest sequence, since that's how long the track will be
        // (as specified in Assumption 3)
        for s in self.sequences.iter() {
            if s.num_steps() > longest_pattern { longest_pattern = s.num_steps(); }
        }

        let mut track: Vec<String> = Vec::with_capacity(longest_pattern);

        for _ in range(0u,longest_pattern) {
            track.push("_".to_string());
        }

        for s in self.sequences.iter() {
            // how many times this pattern should play
            for i in range(0u, longest_pattern) {
                if s.hit(i) {
                    if track[i].as_slice() == "_" {
                        track[i] = s.get_sample();
                    } else {
                        track[i].push_str("+");
                        track[i].push_str(s.get_sample().as_slice());
                    }
                }
            }
        }

        track
    }
}

