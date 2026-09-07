// SPDX-FileCopyrightText: 2026 Thomas Ascher <thomas.ascher@gmx.at>
//
// SPDX-License-Identifier: MIT

//! sRGB rendering of SRM and EBC beer color values.
//!
//! *Øl farve* ("beer color") renders SRM and EBC beer color values as sRGB
//! colors, following the spectral model described by A. J. de Lange,
//! "Color," in *Brewing Materials and Processes*, Elsevier, 2016,
//! pp. 199-249.
//!
//! Given a color value and an optical path length (the width of the glass
//! the beer is viewed through), the sample's spectral transmittance is
//! derived from its absorption coefficient at 430 nm via the Beer-Lambert
//! law, integrated against the CIE 1931 color matching functions of the 2
//! degree standard colorimetric observer under illuminant D65, and the
//! resulting XYZ tristimulus values are transformed to sRGB.
//!
//! # Examples
//!
//! ```
//! use olfarve::{DEFAULT_PATH_LENGTH_CM, absorption_to_srgb, ebc_to_srgb, srm_to_srgb};
//!
//! srm_to_srgb(10.0, DEFAULT_PATH_LENGTH_CM)?.to_hex();
//! ebc_to_srgb(20.0, DEFAULT_PATH_LENGTH_CM)?.to_hex();
//!
//! // The default path length is 5 cm, the width of a typical sample glass
//! srm_to_srgb(10.0, 1.0)?.to_hex();
//!
//! // Results are SrgbColor structs of gamma encoded components in [0, 1]
//! let color = srm_to_srgb(10.0, DEFAULT_PATH_LENGTH_CM)?;
//! let (r, g, b) = (color.r, color.g, color.b);
//! color.to_rgb8();
//!
//! // Or start from an absorbance measured at 430 nm
//! absorption_to_srgb(0.7874, DEFAULT_PATH_LENGTH_CM)?;
//! # Ok::<(), olfarve::Error>(())
//! ```
//!
//! The path length is always explicit; pass [`DEFAULT_PATH_LENGTH_CM`] for
//! the typical case.

mod cie;
mod color;

pub use color::{
    DEFAULT_PATH_LENGTH_CM, Error, SrgbColor, absorption_to_srgb, ebc_to_srgb, srm_to_srgb,
};
