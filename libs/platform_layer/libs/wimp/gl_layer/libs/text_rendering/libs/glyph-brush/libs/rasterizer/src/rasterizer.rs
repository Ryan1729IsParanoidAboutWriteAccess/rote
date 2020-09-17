#![deny(unused)]

#[cfg(not(any(feature = "rusttype", feature = "glyph_brush_draw_cache")))]
compile_error!("Either feature \"rusttype\" or \"glyph_brush_draw_cache\" must be enabled for this crate.");

/// A TextureCoords struct specifies floating point coordinates on the given
/// texture.
pub type TextureCoords = per_backend::Rect;

/// A PixelCoords struct specifies floating point coordinates on the screen
/// for the given glyph.
pub type PixelCoords = per_backend::Rect;

/// A TextureRect specifies a rectangular integer section of the given texture.
pub type TextureRect = per_backend::TextureRect;

pub struct Coords {
    pub texture: TextureCoords,
    pub pixel: PixelCoords,
}

pub use per_backend::*;

//
// rusttype
//
#[cfg(feature = "rusttype")]
mod per_backend {
    use crate::Coords;

    pub use rusttype::{
        Font, PositionedGlyph as Glyph, GlyphId,
        Scale, ScaledGlyph,
        point,
        gpu_cache::{CachedBy, CacheReadErr, CacheWriteErr},
    };
    
    use rusttype::gpu_cache;

    pub type TextureRect = U32Rect;
    
    pub struct Cache<'font>{
        cache: gpu_cache::Cache<'font>,
    }
    
    pub fn new_cache<'font>() -> Cache<'font> {
        Cache {
            cache: gpu_cache::Cache::builder()
                .dimensions(256, 256)
                .scale_tolerance(0.5)
                .position_tolerance(0.25)
                .align_4x4(false)
                .build()
        }
    }
    
    pub fn queue_glyph<'font>(cache: &mut Cache<'font>, font_index: usize, glyph: Glyph<'font>) {
        cache.cache.queue_glyph(font_index, glyph);
    }
    
    pub fn cache_queued<'font, UpdateTexture>(
        cache: &mut Cache<'font>,
        update_texture: UpdateTexture
    ) -> Result<CachedBy, CacheWriteErr>
    where for <'r> UpdateTexture: FnMut(TextureRect, &'r [u8]) {
        cache.cache.cache_queued(update_texture)
    }
    
    pub fn dimensions<'font>(cache: &Cache<'font>) -> (u32, u32) {
        cache.cache.dimensions()
    }
    
    pub fn rect_for(cache: &Cache<'_>, font_index: usize, glyph: &Glyph) -> Result<Option<Coords>, CacheReadErr> {
        cache.cache
            .rect_for(font_index, glyph)
            .map(|op| 
                op.map(|(texture, pixel)| Coords {
                    texture,
                    pixel: Rect {
                        min: point(pixel.min.x as f32, pixel.min.y as f32),
                        max: point(pixel.max.x as f32, pixel.max.y as f32),
                    },
                })
            )
    }
    
    pub fn resize_texture<'font>(
        cache: &mut Cache<'font>,
        new_width: u32,
        new_height: u32,
    ) {
        cache.cache
            .to_builder()
            .dimensions(new_width, new_height)
            .rebuild(&mut cache.cache);
    }
    
    pub type Point = rusttype::Point<f32>;
    pub type Rect = rusttype::Rect<f32>;
    type U32Rect = rusttype::Rect<u32>;
    
    pub fn new_glyph<'font>(
        font: &Font<'font>,
        c: char,
        scale: Scale,
        position: Point
    ) -> Glyph<'font> {
        font.glyph(c).scaled(scale).positioned(position)
    }
    
    pub fn add_position(glyph: &mut Glyph, position: Point) {
        let mut pos = glyph.position();
    
        pos.x += position.x;
        pos.y += position.y;
    
        glyph.set_position(pos);
    }
    
    pub fn get_scale(glyph: &Glyph) -> Scale {
        glyph.scale()
    }
    
    pub fn get_advance_width(_: &Font, glyph: &Glyph) -> f32 {
        glyph.unpositioned().h_metrics().advance_width
    }
    
    pub fn get_line_height(font: &Font, scale: Scale) -> f32 {
        let v_metrics = font.v_metrics(scale);
        v_metrics.ascent - v_metrics.descent + v_metrics.line_gap
    }
    
    pub fn intersects(glyph: &Glyph, clip: &Rect) -> bool {
        glyph
            // TODO when is this None?
            .pixel_bounding_box()
            .map(move |pixel_coords| {
                // true if pixel_coords intersects clip
                pixel_coords.min.x as f32 <= clip.max.x
                && pixel_coords.min.y as f32 <= clip.max.y
                && clip.min.x <= pixel_coords.max.x as f32
                && clip.min.y <= pixel_coords.max.y as f32
            })
            .unwrap_or(true)
    }
}

//
// glyph_brush_draw_cache
//

#[cfg(feature = "glyph_brush_draw_cache")]
mod per_backend {
    use crate::Coords;

    pub use glyph_brush_draw_cache::{
        ab_glyph::{
            GlyphId, Point,
            Rect, PxScale as Scale,
            point,
        },
        CachedBy,
    };
    
    use glyph_brush_draw_cache::{
        ab_glyph::{
            self,
            Font as _,
            ScaleFont as _,
        },
        DrawCache,
        Rectangle,
    };
    
    use std::marker::PhantomData;

    pub type TextureRect = Rectangle<u32>;
    
    pub struct Cache<'font>{
        cache: DrawCache,
        // We'll need this before can compile under glyph_brush_draw_cache
        //cache: glyph_brush_draw_cache::DrawCache,
        allow_lifetime_param: PhantomData<&'font ()>,
    }

    pub type CacheReadErr = ();
    
    pub fn rect_for(cache: &Cache<'_>, font_index: usize, glyph: &Glyph) -> Result<Option<Coords>, CacheReadErr> {
        Ok(
            cache.cache
                .rect_for(font_index, &glyph.glyph)
                .map(|(texture, pixel)| Coords {
                    texture,
                    pixel,
                })
        )
    }
    
    pub struct Font<'font>{
        font: ab_glyph::FontVec,
        allow_lifetime_param: PhantomData<&'font ()>,
    }
    
    pub struct Glyph<'font>{
        glyph: ab_glyph::Glyph,
        allow_lifetime_param: PhantomData<&'font ()>,
    }
    
    pub fn new_glyph<'font>(
        font: &Font<'font>,
        c: char,
        scale: Scale,
        position: Point,
    ) -> Glyph<'font> {
        Glyph {
            glyph: ab_glyph::Glyph {
                id: font.font.glyph_id(c),
                scale,
                position,
            },
            allow_lifetime_param: PhantomData,
        }
    }
    
    pub fn add_position(glyph: &mut Glyph, position: Point) {
        glyph.glyph.position.x += position.x;
        glyph.glyph.position.x += position.y;
    }
    
    pub fn get_scale(glyph: &Glyph) -> Scale {
        glyph.glyph.scale
    }
    
    pub fn get_advance_width(font: &Font, glyph: &Glyph) -> f32 {
        font.font.as_scaled(glyph.glyph.scale).h_advance(glyph.glyph.id)
    }
    
    pub fn get_line_height(font: &Font, scale: Scale) -> f32 {
        let scale_font = font.font.as_scaled(scale);
        scale_font.height() + scale_font.line_gap()
    }
}