extern crate drum_machine;

use drum_machine::sequence::sequence::Sequence;
use drum_machine::track::track::Track;

fn main() {
    let mut song = Track::new(128, "Animal Rights");

    // Patterns are read from right to left as binary.
    // A 1 indicates a hit, a 0 indicates a skip.
    // For this implementation, the pattern is an u32
    // (An unsigned integer, 32 bits in length)

    song.add_sequence(Sequence::new("kick", 0b0001000100010001));
    song.add_sequence(Sequence::new("snare", 0b00010000));
    song.add_sequence(Sequence::new("hihat", 0b01000100));

    song.play();
}
