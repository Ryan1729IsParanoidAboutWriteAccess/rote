#![deny(unused)]
mod font;
mod glyph_brush;
mod owned_section;
mod section;

pub use crate::{
    font::*,
    glyph_brush::*,
    owned_section::*,
    rusttype::*,
    section::*,
};

use std::hash::Hash;

/// A scaled glyph that's relatively positioned.
pub struct RelativePositionedGlyph<'font> {
    pub relative: Point<f32>,
    pub glyph: ScaledGlyph<'font>,
}

impl<'font> RelativePositionedGlyph<'font> {
    #[inline]
    pub fn bounds(&self) -> Option<Rect<f32>> {
        self.glyph.exact_bounding_box().map(|mut bb| {
            bb.min.x += self.relative.x;
            bb.min.y += self.relative.y;
            bb.max.x += self.relative.x;
            bb.max.y += self.relative.y;
            bb
        })
    }

    #[inline]
    pub fn screen_positioned(self, mut pos: Point<f32>) -> PositionedGlyph<'font> {
        pos.x += self.relative.x;
        pos.y += self.relative.y;
        self.glyph.positioned(pos)
    }
}

/// Re-exported rusttype types.
pub mod rusttype {
    pub use full_rusttype::{
        point, vector, Error, Font, Glyph, GlyphId, HMetrics, Point, PositionedGlyph,
        Rect, Scale, ScaledGlyph, SharedBytes, Vector, VMetrics,
    };
}

pub type CalculatedGlyph<'font> = (PositionedGlyph<'font>, Color);

/// Logic to calculate glyph positioning using [`Font`](struct.Font.html),
/// [`SectionGeometry`](struct.SectionGeometry.html) and
/// [`SectionText`](struct.SectionText.html).
pub trait GlyphPositioner: Hash {
    /// Calculate a sequence of positioned glyphs to render. Implementations should
    /// return the same result when called with the same arguments to allow layout caching.
    fn calculate_glyphs<'font>(
        &self,
        font: &Font<'font>,
        scale: Scale,
        geometry: &SectionGeometry,
        sections: &[SectionText<'_>],
    ) -> Vec<CalculatedGlyph<'font>>;
}
