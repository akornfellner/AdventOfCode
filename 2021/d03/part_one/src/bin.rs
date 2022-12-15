#[derive(Debug)]
pub struct Bin {
    pub bits: Vec<usize>,
    pub length: usize,
}

impl Bin {
    pub fn new(bits: Vec<usize>) -> Self {
        let l = bits.len();

        Bin { bits, length: l }
    }

    pub fn new_from_line(line: &str) -> Self {
        let mut bits: Vec<usize> = vec![];
        for c in line.chars() {
            bits.push(c as usize - 48)
        }

        Bin::new(bits)
    }

    pub fn get_dec(&self) -> usize {
        let mut v = usize::pow(2, (self.length - 1) as u32);

        let mut sum = 0usize;
        for bit in &self.bits {
            sum += bit * v;
            v /= 2;
        }

        sum
    }
}
