// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

#[allow(unused_imports)]
use crate::codegen_prelude::*;

pub use read_fonts::tables::gvar::GvarFlags;

/// The ['gvar' header](https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#gvar-header)
#[derive(Clone, Debug, Default)]
pub struct Gvar {
    /// The number of variation axes for this font. This must be the
    /// same number as axisCount in the 'fvar' table.
    pub axis_count: u16,
    /// Offset from the start of this table to the shared tuple records.
    pub shared_tuples: OffsetMarker<SharedTuples, WIDTH_32>,
    /// Offsets from the start of the GlyphVariationData array to each
    /// GlyphVariationData table.
    pub glyph_variation_data_offsets: Vec<GlyphVariationData>,
}

impl Gvar {
    /// Construct a new `Gvar`
    #[allow(clippy::useless_conversion)]
    pub fn new(
        axis_count: u16,
        shared_tuples: SharedTuples,
        glyph_variation_data_offsets: Vec<GlyphVariationData>,
    ) -> Self {
        Self {
            axis_count,
            shared_tuples: shared_tuples.into(),
            glyph_variation_data_offsets,
        }
    }
}

impl FontWrite for Gvar {
    #[allow(clippy::unnecessary_cast)]
    fn write_into(&self, writer: &mut TableWriter) {
        (MajorMinor::VERSION_1_0 as MajorMinor).write_into(writer);
        self.axis_count.write_into(writer);
        (array_len(&self.shared_tuples).unwrap() as u16).write_into(writer);
        self.shared_tuples.write_into(writer);
        (self.compute_glyph_count() as u16).write_into(writer);
        (self.compute_flags() as GvarFlags).write_into(writer);
        (self.compute_data_array_offset() as u32).write_into(writer);
        (self.compile_variation_data()).write_into(writer);
    }
}

impl Validate for Gvar {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("Gvar", |ctx| {
            ctx.in_field("shared_tuples", |ctx| {
                self.shared_tuples.validate_impl(ctx);
            });
            ctx.in_field("glyph_variation_data_offsets", |ctx| {
                self.glyph_variation_data_offsets.validate_impl(ctx);
            });
        })
    }
}

impl TopLevelTable for Gvar {
    const TAG: Tag = Tag::new(b"gvar");
}

impl FontWrite for GvarFlags {
    fn write_into(&self, writer: &mut TableWriter) {
        writer.write_slice(&self.bits().to_be_bytes())
    }
}

/// Array of tuple records shared across all glyph variation data tables.
#[derive(Clone, Debug, Default)]
pub struct SharedTuples {
    pub tuples: Vec<Tuple>,
}

impl SharedTuples {
    /// Construct a new `SharedTuples`
    pub fn new(tuples: Vec<Tuple>) -> Self {
        Self { tuples }
    }
}

impl FontWrite for SharedTuples {
    fn write_into(&self, writer: &mut TableWriter) {
        self.tuples.write_into(writer);
    }
}

impl Validate for SharedTuples {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("SharedTuples", |ctx| {
            ctx.in_field("tuples", |ctx| {
                if self.tuples.len() > (u16::MAX as usize) {
                    ctx.report("array exceeds max length");
                }
                self.tuples.validate_impl(ctx);
            });
        })
    }
}

impl<'a> FromObjRef<read_fonts::tables::gvar::SharedTuples<'a>> for SharedTuples {
    fn from_obj_ref(obj: &read_fonts::tables::gvar::SharedTuples<'a>, _: FontData) -> Self {
        let offset_data = obj.offset_data();
        SharedTuples {
            tuples: obj
                .tuples()
                .iter()
                .filter_map(|x| x.map(|x| FromObjRef::from_obj_ref(&x, offset_data)).ok())
                .collect(),
        }
    }
}

impl<'a> FromTableRef<read_fonts::tables::gvar::SharedTuples<'a>> for SharedTuples {}

/// The [GlyphVariationData](https://learn.microsoft.com/en-us/typography/opentype/spec/gvar#the-glyphvariationdata-table-array) table
#[derive(Clone, Debug, Default)]
pub struct GlyphVariationDataHeader {
    /// A packed field. The high 4 bits are flags, and the low 12 bits
    /// are the number of tuple variation tables for this glyph. The
    /// number of tuple variation tables can be any number between 1
    /// and 4095.
    pub tuple_variation_count: TupleVariationCount,
    /// Array of tuple variation headers.
    pub tuple_variation_headers: Vec<TupleVariationHeader>,
}

impl Validate for GlyphVariationDataHeader {
    fn validate_impl(&self, ctx: &mut ValidationCtx) {
        ctx.in_table("GlyphVariationDataHeader", |ctx| {
            ctx.in_field("tuple_variation_headers", |ctx| {
                self.tuple_variation_headers.validate_impl(ctx);
            });
        })
    }
}