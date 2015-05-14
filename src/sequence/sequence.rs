pub struct Sequence {
    sample: String,
    pattern: u32, // Limit pattern to 32 steps for right now
    //steps: uint, // Generally you'll want to use u32 ... isize and usize not recommended to be used since they are platform specific and because you're doing 64 bit addition (int platform specific)
    steps: u32,
}

impl Sequence {
    pub fn new(sample: &str, pattern: u32) -> Sequence {
        Sequence { sample: sample.to_string(), pattern: pattern, steps: Sequence::steps(pattern) }
    }

    pub fn get_sample(&self) -> String {
        self.sample.clone()
    }

    pub fn num_steps(&self) -> u32 {
        self.steps
    }

    pub fn hit(&self, step: u32) -> bool {
        self.pattern & (1 << (step % self.steps)) != 0
    }

    fn steps(pattern: u32) -> u32 {
        match pattern {
            1...0b11111111 => 8,
            0b100000000...0b1111111111111111 => 16,
            0b10000000000000000...4294967295 => 32,
            _ => 0,
        }
    }
}
