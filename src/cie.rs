// SPDX-FileCopyrightText: 2026 Thomas Ascher <thomas.ascher@gmx.at>
//
// SPDX-License-Identifier: MIT

//! Reference colorimetric data used by [`crate::color`].
//!
//! Two CIE datasets tabulated together from 380 nm to 780 nm in 5 nm steps:
//!
//! * Color matching functions of the CIE 1931 2 degree standard colorimetric
//!   observer, standardized as ISO/CIE 11664-1:2019. Values from
//!   <https://cie.co.at/datatable/cie-1931-colour-matching-functions-2-degree-observer>.
//! * Relative spectral power distribution of CIE standard illuminant D65,
//!   standardized as ISO/CIE 11664-2:2022. Values from
//!   <https://cie.co.at/datatable/cie-standard-illuminant-d65>.
//!
//! This module is private; its contents may change without notice.

/// Wavelength of the first sample in [`CIE_SAMPLES`], in nanometers.
pub(crate) const FIRST_WAVELENGTH_NM: f64 = 380.0;

/// Distance between two consecutive samples in [`CIE_SAMPLES`], in nanometers.
pub(crate) const WAVELENGTH_STEP_NM: f64 = 5.0;

/// One wavelength sample of the CIE 1931 observer and the D65 illuminant.
///
/// Field names follow CIE notation: `x_bar`, `y_bar` and `z_bar` are the
/// color matching functions x(lambda), y(lambda) and z(lambda); `s_d65` is
/// the relative spectral power distribution S(lambda) of illuminant D65.
pub(crate) struct CieSample {
    pub(crate) x_bar: f64,
    pub(crate) y_bar: f64,
    pub(crate) z_bar: f64,
    pub(crate) s_d65: f64,
}

/// Colorimetric samples from 380 nm to 780 nm in 5 nm increments.
///
/// Values are kept exactly as published in the CIE datatables cited above,
/// without digit-group separators, so a reader can diff a row against the
/// source directly.
#[rustfmt::skip]
#[allow(clippy::unreadable_literal)]
pub(crate) const CIE_SAMPLES: [CieSample; 81] = [
    CieSample { x_bar: 0.001368, y_bar: 0.000039, z_bar: 0.006450, s_d65: 49.9755 },
    CieSample { x_bar: 0.002236, y_bar: 0.000064, z_bar: 0.010550, s_d65: 52.3118 },
    CieSample { x_bar: 0.004243, y_bar: 0.000120, z_bar: 0.020050, s_d65: 54.6482 },
    CieSample { x_bar: 0.007650, y_bar: 0.000217, z_bar: 0.036210, s_d65: 68.7015 },
    CieSample { x_bar: 0.014310, y_bar: 0.000396, z_bar: 0.067850, s_d65: 82.7549 },
    CieSample { x_bar: 0.023190, y_bar: 0.000640, z_bar: 0.110200, s_d65: 87.1204 },
    CieSample { x_bar: 0.043510, y_bar: 0.001210, z_bar: 0.207400, s_d65: 91.486 },
    CieSample { x_bar: 0.077630, y_bar: 0.002180, z_bar: 0.371300, s_d65: 92.4589 },
    CieSample { x_bar: 0.134380, y_bar: 0.004000, z_bar: 0.645600, s_d65: 93.4318 },
    CieSample { x_bar: 0.214770, y_bar: 0.007300, z_bar: 1.039050, s_d65: 90.057 },
    CieSample { x_bar: 0.283900, y_bar: 0.011600, z_bar: 1.385600, s_d65: 86.6823 },
    CieSample { x_bar: 0.328500, y_bar: 0.016840, z_bar: 1.622960, s_d65: 95.7736 },
    CieSample { x_bar: 0.348280, y_bar: 0.023000, z_bar: 1.747060, s_d65: 104.865 },
    CieSample { x_bar: 0.348060, y_bar: 0.029800, z_bar: 1.782600, s_d65: 110.936 },
    CieSample { x_bar: 0.336200, y_bar: 0.038000, z_bar: 1.772110, s_d65: 117.008 },
    CieSample { x_bar: 0.318700, y_bar: 0.048000, z_bar: 1.744100, s_d65: 117.41 },
    CieSample { x_bar: 0.290800, y_bar: 0.060000, z_bar: 1.669200, s_d65: 117.812 },
    CieSample { x_bar: 0.251100, y_bar: 0.073900, z_bar: 1.528100, s_d65: 116.336 },
    CieSample { x_bar: 0.195360, y_bar: 0.090980, z_bar: 1.287640, s_d65: 114.861 },
    CieSample { x_bar: 0.142100, y_bar: 0.112600, z_bar: 1.041900, s_d65: 115.392 },
    CieSample { x_bar: 0.095640, y_bar: 0.139020, z_bar: 0.812950, s_d65: 115.923 },
    CieSample { x_bar: 0.057950, y_bar: 0.169300, z_bar: 0.616200, s_d65: 112.367 },
    CieSample { x_bar: 0.032010, y_bar: 0.208020, z_bar: 0.465180, s_d65: 108.811 },
    CieSample { x_bar: 0.014700, y_bar: 0.258600, z_bar: 0.353300, s_d65: 109.082 },
    CieSample { x_bar: 0.004900, y_bar: 0.323000, z_bar: 0.272000, s_d65: 109.354 },
    CieSample { x_bar: 0.002400, y_bar: 0.407300, z_bar: 0.212300, s_d65: 108.578 },
    CieSample { x_bar: 0.009300, y_bar: 0.503000, z_bar: 0.158200, s_d65: 107.802 },
    CieSample { x_bar: 0.029100, y_bar: 0.608200, z_bar: 0.111700, s_d65: 106.296 },
    CieSample { x_bar: 0.063270, y_bar: 0.710000, z_bar: 0.078250, s_d65: 104.79 },
    CieSample { x_bar: 0.109600, y_bar: 0.793200, z_bar: 0.057250, s_d65: 106.239 },
    CieSample { x_bar: 0.165500, y_bar: 0.862000, z_bar: 0.042160, s_d65: 107.689 },
    CieSample { x_bar: 0.225750, y_bar: 0.914850, z_bar: 0.029840, s_d65: 106.047 },
    CieSample { x_bar: 0.290400, y_bar: 0.954000, z_bar: 0.020300, s_d65: 104.405 },
    CieSample { x_bar: 0.359700, y_bar: 0.980300, z_bar: 0.013400, s_d65: 104.225 },
    CieSample { x_bar: 0.433450, y_bar: 0.994950, z_bar: 0.008750, s_d65: 104.046 },
    CieSample { x_bar: 0.512050, y_bar: 1.000000, z_bar: 0.005750, s_d65: 102.023 },
    CieSample { x_bar: 0.594500, y_bar: 0.995000, z_bar: 0.003900, s_d65: 100.0 },
    CieSample { x_bar: 0.678400, y_bar: 0.978600, z_bar: 0.002750, s_d65: 98.1671 },
    CieSample { x_bar: 0.762100, y_bar: 0.952000, z_bar: 0.002100, s_d65: 96.3342 },
    CieSample { x_bar: 0.842500, y_bar: 0.915400, z_bar: 0.001800, s_d65: 96.0611 },
    CieSample { x_bar: 0.916300, y_bar: 0.870000, z_bar: 0.001650, s_d65: 95.788 },
    CieSample { x_bar: 0.978600, y_bar: 0.816300, z_bar: 0.001400, s_d65: 92.2368 },
    CieSample { x_bar: 1.026300, y_bar: 0.757000, z_bar: 0.001100, s_d65: 88.6856 },
    CieSample { x_bar: 1.056700, y_bar: 0.694900, z_bar: 0.001000, s_d65: 89.3459 },
    CieSample { x_bar: 1.062200, y_bar: 0.631000, z_bar: 0.000800, s_d65: 90.0062 },
    CieSample { x_bar: 1.045600, y_bar: 0.566800, z_bar: 0.000600, s_d65: 89.8026 },
    CieSample { x_bar: 1.002600, y_bar: 0.503000, z_bar: 0.000340, s_d65: 89.5991 },
    CieSample { x_bar: 0.938400, y_bar: 0.441200, z_bar: 0.000240, s_d65: 88.6489 },
    CieSample { x_bar: 0.854450, y_bar: 0.381000, z_bar: 0.000190, s_d65: 87.69871 },
    CieSample { x_bar: 0.751400, y_bar: 0.321000, z_bar: 0.000100, s_d65: 85.4936 },
    CieSample { x_bar: 0.642400, y_bar: 0.265000, z_bar: 0.000050, s_d65: 83.2886 },
    CieSample { x_bar: 0.541900, y_bar: 0.217000, z_bar: 0.000030, s_d65: 83.4939 },
    CieSample { x_bar: 0.447900, y_bar: 0.175000, z_bar: 0.000020, s_d65: 83.6992 },
    CieSample { x_bar: 0.360800, y_bar: 0.138200, z_bar: 0.000010, s_d65: 81.863 },
    CieSample { x_bar: 0.283500, y_bar: 0.107000, z_bar: 0.000000, s_d65: 80.0268 },
    CieSample { x_bar: 0.218700, y_bar: 0.081600, z_bar: 0.000000, s_d65: 80.1207 },
    CieSample { x_bar: 0.164900, y_bar: 0.061000, z_bar: 0.000000, s_d65: 80.2146 },
    CieSample { x_bar: 0.121200, y_bar: 0.044580, z_bar: 0.000000, s_d65: 81.2462 },
    CieSample { x_bar: 0.087400, y_bar: 0.032000, z_bar: 0.000000, s_d65: 82.2778 },
    CieSample { x_bar: 0.063600, y_bar: 0.023200, z_bar: 0.000000, s_d65: 80.281 },
    CieSample { x_bar: 0.046770, y_bar: 0.017000, z_bar: 0.000000, s_d65: 78.2842 },
    CieSample { x_bar: 0.032900, y_bar: 0.011920, z_bar: 0.000000, s_d65: 74.0027 },
    CieSample { x_bar: 0.022700, y_bar: 0.008210, z_bar: 0.000000, s_d65: 69.7213 },
    CieSample { x_bar: 0.015840, y_bar: 0.005723, z_bar: 0.000000, s_d65: 70.6652 },
    CieSample { x_bar: 0.011359, y_bar: 0.004102, z_bar: 0.000000, s_d65: 71.6091 },
    CieSample { x_bar: 0.008111, y_bar: 0.002929, z_bar: 0.000000, s_d65: 72.979 },
    CieSample { x_bar: 0.005790, y_bar: 0.002091, z_bar: 0.000000, s_d65: 74.349 },
    CieSample { x_bar: 0.004109, y_bar: 0.001484, z_bar: 0.000000, s_d65: 67.9765 },
    CieSample { x_bar: 0.002899, y_bar: 0.001047, z_bar: 0.000000, s_d65: 61.604 },
    CieSample { x_bar: 0.002049, y_bar: 0.000740, z_bar: 0.000000, s_d65: 65.7448 },
    CieSample { x_bar: 0.001440, y_bar: 0.000520, z_bar: 0.000000, s_d65: 69.8856 },
    CieSample { x_bar: 0.001000, y_bar: 0.000361, z_bar: 0.000000, s_d65: 72.4863 },
    CieSample { x_bar: 0.000690, y_bar: 0.000249, z_bar: 0.000000, s_d65: 75.087 },
    CieSample { x_bar: 0.000476, y_bar: 0.000172, z_bar: 0.000000, s_d65: 69.3398 },
    CieSample { x_bar: 0.000332, y_bar: 0.000120, z_bar: 0.000000, s_d65: 63.5927 },
    CieSample { x_bar: 0.000235, y_bar: 0.000085, z_bar: 0.000000, s_d65: 55.0054 },
    CieSample { x_bar: 0.000166, y_bar: 0.000060, z_bar: 0.000000, s_d65: 46.4182 },
    CieSample { x_bar: 0.000117, y_bar: 0.000042, z_bar: 0.000000, s_d65: 56.6118 },
    CieSample { x_bar: 0.000083, y_bar: 0.000030, z_bar: 0.000000, s_d65: 66.8054 },
    CieSample { x_bar: 0.000059, y_bar: 0.000021, z_bar: 0.000000, s_d65: 65.0941 },
    CieSample { x_bar: 0.000042, y_bar: 0.000015, z_bar: 0.000000, s_d65: 63.3828 },
];
