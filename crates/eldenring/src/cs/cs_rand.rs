use shared::OwnedPtr;
use vtable_rs::VPtr;

#[vtable_rs::vtable]
pub trait CSRandVmt {
    fn destructor(&mut self, should_free: bool);
    fn next_uint(&mut self) -> u32;
    fn next_long(&mut self) -> u64;
}

pub struct CSRand<T: CSRandVmt> {
    pub vftable: VPtr<dyn CSRandVmt, T>,
}

impl<T: CSRandVmt> CSRandVmt for CSRand<T> {
    extern "C" fn destructor(&mut self, _should_free: bool) {}
    extern "C" fn next_uint(&mut self) -> u32 {
        0
    }
    extern "C" fn next_long(&mut self) -> u64 {
        0
    }
}

impl<T: CSRandVmt> CSRand<T> {
    pub fn next_uint(&mut self) -> u32 {
        unsafe {
            // We need to cast self to T to call the vtable methods
            let this = (self as *mut CSRand<T>) as *mut T;
            (self.vftable.next_uint)(&mut *this)
        }
    }
    pub fn next_long(&mut self) -> u64 {
        unsafe {
            // We need to cast self to T to call the vtable methods
            let this = (self as *mut CSRand<T>) as *mut T;
            (self.vftable.next_long)(&mut *this)
        }
    }
    /// Returns a random number in the range [min, max] \
    /// Implemetation is based on original game function
    pub fn rand_uint_range(&mut self, min: u32, max: u32) -> u32 {
        let range = (max - min) + 1;
        min + (self.next_uint() % range)
    }
    /// Returns a random signed number in the range [min, max] \
    /// Implemetation is based on original game function
    pub fn rand_int_range(&mut self, min: i32, max: i32) -> i32 {
        let range = (max - min) + 1;
        if range < 0 {
            return min;
        }
        min + (self.next_uint() as i32 % range)
    }
    /// Returns a random number in the range [0, n] \
    /// Implemetation is based on original game function
    pub fn rand_uint_up_to(&mut self, n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        let v = self.next_uint();
        v % (n + 1)
    }
    /// Returns a random float in the range [0.0, 1.0) \
    /// Implemetation is based on original game function
    pub fn rand_float_0_1(&mut self) -> f32 {
        let u = self.next_uint();
        let mantissa = u >> 9; // 23 bits
        let float_bits = 0x3f800000 | mantissa; // 1.0 + mantissa/2^23
        f32::from_bits(float_bits) - 1.0 // [0, 1)
    }
    /// Returns a random float in the range (0.0, 1.0] \
    /// Implemetation is based on original game function
    pub fn rand_float_open_0_1(&mut self) -> f32 {
        let u = self.next_uint();
        let temp = (u >> 9) + 1; // 1 to 2^23
        let carry = temp >> 23; // 0 or 1
        let mantissa = temp & 0x7fffff; // lower 23 bits
        let float_bits = 0x3f800000 + carry + mantissa;
        f32::from_bits(float_bits) - 1.0 // (0, 1]
    }
    /// Returns a random float in the range [0.0, 1.0] \
    /// Implemetation is based on original game function
    pub fn rand_float_0_1_inclusive(&mut self) -> f32 {
        let u = self.next_uint();
        let mantissa = u >> 9; // 23 bits
        let extra_bit = (u >> 8) & 1; // 1 bit
        let float_bits = 0x3f800000 + mantissa + extra_bit;
        f32::from_bits(float_bits) - 1.0 // [0, 1]
    }
    /// Returns a random float in the range [min, max] \
    /// Implemetation is based on original game function
    pub fn rand_float_range(&mut self, min: f32, max: f32) -> f32 {
        let range = max - min;
        if range <= 0.0 {
            return min;
        }
        range * self.rand_float_0_1() + min
    }
    /// Returns a random float in the range (min, max] \
    /// Implemetation is based on original game function
    pub fn rand_float_open_range(&mut self, min: f32, max: f32) -> f32 {
        let range = max - min;
        if range <= 0.0 {
            return min;
        }
        range * self.rand_float_open_0_1() + min
    }
}

#[repr(C)]
pub struct DLRandomGeneratorXorshift {
    pub state: [u32; 4],
}

#[repr(C)]
pub struct CSRandXorshift {
    pub base: CSRand<Self>,
    pub xorshift_state: DLRandomGeneratorXorshift,
}

impl CSRandXorshift {
    /// Replicates the game's Xorshift RNG initialization
    /// with Linear Congruential Generator seeding.
    pub fn new(seed: u64) -> Self {
        let mut state = [0u32; 4];

        let mut lcg_state = (seed & 0xffff_ffff) * 0x5deece66d + 0xb;
        for item in state.iter_mut() {
            *item = (lcg_state >> 32) as u32;
            lcg_state = lcg_state.wrapping_mul(0x5deece66d).wrapping_add(0xb);
        }

        CSRandXorshift {
            base: CSRand {
                vftable: VPtr::<dyn CSRandVmt, CSRandXorshift>::new(),
            },
            xorshift_state: DLRandomGeneratorXorshift { state },
        }
    }
}

impl CSRandVmt for CSRandXorshift {
    extern "C" fn destructor(&mut self, _should_free: bool) {}
    extern "C" fn next_uint(&mut self) -> u32 {
        let s1 = self.xorshift_state.state[0];
        let s2 = self.xorshift_state.state[1];
        let s3 = self.xorshift_state.state[2];
        let s4 = self.xorshift_state.state[3];

        let mut t = s1 ^ (s1 << 11);
        t ^= t >> 8;
        let new_s4 = s4 ^ (s4 >> 11) ^ t ^ s2;

        self.xorshift_state.state[0] = s2;
        self.xorshift_state.state[1] = s3;
        self.xorshift_state.state[2] = s4;
        self.xorshift_state.state[3] = new_s4;

        new_s4
    }
    extern "C" fn next_long(&mut self) -> u64 {
        let s1 = self.xorshift_state.state[0];
        let s2 = self.xorshift_state.state[1];
        let s3 = self.xorshift_state.state[2];
        let s4 = self.xorshift_state.state[3];

        let mut t1 = s1 ^ (s1 << 11);
        t1 ^= t1 >> 8;
        let new_s3 = s4 ^ (s4 >> 11) ^ t1 ^ s2;

        let mut t2 = s2 ^ (s2 << 11);
        t2 ^= t2 >> 8;
        let new_s4 = new_s3 ^ (new_s3 >> 11) ^ t2 ^ s3;

        self.xorshift_state.state[0] = s2;
        self.xorshift_state.state[1] = s3;
        self.xorshift_state.state[2] = new_s3;
        self.xorshift_state.state[3] = new_s4;

        ((new_s3 as u64) << 32) | (new_s4 as u64)
    }
}

#[repr(C)]
pub struct CSRandSFMT {
    pub base: CSRand<Self>,
    unk8: usize,
    pub state: DLRandomGeneratorSFMT,
}

#[repr(C)]
pub struct DLRandomGeneratorSFMT {
    pub state: [u32; 624],
    pub index: u32,
    pub mt_state_ptr: OwnedPtr<u32>,
    pub mt_state_end_ptr: OwnedPtr<u32>,
}

impl CSRandVmt for CSRandSFMT {
    extern "C" fn destructor(&mut self, _should_free: bool) {}
    extern "C" fn next_uint(&mut self) -> u32 {
        0
    }
    extern "C" fn next_long(&mut self) -> u64 {
        0
    }
}
