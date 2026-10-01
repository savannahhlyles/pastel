//! Small shared helpers for the command-line front end.

use pastel::named::{NamedColor, NAMED_COLORS};
use pastel::Color;

/// Named colors separated from the target by more than this CIEDE2000 distance
/// are not perceptually similar and are dropped from the "similar colors"
/// listing.
const PERCEPTIBLE_CUTOFF: f64 = 17.0;

/// Perceived distance between a named color and a target color, using the
/// CIEDE2000 color-difference metric.
fn perceptual_distance(candidate: &NamedColor, target: &Color) -> f64 {
    candidate.color.distance_delta_e_ciede2000(target)
}

/// Returns the known named colors ordered from most to least similar to the
/// given color, limited to those close enough to actually count as similar.
/// Exact duplicates (colors that share the same RGB value under different names)
/// are collapsed so each distinct color appears once.
pub fn similar_colors(color: &Color) -> Vec<&NamedColor> {
    let mut colors: Vec<&NamedColor> = NAMED_COLORS.iter().collect();
    colors.sort_by_key(|nc| (1000.0 * perceptual_distance(nc, color)) as i32);
    colors.dedup_by(|n1, n2| n1.color == n2.color);
    // Drop colors that are clearly too different to be called "similar".
    colors.retain(|nc| !(perceptual_distance(nc, color) > PERCEPTIBLE_CUTOFF));
    colors
}
