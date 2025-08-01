// Copyright 2022 the Raw Resource Handle Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::Blob;

/// Defines the pixel format of a [raw image](RawImageData).
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum ImageFormat {
    /// 32-bit RGBA with 8-bit channels.
    Rgba8 = 0,
    // NOTICE: If a new value is added, be sure to update the bytemuck CheckedBitPattern impl.
}

impl ImageFormat {
    /// Returns the required size in bytes for an image in this format
    /// of the given dimensions.
    ///
    /// A result of `None` indicates an overflow in the size calculation.
    #[must_use]
    pub fn size_in_bytes(self, width: u32, height: u32) -> Option<usize> {
        match self {
            Self::Rgba8 => 4_usize
                .checked_mul(width as usize)
                .and_then(|x| x.checked_mul(height as usize)),
        }
    }
}

/// Owned shareable image resource.
#[derive(Clone, PartialEq, Debug)]
pub struct RawImageData {
    /// Blob containing the image data.
    pub data: Blob<u8>,
    /// Pixel format of the image.
    pub format: ImageFormat,
    /// Width of the image.
    pub width: u32,
    /// Height of the image.
    pub height: u32,
}

impl RawImageData {
    /// Creates a new image with the given data, [format](ImageFormat) and dimensions.
    #[must_use]
    pub fn new(data: Blob<u8>, format: ImageFormat, width: u32, height: u32) -> Self {
        Self {
            data,
            format,
            width,
            height,
        }
    }
}
