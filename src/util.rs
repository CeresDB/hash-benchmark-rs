use std::hash::Hasher;

use byteorder::{ByteOrder, LittleEndian};
use rand::Rng;

pub fn hash64(mut bytes: &[u8]) -> u64 {
    use murmur3::murmur3_x64_128;

    let mut out = [0; 16];
    murmur3_x64_128(&mut bytes, 0, &mut out);
    // in most cases we run on little endian target
    LittleEndian::read_u64(&out[0..8])
}

#[derive(Debug, Default)]
pub struct MurmurHasher(u64);

impl Hasher for MurmurHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0 ^= hash64(bytes);
    }
}

pub fn gen_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = (0..length)
                .map(|_| rng.gen_range(0..36))
                .map(|n| if n < 26 { (n + 97) as u8 } else { (n - 26 + 48) as u8 } as char)
                .collect();
    chars.iter().collect()
}
