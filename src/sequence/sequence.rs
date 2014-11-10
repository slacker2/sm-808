
pub struct Sequence {
    sample: String,
    pattern: u32, // limit pattern to 32 steps for right now
    steps: uint,
}

pub trait Playable {
    fn play(&self);
}

impl Playable for Sequence {
    fn play(&self) {
        self.play()
    }
}

impl Sequence {

    pub fn new(sample: &str, pattern: u32) -> Sequence {
        Sequence { sample: sample.to_string(), pattern: pattern, steps: Sequence::steps(pattern) }
    }

    pub fn get_sample(&self) -> String {
        self.sample.clone()
    }

    pub fn num_steps(&self) -> uint {
        self.steps
    }

    pub fn hit(&self, step: uint) -> bool {
        self.pattern & (1 << (step % self.steps)) != 0
    }

    fn steps(pattern: u32) -> uint {
        match pattern {
            1...0b11111111 => 8,
            0b100000000...0b1111111111111111 => 16,
            0b10000000000000000...4294967295 => 32,
            _ => 0,
        }
    }
}

