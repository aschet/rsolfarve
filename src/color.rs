// SPDX-FileCopyrightText: 2026 Thomas Ascher <thomas.ascher@gmx.at>
//
// SPDX-License-Identifier: MIT

//! sRGB rendering of SRM and EBC beer color values.
//!
//! The spectral model is A. J. de Lange, "Color," in *Brewing Materials and
//! Processes*, Elsevier, 2016, pp. 199-249: beer's transmittance across the
//! visible range is approximated from its absorption at 430 nm. Integrating
//! that against the CIE 1931 color matching functions under illuminant D65
//! gives XYZ tristimulus values, which are then transformed to sRGB.
//!
//! The sRGB primaries, white point and gamma encoding follow
//! <https://www.w3.org/Graphics/Color/srgb>. The colorimetric data is
//! documented in [`crate::cie`].

use std::fmt;
use std::sync::LazyLock;

use crate::cie::{CIE_SAMPLES, FIRST_WAVELENGTH_NM, WAVELENGTH_STEP_NM};

/// Default optical path length in cm, set to the typical sample glass width
/// specified by the BJCP color guide.
///
/// <https://www.bjcp.org/education-training/education-resources/color-guide>
pub const DEFAULT_PATH_LENGTH_CM: f64 = 5.0;

// Both scales are defined as a multiple of the absorbance at 430 nm measured
// over a 1 cm path: SRM = 12.7 * A430 and EBC = 25.0 * A430.
const SRM_PER_ABSORBANCE: f64 = 12.7;
const EBC_PER_ABSORBANCE: f64 = 25.0;

// The de Lange approximation sums two exponentials decaying away from 430
// nm, giving absorption at any wavelength relative to the absorption there.
const REFERENCE_WAVELENGTH_NM: f64 = 430.0;
const SHORT_DECAY_WEIGHT: f64 = 0.02465;
const SHORT_DECAY_NM: f64 = 17.591;
const LONG_DECAY_WEIGHT: f64 = 0.97535;
const LONG_DECAY_NM: f64 = 82.122;

// Piecewise sRGB gamma encoding: linear below the threshold, a power law
// above it. See https://www.w3.org/Graphics/Color/srgb.
const GAMMA_THRESHOLD: f64 = 0.003_130_8;
const GAMMA_SLOPE: f64 = 12.92;
const GAMMA_SCALE: f64 = 1.055;
const GAMMA_OFFSET: f64 = 0.055;
const GAMMA_EXPONENT: f64 = 1.0 / 2.4;

/// Precomputed wavelength dependent terms of the integration.
///
/// Only the absorbance varies between conversions. The absorption ratios and
/// the colorimetric weights depend solely on wavelength, so they are
/// evaluated once, on first use, rather than on every call.
struct SpectrumEntry {
    absorption_ratio: f64,
    s_d65: f64,
    x_bar: f64,
    y_bar: f64,
    z_bar: f64,
}

/// The precomputed spectrum table together with the D65 normalizing constant.
///
/// A fixed-size array, not a `Vec`: [`CIE_SAMPLES`] has a compile-time known
/// length, so there is no reason to pay for a heap allocation here.
struct Spectrum {
    entries: [SpectrumEntry; CIE_SAMPLES.len()],
    k: f64,
}

static SPECTRUM: LazyLock<Spectrum> = LazyLock::new(build_spectrum);

/// Returns absorption at `wavelength_nm` relative to that at 430 nm.
fn absorption_ratio(wavelength_nm: f64) -> f64 {
    let offset_nm = wavelength_nm - REFERENCE_WAVELENGTH_NM;
    // Deliberately `a * b + c * d`, not `mul_add`: fused multiply-add
    // rounds once instead of twice, which would shift results away from
    // the plain double-precision arithmetic the Python and C# ports use
    // and break their bit-identical reference hex values.
    SHORT_DECAY_WEIGHT * (-offset_nm / SHORT_DECAY_NM).exp()
        + LONG_DECAY_WEIGHT * (-offset_nm / LONG_DECAY_NM).exp()
}

/// Builds the spectrum table and, from it, the D65 normalizing constant.
///
/// CIE defines `k = 100 / sum(S(lambda) * y_bar(lambda))`, putting the
/// luminance of a perfectly transmitting sample at 100. Dropping the factor
/// of 100 puts it at 1.0 instead, which is the range sRGB expects.
// `i` never exceeds CIE_SAMPLES.len() (81), which converts to f64 exactly.
#[allow(clippy::cast_precision_loss)]
fn build_spectrum() -> Spectrum {
    let entries = std::array::from_fn(|i| {
        let sample = &CIE_SAMPLES[i];
        SpectrumEntry {
            absorption_ratio: absorption_ratio(FIRST_WAVELENGTH_NM + i as f64 * WAVELENGTH_STEP_NM),
            s_d65: sample.s_d65,
            x_bar: sample.x_bar,
            y_bar: sample.y_bar,
            z_bar: sample.z_bar,
        }
    });
    let luminance: f64 = CIE_SAMPLES
        .iter()
        .map(|sample| sample.s_d65 * sample.y_bar)
        .sum();
    Spectrum {
        entries,
        k: 1.0 / luminance,
    }
}

/// Quantizes one gamma encoded component to an integer in `[0, 255]`.
// The clamp guarantees `0.0..=255.0` before the cast, so truncation and
// sign loss are both unreachable.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn to_8bit(component: f64) -> u8 {
    (component.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// Gamma encodes one linear component, clamping it to `[0, 1]` first.
///
/// This is the inverse of the sRGB EOTF: it maps a linear tristimulus
/// component to the non-linear signal a display decodes.
fn encode_gamma(linear: f64) -> f64 {
    let linear = linear.clamp(0.0, 1.0);
    if linear <= GAMMA_THRESHOLD {
        linear * GAMMA_SLOPE
    } else {
        // Deliberately not `mul_add`; see absorption_ratio above.
        GAMMA_SCALE * linear.powf(GAMMA_EXPONENT) - GAMMA_OFFSET
    }
}

/// The error returned when a beer color conversion is given a negative
/// input.
///
/// Every conversion in this module takes only physical, non-negative
/// quantities (an absorption coefficient, a color value, an optical path
/// length), so a negative argument can never be converted and is rejected
/// rather than silently clamped.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// The absorption coefficient at 430 nm (`absorption_430`) was negative.
    NegativeAbsorption430(f64),
    /// The optical path length (`path_length_cm`) was negative.
    NegativePathLength(f64),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NegativeAbsorption430(value) => {
                write!(f, "absorption_430 must not be negative, got {value}")
            }
            Self::NegativePathLength(value) => {
                write!(f, "path_length_cm must not be negative, got {value}")
            }
        }
    }
}

impl std::error::Error for Error {}

/// An sRGB color, gamma encoded, with components in `[0, 1]`.
///
/// A plain, [`Copy`] triplet: it destructures like one with
/// `let SrgbColor { r, g, b } = color;`, and [`SrgbColor::to_rgb8`] and the
/// [`Display`](fmt::Display) implementation (`"{color}"`) give the two most
/// common lossy views. The [`Default`] instance is black.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct SrgbColor {
    /// The gamma encoded red component.
    pub r: f64,
    /// The gamma encoded green component.
    pub g: f64,
    /// The gamma encoded blue component.
    pub b: f64,
}

impl SrgbColor {
    /// Returns the color quantized to 8 bits per channel.
    ///
    /// Components are clamped into gamut first, so the result is a valid 8
    /// bit triplet even for an instance built by hand out of range.
    ///
    /// # Examples
    ///
    /// ```
    /// use olfarve::SrgbColor;
    ///
    /// assert_eq!(SrgbColor { r: 1.0, g: 0.5, b: 0.0 }.to_rgb8(), (255, 128, 0));
    /// assert_eq!(SrgbColor { r: 2.0, g: -1.0, b: 0.0 }.to_rgb8(), (255, 0, 0));
    /// ```
    #[must_use]
    pub fn to_rgb8(self) -> (u8, u8, u8) {
        (to_8bit(self.r), to_8bit(self.g), to_8bit(self.b))
    }

    /// Returns the color as a `#rrggbb` string.
    ///
    /// # Examples
    ///
    /// ```
    /// use olfarve::SrgbColor;
    ///
    /// assert_eq!(SrgbColor { r: 1.0, g: 0.5, b: 0.0 }.to_hex(), "#ff8000");
    /// assert_eq!(SrgbColor { r: 2.0, g: -1.0, b: 0.0 }.to_hex(), "#ff0000");
    /// ```
    #[must_use]
    pub fn to_hex(self) -> String {
        self.to_string()
    }
}

impl fmt::Display for SrgbColor {
    /// Formats the color as a `#rrggbb` string; equivalent to
    /// [`SrgbColor::to_hex`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = self.to_rgb8();
        write!(f, "#{r:02x}{g:02x}{b:02x}")
    }
}

impl From<SrgbColor> for (f64, f64, f64) {
    fn from(color: SrgbColor) -> Self {
        (color.r, color.g, color.b)
    }
}

impl From<SrgbColor> for (u8, u8, u8) {
    fn from(color: SrgbColor) -> Self {
        color.to_rgb8()
    }
}

/// Converts a beer's absorption at 430 nm into an sRGB color.
///
/// Prefer [`srm_to_srgb`] or [`ebc_to_srgb`] when you have a color value,
/// which is what brewing software reports. This function is for a
/// photometer reading taken directly, where the absorbance is the
/// measurement and the SRM or EBC value is derived from it.
///
/// # Errors
///
/// Returns [`Error::NegativeAbsorption430`] if `absorption_430` is negative,
/// or [`Error::NegativePathLength`] if `path_length_cm` is negative.
///
/// # Examples
///
/// ```
/// use olfarve::{absorption_to_srgb, DEFAULT_PATH_LENGTH_CM};
///
/// let color = absorption_to_srgb(10.0 / 12.7, DEFAULT_PATH_LENGTH_CM)?;
/// assert_eq!(color.to_hex(), "#ba5b00");
/// # Ok::<(), olfarve::Error>(())
/// ```
///
/// # Parameters
///
/// * `absorption_430` - Linear decadic absorption coefficient at 430 nm, in
///   cm^-1. Numerically this is the ASBC/EBC absorbance A430, which is
///   defined for a 1 cm path length.
/// * `path_length_cm` - Optical path length in cm, e.g. the glass width.
///   [`DEFAULT_PATH_LENGTH_CM`] is a sensible default.
pub fn absorption_to_srgb(absorption_430: f64, path_length_cm: f64) -> Result<SrgbColor, Error> {
    if absorption_430 < 0.0 {
        return Err(Error::NegativeAbsorption430(absorption_430));
    }
    if path_length_cm < 0.0 {
        return Err(Error::NegativePathLength(path_length_cm));
    }

    // Beer-Lambert law: absorbance A = a * l, and transmittance T = 10 ** -A.
    let absorbance_430 = absorption_430 * path_length_cm;

    let spectrum = &*SPECTRUM;
    let mut tristimulus_x = 0.0;
    let mut tristimulus_y = 0.0;
    let mut tristimulus_z = 0.0;
    for entry in &spectrum.entries {
        let transmitted_power =
            entry.s_d65 * 10.0_f64.powf(-absorbance_430 * entry.absorption_ratio);
        // Deliberately `+=` with a separate multiply, not `mul_add`; see
        // absorption_ratio's doc comment above.
        tristimulus_x += transmitted_power * entry.x_bar;
        tristimulus_y += transmitted_power * entry.y_bar;
        tristimulus_z += transmitted_power * entry.z_bar;
    }

    tristimulus_x *= spectrum.k;
    tristimulus_y *= spectrum.k;
    tristimulus_z *= spectrum.k;

    // XYZ to linear sRGB, D65 white point.
    Ok(SrgbColor {
        r: encode_gamma(
            tristimulus_x * 3.240_625_5 + tristimulus_y * -1.537_208 + tristimulus_z * -0.498_628_6,
        ),
        g: encode_gamma(
            tristimulus_x * -0.968_930_7
                + tristimulus_y * 1.875_756_1
                + tristimulus_z * 0.041_517_5,
        ),
        b: encode_gamma(
            tristimulus_x * 0.055_710_1
                + tristimulus_y * -0.204_021_1
                + tristimulus_z * 1.056_995_9,
        ),
    })
}

/// Converts a Standard Reference Method color value into an sRGB color.
///
/// # Errors
///
/// Returns [`Error::NegativeAbsorption430`] if `srm` is negative, or
/// [`Error::NegativePathLength`] if `path_length_cm` is negative.
///
/// # Examples
///
/// ```
/// use olfarve::{srm_to_srgb, DEFAULT_PATH_LENGTH_CM};
///
/// let color = srm_to_srgb(10.0, DEFAULT_PATH_LENGTH_CM)?;
/// assert_eq!(color.to_hex(), "#ba5b00");
/// # Ok::<(), olfarve::Error>(())
/// ```
///
/// # Parameters
///
/// * `srm` - The SRM color value.
/// * `path_length_cm` - Optical path length in cm, e.g. the glass width.
///   [`DEFAULT_PATH_LENGTH_CM`] is a sensible default.
#[inline]
pub fn srm_to_srgb(srm: f64, path_length_cm: f64) -> Result<SrgbColor, Error> {
    absorption_to_srgb(srm / SRM_PER_ABSORBANCE, path_length_cm)
}

/// Converts a European Brewery Convention color value into an sRGB color.
///
/// # Errors
///
/// Returns [`Error::NegativeAbsorption430`] if `ebc` is negative, or
/// [`Error::NegativePathLength`] if `path_length_cm` is negative.
///
/// # Examples
///
/// ```
/// use olfarve::{ebc_to_srgb, DEFAULT_PATH_LENGTH_CM};
///
/// let color = ebc_to_srgb(20.0, DEFAULT_PATH_LENGTH_CM)?;
/// assert_eq!(color.to_hex(), "#b95900");
/// # Ok::<(), olfarve::Error>(())
/// ```
///
/// # Parameters
///
/// * `ebc` - The EBC color value.
/// * `path_length_cm` - Optical path length in cm, e.g. the glass width.
///   [`DEFAULT_PATH_LENGTH_CM`] is a sensible default.
#[inline]
pub fn ebc_to_srgb(ebc: f64, path_length_cm: f64) -> Result<SrgbColor, Error> {
    absorption_to_srgb(ebc / EBC_PER_ABSORBANCE, path_length_cm)
}

// Every comparison here is between exact literals or between two
// evaluations of the same deterministic computation, not between values
// that accumulated independent rounding error, so exact equality is the
// correct check.
#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn spectrum_table_matches_the_model() {
        // The precomputed table must equal evaluating the model per
        // wavelength; a drift here would silently change every result.
        assert_eq!(SPECTRUM.entries.len(), CIE_SAMPLES.len());
        let mut wavelength_nm = FIRST_WAVELENGTH_NM;
        for (entry, sample) in SPECTRUM.entries.iter().zip(&CIE_SAMPLES) {
            assert_eq!(entry.absorption_ratio, absorption_ratio(wavelength_nm));
            assert_eq!(entry.s_d65, sample.s_d65);
            assert_eq!(entry.x_bar, sample.x_bar);
            assert_eq!(entry.y_bar, sample.y_bar);
            assert_eq!(entry.z_bar, sample.z_bar);
            wavelength_nm += WAVELENGTH_STEP_NM;
        }
    }

    #[test]
    fn cie_table_shape() {
        assert_eq!(CIE_SAMPLES.len(), 81);
        for sample in &CIE_SAMPLES {
            assert!(sample.x_bar >= 0.0);
            assert!(sample.y_bar >= 0.0);
            assert!(sample.z_bar >= 0.0);
            assert!(sample.s_d65 >= 0.0);
        }
    }

    #[test]
    fn gamma_encoding_clamps_out_of_range_input() {
        assert_eq!(encode_gamma(-1.0), 0.0);
        assert!((encode_gamma(2.0) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn gamma_encoding_is_continuous_at_the_knee() {
        // The two branches meet, up to the rounding of the sRGB constants.
        let below = encode_gamma(GAMMA_THRESHOLD);
        let above = encode_gamma(GAMMA_THRESHOLD + 1e-12);
        assert!((below - above).abs() < 1e-7);
    }

    #[test]
    fn gamma_encoding_endpoints() {
        assert_eq!(encode_gamma(0.0), 0.0);
        assert!((encode_gamma(1.0) - 1.0).abs() < 1e-12);
    }
}
