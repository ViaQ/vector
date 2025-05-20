// Copyright 2025 Don MacAskill. Licensed under MIT or Apache-2.0.

//! This module provides AVX2 and VPCLMULQDQ-specific implementations of the ArchOps trait.

#![cfg(all(target_arch = "x86_64", feature = "vpclmulqdq"))]

use crate::arch::x86::X86Ops;
use crate::enums::Reflector;
use crate::structs::CrcState;
use crate::traits::{ArchOps, EnhancedCrcWidth};
use std::arch::x86_64::*;
use std::ops::BitXor;

/// Implements the ArchOps trait using 256-bit AVX2 and VPCLMULQDQ instructions
/// Delegates to X86Ops for standard 128-bit operations
#[derive(Debug, Copy, Clone)]
pub struct VpclmulqdqOps(X86Ops);

impl VpclmulqdqOps {
    #[inline(always)]
    pub fn new() -> Self {
        Self(X86Ops)
    }
}

// Wrapper for __m256i to make it easier to work with
#[derive(Debug, Copy, Clone)]
struct Simd256(__m256i);

impl Simd256 {
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn new(x3: u64, x2: u64, x1: u64, x0: u64) -> Self {
        Self(_mm256_set_epi64x(
            x3 as i64, x2 as i64, x1 as i64, x0 as i64,
        ))
    }

    #[inline]
    #[target_feature(enable = "avx2", enable = "vpclmulqdq")]
    unsafe fn fold_32(&self, coeff: &Self) -> Self {
        let result = _mm256_xor_si256(
            _mm256_clmulepi64_epi128(self.0, coeff.0, 0x00),
            _mm256_clmulepi64_epi128(self.0, coeff.0, 0x11),
        );

        Self(result)
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn extract_u64s(&self) -> [u64; 4] {
        let mut result = [0u64; 4];
        _mm256_storeu_si256(result.as_mut_ptr().cast(), self.0);

        result
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    #[allow(unused)]
    unsafe fn from_128i(low: __m128i, high: __m128i) -> Self {
        Self(_mm256_inserti128_si256(
            _mm256_castsi128_si256(low),
            high,
            1,
        ))
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn to_128i_low(self) -> __m128i {
        _mm256_castsi256_si128(self.0)
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn to_128i_high(self) -> __m128i {
        _mm256_extracti128_si256(self.0, 1)
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn xor(&self, other: &Self) -> Self {
        Self(_mm256_xor_si256(self.0, other.0))
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    #[allow(unused)]
    unsafe fn print_hex(&self, prefix: &str) {
        let values = self.extract_u64s();
        println!(
            "{}={:#016x}_{:016x}_{:016x}_{:016x}",
            prefix, values[3], values[2], values[1], values[0]
        );
    }

    #[inline]
    #[target_feature(enable = "avx2,sse2,sse4.1")]
    #[allow(unused)]
    unsafe fn from_m128i_pair(high: __m128i, low: __m128i) -> Self {
        // Extract u64 values from the __m128i registers
        let mut high_u64s = [0u64; 2];
        let mut low_u64s = [0u64; 2];

        _mm_storeu_si128(high_u64s.as_mut_ptr() as *mut __m128i, high);
        _mm_storeu_si128(low_u64s.as_mut_ptr() as *mut __m128i, low);

        // Create Simd256 using the extracted u64 values
        // The order matters to ensure consistent data representation:
        // high_u64s[1], high_u64s[0], low_u64s[1], low_u64s[0]
        Self::new(high_u64s[1], high_u64s[0], low_u64s[1], low_u64s[0])
    }
}

// VPCLMULQDQ-optimized implementation for large inputs
impl VpclmulqdqOps {
    /// Process aligned blocks using VPCLMULQDQ
    #[inline]
    #[target_feature(enable = "avx2,vpclmulqdq,sse2,sse4.1,pclmulqdq")]
    unsafe fn process_vpclmulqdq_blocks<W: EnhancedCrcWidth>(
        &self,
        state: &mut CrcState<<VpclmulqdqOps as ArchOps>::Vector>,
        first: &[__m128i; 8],
        rest: &[[__m128i; 8]],
        keys: [u64; 21],
        reflected: bool,
    ) -> W::Value
    where
        W::Value: Copy + BitXor<Output = W::Value>,
    {
        let state_u64s = self.extract_u64s(state.value);

        // Position the state correctly based on reflection mode
        let positioned_state = if reflected {
            // For reflected mode, state goes in the low bits of the low 64-bit word
            Simd256::new(0, 0, 0, state_u64s[0])
        } else {
            // For non-reflected mode, state goes in the high bits of the high 64-bit word
            Simd256::new(state_u64s[1], 0, 0, 0)
        };

        let reflector = create_reflector256(reflected);

        // Initialize the 4 x 256-bit registers for first block
        // Convert __m128i to Simd256 and apply reflection in a single step
        let mut x: [Simd256; 4] = [
            reflect_bytes256(&reflector, Simd256::from_m128i_pair(first[1], first[0])),
            reflect_bytes256(&reflector, Simd256::from_m128i_pair(first[3], first[2])),
            reflect_bytes256(&reflector, Simd256::from_m128i_pair(first[5], first[4])),
            reflect_bytes256(&reflector, Simd256::from_m128i_pair(first[7], first[6])),
        ];

        x[0] = positioned_state.xor(&x[0]);

        // Create the folding coefficient
        let coeff = self.create_vpclmulqdq_coefficient(keys, reflected);

        // Process remaining blocks
        for block in rest {
            for (i, chunk) in x.iter_mut().enumerate() {
                let reflected_chunk = reflect_bytes256(
                    &reflector,
                    Simd256::from_m128i_pair(block[i * 2 + 1], block[i * 2]),
                );

                *chunk = chunk.fold_32(&coeff).xor(&reflected_chunk);
            }
        }

        // Fold from 4 x 256-bit to 1 x 128-bit
        let folded = self.fold_from_256_to_128(x, keys, reflected);

        // leverage existing 128-bit code to finalize
        W::perform_final_reduction(folded, reflected, keys, self)
    }

    /// Create a folding coefficient for VPCLMULQDQ
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn create_vpclmulqdq_coefficient(&self, keys: [u64; 21], reflected: bool) -> Simd256 {
        let (k1, k2) = if reflected {
            (keys[3], keys[4])
        } else {
            (keys[4], keys[3])
        };

        Simd256::new(k1, k2, k1, k2)
    }

    /// Fold from 4 x 256-bit to 1 x 128-bit
    #[inline]
    #[target_feature(enable = "avx2,sse2,sse4.1,pclmulqdq")]
    unsafe fn fold_from_256_to_128(
        &self,
        x: [Simd256; 4],
        keys: [u64; 21],
        reflected: bool,
    ) -> __m128i {
        // Create the fold coefficients for different distances
        let fold_coefficients = [
            // 112, 96, 80, 64, 48, 32, 16 bytes
            self.create_vector_from_u64_pair(keys[10], keys[9], reflected),
            self.create_vector_from_u64_pair(keys[12], keys[11], reflected),
            self.create_vector_from_u64_pair(keys[14], keys[13], reflected),
            self.create_vector_from_u64_pair(keys[16], keys[15], reflected),
            self.create_vector_from_u64_pair(keys[18], keys[17], reflected),
            self.create_vector_from_u64_pair(keys[20], keys[19], reflected),
            self.create_vector_from_u64_pair(keys[2], keys[1], reflected),
        ];

        // Extract the 8 x 128-bit vectors from the 4 x 256-bit vectors
        let v128 = if reflected {
            [
                x[0].to_128i_low(),
                x[0].to_128i_high(),
                x[1].to_128i_low(),
                x[1].to_128i_high(),
                x[2].to_128i_low(),
                x[2].to_128i_high(),
                x[3].to_128i_low(),
                x[3].to_128i_high(),
            ]
        } else {
            [
                x[0].to_128i_high(),
                x[0].to_128i_low(),
                x[1].to_128i_high(),
                x[1].to_128i_low(),
                x[2].to_128i_high(),
                x[2].to_128i_low(),
                x[3].to_128i_high(),
                x[3].to_128i_low(),
            ]
        };

        // Fold to a single 128-bit vector
        let mut acc = v128[7];
        for i in 0..7 {
            // Fold and XOR
            let folded = self.carryless_mul_00(v128[i], fold_coefficients[i]);
            let folded2 = self.carryless_mul_11(v128[i], fold_coefficients[i]);
            acc = self.xor_vectors(acc, self.xor_vectors(folded, folded2));
        }

        acc
    }
}

// First, define a 256-bit version of the Reflector
#[derive(Clone, Copy)]
enum Reflector256 {
    NoReflector,
    ForwardReflector { smask: Simd256 },
}

// Function to create the appropriate reflector based on CRC parameters
#[inline]
#[target_feature(enable = "avx2")]
unsafe fn create_reflector256(reflected: bool) -> Reflector256 {
    if reflected {
        Reflector256::NoReflector
    } else {
        // Load shuffle mask similar to the 128-bit implementation
        // Using the same constants from Width64::load_constants
        let smask = Simd256::new(
            0x08090a0b0c0d0e0f,
            0x0001020304050607,
            0x08090a0b0c0d0e0f,
            0x0001020304050607,
        );
        Reflector256::ForwardReflector { smask }
    }
}

// Function to apply reflection to a 256-bit vector
#[inline]
#[target_feature(enable = "avx2,sse2,sse4.1")]
unsafe fn reflect_bytes256(reflector: &Reflector256, data: Simd256) -> Simd256 {
    match reflector {
        Reflector256::NoReflector => data,
        Reflector256::ForwardReflector { smask } => {
            // Perform shuffle at the byte level
            // This would require implementing a 256-bit shuffle operation
            // Either using existing AVX2 instructions or two 128-bit shuffles
            shuffle_bytes256(data, *smask)
        }
    }
}

// Implement a 256-bit byte shuffle function with correct u64 block ordering
#[inline]
#[target_feature(enable = "avx2,sse2,sse4.1")]
unsafe fn shuffle_bytes256(data: Simd256, mask: Simd256) -> Simd256 {
    let ops = X86Ops;

    // Extract the u64 values
    let values = data.extract_u64s(); // [u64_0, u64_1, u64_2, u64_3]

    let shuffled_low = ops.shuffle_bytes(
        ops.create_vector_from_u64_pair(values[1], values[0], false),
        mask.to_128i_low(),
    );
    let shuffled_high = ops.shuffle_bytes(
        ops.create_vector_from_u64_pair(values[3], values[2], false),
        mask.to_128i_high(),
    );

    // Extract the shuffled u64 values
    let shuffled_values_low = ops.extract_u64s(shuffled_low);
    let shuffled_values_high = ops.extract_u64s(shuffled_high);

    // Recombine in the correct order
    Simd256::new(
        shuffled_values_low[0],  // u64_0
        shuffled_values_low[1],  // u64_1
        shuffled_values_high[0], // u64_2
        shuffled_values_high[1], // u64_3
    )
}

// Delegate all ArchOps methods to the inner X86Ops instance
impl ArchOps for VpclmulqdqOps {
    type Vector = __m128i;

    #[inline]
    #[target_feature(enable = "avx2,vpclmulqdq,sse2,sse4.1,pclmulqdq")]
    unsafe fn process_enhanced_simd_blocks<W: EnhancedCrcWidth>(
        &self,
        state: &mut CrcState<Self::Vector>,
        first: &[Self::Vector; 8],
        rest: &[[Self::Vector; 8]],
        _reflector: &Reflector<Self::Vector>,
        keys: [u64; 21],
    ) -> bool
    where
        Self::Vector: Copy,
    {
        // Update the state with the result
        *state = W::create_state(
            self.process_vpclmulqdq_blocks::<W>(state, first, rest, keys, state.reflected),
            state.reflected,
            self,
        );

        // Return true to indicate we handled it
        true
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn create_vector_from_u64_pair(
        &self,
        high: u64,
        low: u64,
        reflected: bool,
    ) -> Self::Vector {
        self.0.create_vector_from_u64_pair(high, low, reflected)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn create_vector_from_u64_pair_non_reflected(
        &self,
        high: u64,
        low: u64,
    ) -> Self::Vector {
        self.0.create_vector_from_u64_pair_non_reflected(high, low)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn create_vector_from_u64(&self, value: u64, high: bool) -> Self::Vector {
        self.0.create_vector_from_u64(value, high)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn extract_u64s(&self, vector: Self::Vector) -> [u64; 2] {
        self.0.extract_u64s(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn extract_poly64s(&self, vector: Self::Vector) -> [u64; 2] {
        self.0.extract_poly64s(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn xor_vectors(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.xor_vectors(a, b)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn load_bytes(&self, ptr: *const u8) -> Self::Vector {
        self.0.load_bytes(ptr)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn load_aligned(&self, ptr: *const [u64; 2]) -> Self::Vector {
        self.0.load_aligned(ptr)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn shuffle_bytes(&self, data: Self::Vector, mask: Self::Vector) -> Self::Vector {
        self.0.shuffle_bytes(data, mask)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn blend_vectors(
        &self,
        a: Self::Vector,
        b: Self::Vector,
        mask: Self::Vector,
    ) -> Self::Vector {
        self.0.blend_vectors(a, b, mask)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn shift_left_8(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_left_8(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn set_all_bytes(&self, value: u8) -> Self::Vector {
        self.0.set_all_bytes(value)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn create_compare_mask(&self, vector: Self::Vector) -> Self::Vector {
        self.0.create_compare_mask(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn and_vectors(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.and_vectors(a, b)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn shift_right_32(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_32(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn shift_left_32(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_left_32(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn create_vector_from_u32(&self, value: u32, high: bool) -> Self::Vector {
        self.0.create_vector_from_u32(value, high)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn shift_left_4(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_left_4(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn shift_right_4(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_4(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn shift_right_8(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_8(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn shift_right_5(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_5(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn shift_right_6(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_6(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn shift_right_7(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_7(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn shift_right_12(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_right_12(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1")]
    unsafe fn shift_left_12(&self, vector: Self::Vector) -> Self::Vector {
        self.0.shift_left_12(vector)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1,pclmulqdq")]
    unsafe fn carryless_mul_00(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.carryless_mul_00(a, b)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1,pclmulqdq")]
    unsafe fn carryless_mul_01(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.carryless_mul_01(a, b)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1,pclmulqdq")]
    unsafe fn carryless_mul_10(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.carryless_mul_10(a, b)
    }

    #[inline]
    #[target_feature(enable = "sse2,sse4.1,pclmulqdq")]
    unsafe fn carryless_mul_11(&self, a: Self::Vector, b: Self::Vector) -> Self::Vector {
        self.0.carryless_mul_11(a, b)
    }
}
