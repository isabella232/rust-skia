#[allow(non_camel_case_types)]

pub type scalar = rust_skia::SkScalar;

pub trait Scalar {
    const NEARLY_ZERO: Self;
    const ONE: Self;
    const HALF: Self;
}

impl Scalar for scalar {
    const NEARLY_ZERO: Self = 1.0 / ((1 << 12) as Self);
    const ONE: Self = 1.0;
    const HALF: Self = 0.5;
}

pub(crate) type u8cpu = rust_skia::U8CPU;
pub type GlyphId = rust_skia::SkGlyphID;
pub type Unichar = rust_skia::SkUnichar;
pub type FontTableTag = rust_skia::SkFontTableTag;

mod bbh_factory;
pub use self::bbh_factory::*;

mod bitmap;
pub use self::bitmap::*;

mod blend_mode;
pub use self::blend_mode::*;

mod canvas;
pub use self::canvas::*;

mod color;
pub use self::color::*;

mod color_filter;
pub use self::color_filter::*;

mod color_space;
pub use self::color_space::*;

mod data;
pub use self::data::*;

mod encoded_image_format;
pub use self::encoded_image_format::*;

mod font;
pub use self::font::*;

mod font_metrics;
pub use self::font_metrics::*;

mod font_style;
pub use self::font_style::*;

mod font_types;
pub use self::font_types::*;

mod image;
pub use self::image::*;

mod image_info;
pub use self::image_info::*;

mod matrix;
pub use self::matrix::*;

mod matrix44;
pub use self::matrix44::*;

mod matrix_typemask;
pub use self::matrix_typemask::*;

mod paint;
pub use self::paint::*;

mod path;
pub use self::path::*;

mod path_effect;
pub use self::path_effect::*;

mod picture;
pub use self::picture::*;

mod picture_recorder;
pub use self::picture_recorder::*;

mod region;
pub use self::region::*;

mod rrect;
pub use self::rrect::*;

mod stroke_rec;
pub use self::stroke_rec::*;

mod surface;
pub use self::surface::*;

mod surface_props;
pub use self::surface_props::*;

mod typeface;
pub use self::typeface::*;

mod vector4;
pub use self::vector4::*;

mod vertices;
pub use self::vertices::*;

mod yuva_index;
pub use self::yuva_index::*;

pub use crate::skia_euclid::*;
