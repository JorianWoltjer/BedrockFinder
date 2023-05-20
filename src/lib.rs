use std::num::Wrapping;  // `Wrapping()` functions make sure overflow is allowed where needed


pub trait Generator {
    fn new(world_seed: i64) -> Self;
    fn is_bedrock(&self, x: i32, y: i32, z: i32) -> bool;
}

#[inline(always)]
fn hashcode(x: i32, y: i32, z: i32) -> i64 {
    // Overflow is required here
    let mut l: i64 = (Wrapping(x) * Wrapping(3129871)).0 as i64 ^ (z as i64 * 116129781) ^ y as i64;

    l = (Wrapping(l) * Wrapping(l) * Wrapping(42317861) + Wrapping(l) * Wrapping(11)).0;
    
    l >> 16
}

pub struct LegacyGenerator {
    pub seed: i64,
}

impl LegacyGenerator {
    #[inline(always)]
    fn iterate_seed(seed: i64) -> i64 {
        ((Wrapping(seed) * Wrapping(25214903917)).0 + 11) & 0xffffffffffff  // 48 bits
    }

    fn set_seed_value(seed: i64) -> i64 {
        let mut seed = (seed ^ 0x5DEECE66D) & 0xffffffffffff;  // CheckedRandom.setSeed()

        // CheckedRandom.nextLong()
        seed = LegacyGenerator::iterate_seed(seed);
        let i = (seed >> (48 - 32)) as i32; // .next(32)

        seed = LegacyGenerator::iterate_seed(seed);
        let j = (seed >> (48 - 32)) as i32; // .next(32)

        let l = (i as i64) << 32;

        l + j as i64  // Return value
    }
}

impl Generator for LegacyGenerator {
    fn new(world_seed: i64) -> Self {
        let mut seed = world_seed;  // .create(seed) in NoiseConfig.java:50

        seed = LegacyGenerator::set_seed_value(seed);

        // CheckedRandom.split(String)
        let i = 2042456806;  // "minecraft:bedrock_floor".hashCode()
        seed ^= i;

        // Final .nextSplitter()
        seed = LegacyGenerator::set_seed_value(seed);
        
        Self { seed }
    }

    #[inline(always)]
    fn is_bedrock(&self, x: i32, y: i32, z: i32) -> bool {
        // CheckedRandom.split(x, y, z)
        let l = hashcode(x, y, z);
        let mut seed = l ^ self.seed;

        seed = (seed ^ 0x5DEECE66D) & 0xffffffffffff;  // CheckedRandom.setSeed()

        // nextFloat()
        seed = LegacyGenerator::iterate_seed(seed);

        let random = seed >> (48 - 24);

        match y {  // Threshold
            4 => random < 3355443,   // 0.2 / 5.9604645e-8
            3 => random < 6710886,   // 0.4 / 5.9604645e-8
            2 => random < 10066330,  // 0.6 / 5.9604645e-8
            1 => random < 13421773,  // 0.8 / 5.9604645e-8
            _ => {
                panic!("Unexpected y value: {y} (should be 1-4)");
            }
        }
    }

}

pub struct OldPaperLegacyGenerator {
    pub seed: i64,
}

// Paper versions before #213 (2022-10-15) have a bug where y=0 in the hashcode() function
impl Generator for OldPaperLegacyGenerator {
    fn new(world_seed: i64) -> Self {  // Same as in LegacyGenerator
        let mut seed = world_seed;

        seed = LegacyGenerator::set_seed_value(seed);

        let i = 2042456806;
        seed ^= i;

        seed = LegacyGenerator::set_seed_value(seed);
        
        Self { seed }
    }

    #[inline(always)]
    fn is_bedrock(&self, x: i32, y: i32, z: i32) -> bool {  // Almost same as in LegacyGenerator
        let l = hashcode(x, 0, z);  // ! Changed y=0 here (see https://github.com/PaperMC/Paper/pull/8474)
        let mut seed = l ^ self.seed;

        seed = (seed ^ 0x5DEECE66D) & 0xffffffffffff;

        seed = LegacyGenerator::iterate_seed(seed);

        let random = seed >> (48 - 24);

        match y {  // Threshold
            4 => random < 3355443,   // 0.2 / 5.9604645e-8
            3 => random < 6710886,   // 0.4 / 5.9604645e-8
            2 => random < 10066330,  // 0.6 / 5.9604645e-8
            1 => random < 13421773,  // 0.8 / 5.9604645e-8
            _ => {
                panic!("Unexpected y value: {y} (should be 1-4)");
            }
        }
    }

}
