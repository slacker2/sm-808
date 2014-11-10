
extern crate drum_machine;

use drum_machine::sequence::sequence::Sequence;
use drum_machine::track::track::Track;

fn main() {
    let mut song = Track::new(128, "Title".to_string());

    let first = Sequence::new("kick", 0b0001000100010001);
    song.add_sequence(first);
    song.add_sequence(Sequence::new("snare", 0b00010000));
    song.add_sequence(Sequence::new("hihat", 0b01000100));

    song.play();
}
