// Copyright 2025 the Raw Resource Handle Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(unsafe_code)] // reason = "unsafe is required for bytemuck unsafe impls"

use crate::ImageFormat;

// Safety: The enum is `repr(u8)` and has only fieldless variants.
unsafe impl bytemuck::NoUninit for ImageFormat {}

// Safety: The enum is `repr(u8)` and `0` is a valid value.
unsafe impl bytemuck::Zeroable for ImageFormat {}

// Safety: The enum is `repr(u8)`.
unsafe impl bytemuck::checked::CheckedBitPattern for ImageFormat {
    type Bits = u8;

    fn is_valid_bit_pattern(bits: &u8) -> bool {
        #![allow(clippy::absurd_extreme_comparisons)] // reason = "There is only one value."
        use bytemuck::Contiguous;
        // Don't need to compare against MIN_VALUE as this is u8 and 0 is the MIN_VALUE.
        *bits <= Self::MAX_VALUE
    }
}

// Safety: The enum is `repr(u8)`. All values are `u8` and fall within
// the min and max values.
unsafe impl bytemuck::Contiguous for ImageFormat {
    type Int = u8;
    const MIN_VALUE: u8 = Self::Rgba8 as u8;
    const MAX_VALUE: u8 = Self::Rgba8 as u8;
}
