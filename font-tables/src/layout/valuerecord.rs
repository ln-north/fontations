//! A GPOS ValueRecord

use super::ValueFormat;
use crate::{
    parse_prelude::*,
    read::{ComputeSize, FontReadWithArgs},
};

impl ValueFormat {
    /// Return the number of bytes required to store a [`ValueRecord`] in this format.
    #[inline]
    pub fn record_byte_len(self) -> usize {
        self.bits().count_ones() as usize * u16::RAW_BYTE_LEN
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct ValueRecord {
    pub x_placement: Option<BigEndian<i16>>,
    pub y_placement: Option<BigEndian<i16>>,
    pub x_advance: Option<BigEndian<i16>>,
    pub y_advance: Option<BigEndian<i16>>,
    pub x_placement_device: Option<BigEndian<i16>>,
    pub y_placement_device: Option<BigEndian<i16>>,
    pub x_advance_device: Option<BigEndian<i16>>,
    pub y_advance_device: Option<BigEndian<i16>>,
}

impl ValueRecord {
    pub fn read_old(data: &[u8], format: ValueFormat) -> Result<Self, ReadError> {
        let data = FontData::new(data);
        Self::read(data, format)
    }

    pub fn read<'a>(data: FontData<'a>, format: ValueFormat) -> Result<Self, ReadError> {
        let mut this = ValueRecord::default();
        let mut cursor = data.cursor();

        if format.contains(ValueFormat::X_PLACEMENT) {
            this.x_placement = Some(cursor.read()?);
        }
        if format.contains(ValueFormat::Y_PLACEMENT) {
            this.y_placement = Some(cursor.read()?);
        }
        if format.contains(ValueFormat::X_ADVANCE) {
            this.x_advance = Some(cursor.read()?);
        }
        if format.contains(ValueFormat::Y_ADVANCE) {
            this.y_advance = Some(cursor.read()?);
        }
        if format.contains(ValueFormat::X_PLACEMENT_DEVICE) {
            this.x_placement_device = Some(cursor.read()?);
        }
        if format.contains(ValueFormat::Y_PLACEMENT_DEVICE) {
            this.y_placement_device = Some(cursor.read()?);
        }
        if format.contains(ValueFormat::X_ADVANCE_DEVICE) {
            this.x_advance_device = Some(cursor.read()?);
        }
        if format.contains(ValueFormat::Y_ADVANCE_DEVICE) {
            this.y_advance_device = Some(cursor.read()?);
        }
        Ok(this)
    }
}

impl<'a> FontReadWithArgs<'a, ValueFormat> for ValueRecord {
    fn read_with_args(data: FontData<'a>, args: &ValueFormat) -> Result<Self, ReadError> {
        ValueRecord::read(data, *args)
    }
}

impl std::fmt::Debug for ValueRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut f = f.debug_struct("ValueRecord");
        self.x_placement.map(|x| f.field("x_placement", &x));
        self.y_placement.map(|y| f.field("y_placement", &y));
        self.x_advance.map(|x| f.field("x_advance", &x));
        self.y_advance.map(|y| f.field("y_advance", &y));
        self.x_placement_device
            .map(|x| f.field("x_placement_device", &x));
        self.y_placement_device
            .map(|y| f.field("y_placement_device", &y));
        self.x_advance_device
            .map(|x| f.field("x_advance_device", &x));
        self.y_advance_device
            .map(|y| f.field("y_advance_device", &y));
        f.finish()
    }
}

impl ComputeSize<ValueFormat> for ValueRecord {
    #[inline]
    fn compute_size(args: &ValueFormat) -> usize {
        args.record_byte_len()
    }
}
