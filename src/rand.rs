const N: usize = 624;
const M: usize = 397;
const F: u32 = 1812433253;

pub struct Rand {
    mt: [u32; N],
    index: usize,
}

impl Rand {
    pub fn init (seed: u32) -> Rand {
        let mut r: Rand = Rand {
            mt: [0; N],
            index: N,
        };

        r.mt[0] = seed & 0xffff_ffff;

        for i in 1..(N - 1)  {
            r.mt[i] = (F.wrapping_mul((r.mt[i - 1] ^ ((r.mt[i - 1] >> 30))) + i as u32)) & 0xffff_ffff;
        }

        return r
    }

    pub fn next (&mut self) -> u32 {
        if self.index >= N {
            self.update();
        }

        let mut y = self.mt[self.index];
        y ^= (y >> 11) & 0xffff_ffff;
        y ^= (y >> 7) & 0x9d2c_5680;
        y ^= (y >> 15) & 0xefc6_0000;
        y ^= y >> 18;

        self.index += 1;

        return (y >> 1) & 0xffff_ffff;
    }

    fn update (&mut self) {
        for i in 0..(N - M) {
            self.mt[i] = self.mt[i + M] ^ self.twist(self.mt[i], self.mt[i + 1]);
        }

        for i in (N - M)..(N - 1) {
            self.mt[i] = self.mt[(i + M) % N] ^ self.twist(self.mt[i], self.mt[i + 1]);
        }

        self.mt[N - 1] = self.mt[M - 1] ^ self.twist(self.mt[N - 1], self.mt[0]);
        self.index = 0;
    }

    fn twist (&self, a: u32, b: u32) -> u32 {
        let t = (a & 0x8000_0000) | (b & 0x7fff_ffff) >> 1;

        if b & 0x1 == 0 {
            return t;
        } else {
            return t ^ 0x9908_b0df;
        }
    }
}

