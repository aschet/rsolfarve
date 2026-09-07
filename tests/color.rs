// SPDX-FileCopyrightText: 2026 Thomas Ascher <thomas.ascher@gmx.at>
//
// SPDX-License-Identifier: MIT

//! Black-box tests against olfarve's public API.
//!
//! Private items (the spectrum table, the gamma encoder) are unit tested
//! from inside `src/color.rs` instead; see the module docs there.

use olfarve::{
    DEFAULT_PATH_LENGTH_CM, Error, SrgbColor, absorption_to_srgb, ebc_to_srgb, srm_to_srgb,
};

const SRM_REFERENCE: &[(f64, &str)] = &[
    (1.0, "#fae8b6"),
    (2.0, "#f4d180"),
    (4.0, "#e7aa31"),
    (10.0, "#ba5b00"),
    (20.0, "#7d1900"),
    (30.0, "#540000"),
    (40.0, "#390000"),
    (50.0, "#270000"),
];

#[test]
fn normalization_factor_scales_white_to_one() {
    // An unabsorbing sample renders as white.
    let color = absorption_to_srgb(0.0, DEFAULT_PATH_LENGTH_CM).unwrap();
    assert!((color.r - 1.0).abs() < 1e-4);
    assert!((color.g - 1.0).abs() < 1e-4);
    assert!((color.b - 1.0).abs() < 1e-4);
    assert_eq!(color.to_hex(), "#ffffff");
}

#[test]
fn srm_reference_colors() {
    for &(srm, expected) in SRM_REFERENCE {
        assert_eq!(
            srm_to_srgb(srm, DEFAULT_PATH_LENGTH_CM).unwrap().to_hex(),
            expected,
            "srm = {srm}"
        );
    }
}

#[test]
fn ebc_matches_equivalent_srm() {
    // EBC and SRM are the same scale up to the 25.0 / 12.7 factor.
    for srm in [1.0, 5.0, 10.0, 25.0, 40.0] {
        let ebc = srm * 25.0 / 12.7;
        let from_ebc = ebc_to_srgb(ebc, DEFAULT_PATH_LENGTH_CM).unwrap();
        let from_srm = srm_to_srgb(srm, DEFAULT_PATH_LENGTH_CM).unwrap();
        assert!((from_ebc.r - from_srm.r).abs() < 1e-9);
        assert!((from_ebc.g - from_srm.g).abs() < 1e-9);
        assert!((from_ebc.b - from_srm.b).abs() < 1e-9);
    }
}

#[test]
fn components_are_within_unit_range() {
    for srm in 0..=60 {
        let color = srm_to_srgb(f64::from(srm), DEFAULT_PATH_LENGTH_CM).unwrap();
        for component in [color.r, color.g, color.b] {
            assert!((0.0..=1.0).contains(&component));
        }
    }
}

#[test]
fn color_darkens_monotonically_with_color_value() {
    let mut previous = f64::INFINITY;
    for srm in 0..=40 {
        let color = srm_to_srgb(f64::from(srm), DEFAULT_PATH_LENGTH_CM).unwrap();
        let luminance = color.r + color.g + color.b;
        assert!(luminance < previous);
        previous = luminance;
    }
}

#[test]
fn longer_path_length_darkens_color() {
    let short = srm_to_srgb(10.0, 1.0).unwrap();
    let long = srm_to_srgb(10.0, 10.0).unwrap();
    assert!(long.r + long.g + long.b < short.r + short.g + short.b);
}

#[test]
fn zero_path_length_is_white() {
    assert_eq!(srm_to_srgb(20.0, 0.0).unwrap().to_hex(), "#ffffff");
}

// DEFAULT_PATH_LENGTH_CM is an exact literal, not a computed value.
#[test]
#[allow(clippy::float_cmp)]
fn default_path_length_matches_bjcp_glass_width() {
    assert_eq!(DEFAULT_PATH_LENGTH_CM, 5.0);
    assert_eq!(
        srm_to_srgb(10.0, 5.0).unwrap(),
        srm_to_srgb(10.0, DEFAULT_PATH_LENGTH_CM).unwrap()
    );
}

#[test]
fn negative_absorption_430_is_rejected() {
    assert_eq!(
        absorption_to_srgb(-0.1, DEFAULT_PATH_LENGTH_CM),
        Err(Error::NegativeAbsorption430(-0.1))
    );
}

#[test]
fn negative_path_length_is_rejected() {
    assert_eq!(
        absorption_to_srgb(1.0, -1.0),
        Err(Error::NegativePathLength(-1.0))
    );
    assert_eq!(srm_to_srgb(1.0, -1.0), Err(Error::NegativePathLength(-1.0)));
    assert_eq!(ebc_to_srgb(1.0, -1.0), Err(Error::NegativePathLength(-1.0)));
}

#[test]
fn negative_color_value_is_rejected() {
    assert!(srm_to_srgb(-1.0, DEFAULT_PATH_LENGTH_CM).is_err());
    assert!(ebc_to_srgb(-1.0, DEFAULT_PATH_LENGTH_CM).is_err());
}

#[test]
fn error_messages_report_the_offending_value() {
    let error = absorption_to_srgb(-0.1, DEFAULT_PATH_LENGTH_CM).unwrap_err();
    assert_eq!(
        error.to_string(),
        "absorption_430 must not be negative, got -0.1"
    );

    let error = absorption_to_srgb(1.0, -1.0).unwrap_err();
    assert_eq!(
        error.to_string(),
        "path_length_cm must not be negative, got -1"
    );
}

#[test]
fn to_rgb8_matches_reference() {
    let color = SrgbColor {
        r: 1.0,
        g: 0.5,
        b: 0.0,
    };
    assert_eq!(color.to_rgb8(), (255, 128, 0));
}

#[test]
fn converts_into_f64_tuple() {
    let color = SrgbColor {
        r: 1.0,
        g: 0.5,
        b: 0.0,
    };
    let tuple: (f64, f64, f64) = color.into();
    assert_eq!(tuple, (1.0, 0.5, 0.0));
}

#[test]
fn converts_into_u8_tuple() {
    let color = SrgbColor {
        r: 1.0,
        g: 0.5,
        b: 0.0,
    };
    let tuple: (u8, u8, u8) = color.into();
    assert_eq!(tuple, color.to_rgb8());
}

mod hex_output {
    use super::{DEFAULT_PATH_LENGTH_CM, SrgbColor, srm_to_srgb};

    #[test]
    fn endpoints() {
        assert_eq!(
            SrgbColor {
                r: 1.0,
                g: 1.0,
                b: 1.0
            }
            .to_hex(),
            "#ffffff"
        );
        assert_eq!(
            SrgbColor {
                r: 0.0,
                g: 0.0,
                b: 0.0
            }
            .to_hex(),
            "#000000"
        );
        assert_eq!(
            SrgbColor {
                r: 1.0,
                g: 0.0,
                b: 0.0
            }
            .to_hex(),
            "#ff0000"
        );
    }

    #[test]
    fn output_is_lowercase_and_padded() {
        let text = SrgbColor {
            r: 0.04,
            g: 0.04,
            b: 0.04,
        }
        .to_hex();
        assert_eq!(text, text.to_lowercase());
        assert_eq!(text.len(), 7);
    }

    #[test]
    fn agrees_with_to_rgb8() {
        // to_hex is derived from to_rgb8, so the two must never disagree.
        for srm in 0..=60 {
            let color = srm_to_srgb(f64::from(srm), DEFAULT_PATH_LENGTH_CM).unwrap();
            let (r, g, b) = color.to_rgb8();
            assert_eq!(color.to_hex(), format!("#{r:02x}{g:02x}{b:02x}"));
        }
    }

    #[test]
    fn out_of_gamut_components_are_clamped() {
        // A hand built out of range color must still yield 7 characters.
        let cases = [
            (
                SrgbColor {
                    r: 2.0,
                    g: 0.0,
                    b: 0.0,
                },
                "#ff0000",
            ),
            (
                SrgbColor {
                    r: -1.0,
                    g: -1.0,
                    b: -1.0,
                },
                "#000000",
            ),
            (
                SrgbColor {
                    r: 1.5,
                    g: 0.0,
                    b: 0.0,
                },
                "#ff0000",
            ),
        ];
        for (color, expected) in cases {
            // to_rgb8 returns u8, so range is already guaranteed by the type;
            // only the exact digits need checking here.
            let hex = color.to_hex();
            assert_eq!(hex.len(), 7);
            assert_eq!(hex, expected);
        }
    }
}
