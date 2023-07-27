use super::{CharsetsFormat0, CharsetsFormat0Marker, CharsetsRange1, CharsetsRange2};
#[allow(unused_imports)]
use crate::{codegen_prelude::*, FontData, FontReadWithArgs, ReadArgs, ReadError};

// Definition of Charsets enum, which can represent multiple format types
pub enum Charsets<'a> {
    Format0(CharsetsFormat0<'a>),
    Format1(CharsetsFormat1<'a>),
    Format2(CharsetsFormat2<'a>),
}

impl<'a> Charsets<'a> {
    // Depending on the format read from the data, the function creates an appropriate Charsets variant.
    pub fn read(data: FontData<'a>, num_glyphs: u16) -> Result<Self, ReadError> {
        let format: u8 = data.read_at(0usize)?;
        match format {
            CharsetsFormat0Marker::FORMAT => {
                Ok(Self::Format0(CharsetsFormat0::read(data, num_glyphs)?))
            }
            CharsetsFormat1Marker::FORMAT => {
                Ok(Self::Format1(CharsetsFormat1::read(data, num_glyphs)?))
            }
            CharsetsFormat2Marker::FORMAT => {
                Ok(Self::Format2(CharsetsFormat2::read(data, num_glyphs)?))
            }
            other => Err(ReadError::InvalidFormat(other.into())),
        }
    }
}

impl Format<u8> for CharsetsFormat1Marker {
    const FORMAT: u8 = 1;
}

/// Charsets format 1.
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct CharsetsFormat1Marker {
    ranges_byte_len: usize,
}

impl CharsetsFormat1Marker {
    fn format_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u8::RAW_BYTE_LEN
    }
    fn ranges_byte_range(&self) -> Range<usize> {
        let start = self.format_byte_range().end;
        start..start + self.ranges_byte_len
    }
}

impl ReadArgs for CharsetsFormat1<'_> {
    type Args = u16;
}

impl<'a> FontReadWithArgs<'a> for CharsetsFormat1<'a> {
    fn read_with_args(data: FontData<'a>, args: &Self::Args) -> Result<Self, ReadError> {
        let num_glyphs = *args;
        let mut cursor = data.cursor();
        let mut ranges_byte_len = 0;
        cursor.advance::<u8>();
        let mut gid = 1;
        while gid < num_glyphs {
            let range = CharsetsRange1 {
                first: BigEndian::<u16>::new([cursor.read::<u8>()?, cursor.read::<u8>()?]),
                n_left: cursor.read::<u8>()?,
            };
            ranges_byte_len += CharsetsRange1::RAW_BYTE_LEN;
            gid += range.n_left() as u16 + 1;
        }
        cursor.finish(CharsetsFormat1Marker { ranges_byte_len })
    }
}

impl<'a> CharsetsFormat1<'a> {
    /// A constructor that requires additional arguments.
    ///
    /// This type requires some external state in order to be
    /// parsed.
    pub fn read(data: FontData<'a>, num_glyphs: u16) -> Result<Self, ReadError> {
        let args = num_glyphs;
        Self::read_with_args(data, &args)
    }
}

/// Charsets format 1.
pub type CharsetsFormat1<'a> = TableRef<'a, CharsetsFormat1Marker>;

impl<'a> CharsetsFormat1<'a> {
    /// Format = 1.
    pub fn format(&self) -> u8 {
        let range = self.shape.format_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Range1 array.
    pub fn ranges(&self) -> &'a [CharsetsRange1] {
        let range = self.shape.ranges_byte_range();
        self.data.read_array(range).unwrap()
    }
}

impl Format<u8> for CharsetsFormat2Marker {
    const FORMAT: u8 = 2;
}

/// Charsets format 2.
#[derive(Debug, Clone, Copy)]
#[doc(hidden)]
pub struct CharsetsFormat2Marker {
    ranges_byte_len: usize,
}

impl CharsetsFormat2Marker {
    fn format_byte_range(&self) -> Range<usize> {
        let start = 0;
        start..start + u8::RAW_BYTE_LEN
    }
    fn ranges_byte_range(&self) -> Range<usize> {
        let start = self.format_byte_range().end;
        start..start + self.ranges_byte_len
    }
}

impl ReadArgs for CharsetsFormat2<'_> {
    type Args = u16;
}

impl<'a> FontReadWithArgs<'a> for CharsetsFormat2<'a> {
    fn read_with_args(data: FontData<'a>, args: &Self::Args) -> Result<Self, ReadError> {
        let num_glyphs = *args;
        let mut cursor = data.cursor();
        let mut ranges_byte_len = 0;
        cursor.advance::<u8>();
        let mut gid = 1;
        while gid < num_glyphs {
            let range = CharsetsRange2 {
                first: BigEndian::<u16>::new([cursor.read::<u8>()?, cursor.read::<u8>()?]),
                n_left: BigEndian::<u16>::new([cursor.read::<u8>()?, cursor.read::<u8>()?]),
            };
            ranges_byte_len += CharsetsRange2::RAW_BYTE_LEN;
            gid += range.n_left() + 1;
        }
        cursor.finish(CharsetsFormat2Marker { ranges_byte_len })
    }
}

impl<'a> CharsetsFormat2<'a> {
    /// A constructor that requires additional arguments.
    ///
    /// This type requires some external state in order to be
    /// parsed.
    pub fn read(data: FontData<'a>, num_glyphs: u16) -> Result<Self, ReadError> {
        let args = num_glyphs;
        Self::read_with_args(data, &args)
    }
}

/// Charsets format 2.
pub type CharsetsFormat2<'a> = TableRef<'a, CharsetsFormat2Marker>;

impl<'a> CharsetsFormat2<'a> {
    /// Format = 2.
    pub fn format(&self) -> u8 {
        let range = self.shape.format_byte_range();
        self.data.read_at(range.start).unwrap()
    }

    /// Range2 array.
    pub fn ranges(&self) -> &'a [CharsetsRange2] {
        let range = self.shape.ranges_byte_range();
        return self.data.read_array(range).unwrap();
    }
}

impl<'a> Charsets<'a> {
    // This method attempts to return the SID for a given glyph ID. If no such SID is found, it returns None.
    // Note that glyph IDs (GID) are 1-based in this system, and a GID of 0 is not considered (hence the subtraction by 1 at the start)
    pub fn sid(&self, glyph_id: GlyphId) -> Option<u16> {
        let gid = glyph_id.to_u16() - 1;

        if gid == 0 {
            return Some(0);
        }

        match self {
            Self::Format0(sids) => {
                let sids = sids.glyph();
                sids.get(gid as usize).map(|sid| sid.get())
            }
            Self::Format1(ranges) => {
                let ranges = ranges.ranges();
                for range in ranges {
                    let first = range.first();
                    let last = first + (range.n_left() as u16) + 1;
                    if gid >= first && gid < last {
                        return Some((range.first() + ((gid - first) as u16)).into());
                    }
                }
                None
            }
            Self::Format2(ranges) => {
                let ranges = ranges.ranges();
                for range in ranges {
                    let first = range.first();
                    let last = first + range.n_left() + 1;
                    if gid >= first && gid < last {
                        return Some((range.first() + ((gid - first) as u16)).into());
                    }
                }
                None
            }
        }
    }

    // This function calculates and returns the order of glyphs for this charset.
    // The glyph order is a list where the index is the GID and the value is the corresponding SID.
    // Note that as GID is 1-based, the 0 index (which does not correspond to a valid GID) is filled with 0.
    pub fn glyph_order(&self) -> Vec<u16> {
        match self {
            Self::Format0(sids) => {
                let mut glyph_order: Vec<u16> = vec![0];
                glyph_order.extend(sids.glyph().iter().map(|sid| sid.get()));
                glyph_order
            }
            Self::Format1(ranges1) => {
                let mut glyph_order: Vec<u16> = vec![0];
                glyph_order.extend(ranges1.ranges().iter().flat_map(|range| {
                    let first = range.first();
                    let last = first + (range.n_left() as u16) + 1;
                    (first..last).collect::<Vec<_>>()
                }));
                glyph_order
            }
            Self::Format2(ranges2) => {
                let mut glyph_order: Vec<u16> = vec![0];
                glyph_order.extend(ranges2.ranges().iter().flat_map(|range: &CharsetsRange2| {
                    let first = range.first();
                    let last = first + range.n_left() + 1;
                    (first..last).collect::<Vec<_>>()
                }));
                glyph_order
            }
        }
    }
}
