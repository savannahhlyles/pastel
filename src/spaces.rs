//! The CIE and Ok families of color-space representation types (XYZ, LMS,
//! Lab, OkLab, LCh and OkLCh), along with their conversions to and from the
//! central [`Color`] type.

use std::fmt;

use crate::colorspace::ColorSpace;
use crate::helper::{interpolate, interpolate_angle, mod_positive};
use crate::types::Scalar;
use crate::{Color, Fraction, D65_XN, D65_YN, D65_ZN, RGBA};

#[derive(Debug, Clone, PartialEq)]
pub struct XYZ {
    /// The X tristimulus value.
    pub x: Scalar,
    /// The Y tristimulus value (luminance).
    pub y: Scalar,
    /// The Z tristimulus value.
    pub z: Scalar,
    /// The alpha (opacity) channel in `[0, 1]`.
    pub alpha: Scalar,
}

/// Convert a color into the CIE 1931 XYZ color space.
impl From<&Color> for XYZ {
    fn from(color: &Color) -> Self {
        #![allow(clippy::many_single_char_names)]
        let finv = |c_: f64| {
            if c_ <= 0.04045 {
                c_ / 12.92
            } else {
                Scalar::powf((c_ + 0.055) / 1.055, 2.4)
            }
        };

        let rec = RGBA::from(color);
        let r = finv(rec.r);
        let g = finv(rec.g);
        let b = finv(rec.b);

        let x = 0.4124 * r + 0.3576 * g + 0.1805 * b;
        let y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        let z = 0.0193 * r + 0.1192 * g + 0.9505 * b;

        XYZ {
            x,
            y,
            z,
            alpha: color.alpha,
        }
    }
}

impl fmt::Display for XYZ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "XYZ({x}, {y}, {z})", x = self.x, y = self.y, z = self.z,)
    }
}

/// A color space whose axes correspond to the responsivity spectra of the long-, medium-, and
/// short-wavelength cone cells in the human eye. More info
/// [here](https://en.wikipedia.org/wiki/LMS_color_space).
#[derive(Debug, Clone, PartialEq)]
pub struct LMS {
    /// The long-wavelength (L) cone response.
    pub l: Scalar,
    /// The medium-wavelength (M) cone response.
    pub m: Scalar,
    /// The short-wavelength (S) cone response.
    pub s: Scalar,
    /// The alpha (opacity) channel in `[0, 1]`.
    pub alpha: Scalar,
}

/// Convert a color into the LMS color space.
impl From<&Color> for LMS {
    fn from(color: &Color) -> Self {
        let XYZ { x, y, z, alpha } = XYZ::from(color);
        let l = 0.38971 * x + 0.68898 * y - 0.07868 * z;
        let m = -0.22981 * x + 1.18340 * y + 0.04641 * z;
        let s = 0.00000 * x + 0.00000 * y + 1.00000 * z;

        LMS { l, m, s, alpha }
    }
}

impl fmt::Display for LMS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LMS({l}, {m}, {s})", l = self.l, m = self.m, s = self.s,)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lab {
    /// The lightness component (0 = black, 100 = white).
    pub l: Scalar,
    /// The green-red chromaticity (a*) component.
    pub a: Scalar,
    /// The blue-yellow chromaticity (b*) component.
    pub b: Scalar,
    /// The alpha (opacity) channel in `[0, 1]`.
    pub alpha: Scalar,
}

impl ColorSpace for Lab {
    fn from_color(c: &Color) -> Self {
        c.to_lab()
    }

    fn into_color(self) -> Color {
        Color::from_lab(self.l, self.a, self.b, self.alpha)
    }

    fn mix(&self, other: &Self, fraction: Fraction) -> Self {
        Self {
            l: interpolate(self.l, other.l, fraction),
            a: interpolate(self.a, other.a, fraction),
            b: interpolate(self.b, other.b, fraction),
            alpha: interpolate(self.alpha, other.alpha, fraction),
        }
    }
}

/// Convert a color into the CIELAB color space.
impl From<&Color> for Lab {
    fn from(color: &Color) -> Self {
        let rec = XYZ::from(color);

        let cut = Scalar::powf(6.0 / 29.0, 3.0);
        let f = |t| {
            if t > cut {
                Scalar::powf(t, 1.0 / 3.0)
            } else {
                (1.0 / 3.0) * Scalar::powf(29.0 / 6.0, 2.0) * t + 4.0 / 29.0
            }
        };

        let fy = f(rec.y / D65_YN);

        let l = 116.0 * fy - 16.0;
        let a = 500.0 * (f(rec.x / D65_XN) - fy);
        let b = 200.0 * (fy - f(rec.z / D65_ZN));

        Lab {
            l,
            a,
            b,
            alpha: color.alpha,
        }
    }
}

impl fmt::Display for Lab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lab({l}, {a}, {b})", l = self.l, a = self.a, b = self.b,)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OkLab {
    /// The lightness component.
    pub l: Scalar,
    /// The green-red (a) component.
    pub a: Scalar,
    /// The blue-yellow (b) component.
    pub b: Scalar,
    /// The alpha (opacity) channel in `[0, 1]`.
    pub alpha: Scalar,
}

impl ColorSpace for OkLab {
    fn from_color(c: &Color) -> Self {
        c.to_oklab()
    }

    fn into_color(self) -> Color {
        Color::from_oklab(self.l, self.a, self.b, self.alpha)
    }

    fn mix(&self, other: &Self, fraction: Fraction) -> Self {
        Self {
            l: interpolate(self.l, other.l, fraction),
            a: interpolate(self.a, other.a, fraction),
            b: interpolate(self.b, other.b, fraction),
            alpha: interpolate(self.alpha, other.alpha, fraction),
        }
    }
}

/// Convert a color into the Oklab color space.
impl From<&Color> for OkLab {
    fn from(value: &Color) -> Self {
        let rec = XYZ::from(value);

        // https://bottosson.github.io/posts/oklab/?#converting-from-xyz-to-oklab

        // multiply with M1 and apply non-linearity
        let long =
            (0.8189330101 * rec.x + 0.3618667424 * rec.y + -0.1288597137 * rec.z).powf(1. / 3.);
        let medium =
            (0.0329845436 * rec.x + 0.9293118715 * rec.y + 0.0361456387 * rec.z).powf(1. / 3.);
        let short =
            (0.0482003018 * rec.x + 0.2643662691 * rec.y + 0.6338517070 * rec.z).powf(1. / 3.);

        // multiply with M2
        let l = 0.2104542553 * long + 0.7936177850 * medium + -0.0040720468 * short;
        let a = 1.9779984951 * long + -2.4285922050 * medium + 0.4505937099 * short;
        let b = 0.0259040371 * long + 0.7827717662 * medium + -0.8086757660 * short;

        Self {
            l,
            a,
            b,
            alpha: rec.alpha,
        }
    }
}

impl fmt::Display for OkLab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OkLab({l}, {a}, {b})",
            l = self.l,
            a = self.a,
            b = self.b,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LCh {
    /// The lightness component.
    pub l: Scalar,
    /// The chroma (colorfulness) component.
    pub c: Scalar,
    /// The hue angle in degrees.
    pub h: Scalar,
    /// The alpha (opacity) channel in `[0, 1]`.
    pub alpha: Scalar,
}

impl ColorSpace for LCh {
    fn from_color(c: &Color) -> Self {
        c.to_lch()
    }

    fn into_color(self) -> Color {
        Color::from_lch(self.l, self.c, self.h, self.alpha)
    }

    fn mix(&self, other: &Self, fraction: Fraction) -> Self {
        // make sure that the hue is preserved when mixing with gray colors
        let self_hue = if self.c < 0.1 { other.h } else { self.h };
        let other_hue = if other.c < 0.1 { self.h } else { other.h };

        Self {
            l: interpolate(self.l, other.l, fraction),
            c: interpolate(self.c, other.c, fraction),
            h: interpolate_angle(self_hue, other_hue, fraction),
            alpha: interpolate(self.alpha, other.alpha, fraction),
        }
    }
}

/// Convert a color into the CIELCh color space.
impl From<&Color> for LCh {
    fn from(color: &Color) -> Self {
        let Lab { l, a, b, alpha } = Lab::from(color);

        const RAD2DEG: Scalar = 180.0 / std::f64::consts::PI;

        let c = Scalar::sqrt(a * a + b * b);
        let h = mod_positive(Scalar::atan2(b, a) * RAD2DEG, 360.0);

        LCh { l, c, h, alpha }
    }
}

impl fmt::Display for LCh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LCh({l}, {c}, {h})", l = self.l, c = self.c, h = self.h,)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OkLCh {
    /// The lightness component.
    pub l: Scalar,
    /// The chroma (colorfulness) component.
    pub c: Scalar,
    /// The hue angle in degrees.
    pub h: Scalar,
    /// The alpha (opacity) channel in `[0, 1]`.
    pub alpha: Scalar,
}

impl ColorSpace for OkLCh {
    fn from_color(c: &Color) -> Self {
        c.to_oklch()
    }

    fn into_color(self) -> Color {
        Color::from_oklch(self.l, self.c, self.h, self.alpha)
    }

    fn mix(&self, other: &Self, fraction: Fraction) -> Self {
        // make sure that the hue is preserved when mixing with gray colors
        let self_hue = if self.c < 0.0003 { other.h } else { self.h };
        let other_hue = if other.c < 0.0003 { self.h } else { other.h };

        Self {
            l: interpolate(self.l, other.l, fraction),
            c: interpolate(self.c, other.c, fraction),
            h: interpolate_angle(self_hue, other_hue, fraction),
            alpha: interpolate(self.alpha, other.alpha, fraction),
        }
    }
}

/// Convert a color into the OkLCh color space.
impl From<&Color> for OkLCh {
    fn from(color: &Color) -> Self {
        let OkLab { l, a, b, alpha } = OkLab::from(color);

        const RAD2DEG: Scalar = 180.0 / std::f64::consts::PI;

        let c = Scalar::sqrt(a * a + b * b);
        let h = mod_positive(Scalar::atan2(b, a) * RAD2DEG, 360.0);

        OkLCh { l, c, h, alpha }
    }
}

impl fmt::Display for OkLCh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OkLCh({l}, {c}, {h})",
            l = self.l,
            c = self.c,
            h = self.h,
        )
    }
}
