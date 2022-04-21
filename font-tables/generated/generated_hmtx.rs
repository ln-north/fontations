// THIS FILE IS AUTOGENERATED.
// Any changes to this file will be overwritten.
// For more information about how codegen works, see font-codegen/README.md

use font_types::*;

/// The [hmtx (Horizontal Metrics)](https://docs.microsoft.com/en-us/typography/opentype/spec/hmtx) table
pub struct Hmtx<'a> {
    h_metrics: zerocopy::LayoutVerified<&'a [u8], [LongHorMetric]>,
    left_side_bearings: zerocopy::LayoutVerified<&'a [u8], [BigEndian<i16>]>,
}

impl<'a> font_types::FontReadWithArgs<'a, (u16, u16)> for Hmtx<'a> {
    fn read_with_args(bytes: &'a [u8], args: &(u16, u16)) -> Option<(Self, &'a [u8])> {
        let __resolved_number_of_h_metrics = args.0;
        let __resolved_num_glyphs = args.1;
        let (h_metrics, bytes) =
            zerocopy::LayoutVerified::<_, [LongHorMetric]>::new_slice_unaligned_from_prefix(
                bytes,
                __resolved_number_of_h_metrics as usize as usize,
            )?;
        let (left_side_bearings, bytes) =
            zerocopy::LayoutVerified::<_, [BigEndian<i16>]>::new_slice_unaligned_from_prefix(
                bytes,
                n_glyphs_less_n_metrics(__resolved_num_glyphs, __resolved_number_of_h_metrics)
                    as usize,
            )?;
        let _bytes = bytes;
        let result = Hmtx {
            h_metrics,
            left_side_bearings,
        };
        Some((result, _bytes))
    }
}

impl<'a> Hmtx<'a> {
    /// Paired advance width and left side bearing values for each
    /// glyph. Records are indexed by glyph ID.
    pub fn h_metrics(&self) -> &[LongHorMetric] {
        &self.h_metrics
    }

    /// Left side bearings for glyph IDs greater than or equal to
    /// numberOfHMetrics.
    pub fn left_side_bearings(&self) -> &[BigEndian<i16>] {
        &self.left_side_bearings
    }
}

#[derive(Clone, Copy, Debug, zerocopy :: FromBytes, zerocopy :: Unaligned)]
#[repr(C)]
pub struct LongHorMetric {
    /// Advance width, in font design units.
    pub advance_width: BigEndian<u16>,
    /// Glyph left side bearing, in font design units.
    pub lsb: BigEndian<i16>,
}

impl LongHorMetric {
    /// Advance width, in font design units.
    pub fn advance_width(&self) -> u16 {
        self.advance_width.get()
    }

    /// Glyph left side bearing, in font design units.
    pub fn lsb(&self) -> i16 {
        self.lsb.get()
    }
}

fn n_glyphs_less_n_metrics(num_glyphs: u16, num_metrics: u16) -> usize {
    num_glyphs.saturating_sub(num_metrics) as usize
}

