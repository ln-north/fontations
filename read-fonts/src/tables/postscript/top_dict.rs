use std::ops::Range;

use crate::tables::postscript::{ Error, StringId, dict::{ Entry, entries } };
use types::Fixed;

#[derive(Default)]
pub struct TopDict {
    pub version: Option<StringId>,
    pub notice: Option<StringId>,
    pub full_name: Option<StringId>,
    pub family_name: Option<StringId>,
    pub weight: Option<StringId>,
    pub font_bbox: Option<[Fixed; 4]>,
    pub charstrings_offset: Option<usize>,
    pub private_dict_range: Option<Range<usize>>,
    pub variation_store_offset: Option<usize>,
    pub copyright: Option<StringId>,
    pub is_fixed_pitch: Option<bool>,
    pub italic_angle: Option<Fixed>,
    pub underline_position: Option<Fixed>,
    pub underline_thickness: Option<Fixed>,
    pub paint_type: Option<i32>,
    pub charstring_type: Option<i32>,
    pub font_matrix: Option<[Fixed; 6]>,
    pub stroke_width: Option<Fixed>,
    pub fd_array_offset: Option<usize>,
    pub fd_select_offset: Option<usize>,
    pub encoding: Option<i32>,
    pub charset: Option<i32>,
    pub unique_id: Option<i32>,
    pub synthetic_base: Option<i32>,
    pub post_script: Option<StringId>,
    pub base_font_name: Option<StringId>,
    pub ros: Option<Ros>,
    pub cid_font_version: Option<Fixed>,
    pub cid_font_revision: Option<Fixed>,
    pub cid_font_type: Option<i32>,
    pub cid_count: Option<u32>,
    pub uid_base: Option<i32>,
    pub font_name: Option<StringId>,
}

pub struct Ros {
  pub registry: Option<StringId>,
  pub ordering: Option<StringId>,
  pub supplement: Option<Fixed>,
}


impl TopDict {
    pub fn new(top_dict_data: &[u8]) -> Result<Self, Error> {
        let mut top_dict = Self::default();

        for entry in entries(top_dict_data, None) {
            match entry.unwrap() {
                Entry::Version(version) => {
                    top_dict.version = Some(version);
                }
                Entry::Notice(notice) => {
                    top_dict.notice = Some(notice);
                }
                Entry::FullName(full_name) => {
                    top_dict.full_name = Some(full_name);
                }
                Entry::FamilyName(family_name) => {
                    top_dict.family_name = Some(family_name);
                }
                Entry::Weight(weight) => {
                    top_dict.weight = Some(weight);
                }
                Entry::FontBbox(bbox) => {
                    top_dict.font_bbox = Some(bbox);
                }
                Entry::CharstringsOffset(offset) => {
                    top_dict.charstrings_offset = Some(offset);
                }
                Entry::PrivateDictRange(range) => {
                    top_dict.private_dict_range = Some(range);
                }
                Entry::VariationStoreOffset(offset) => {
                    top_dict.variation_store_offset = Some(offset);
                }
                Entry::Copyright(copyright) => {
                    top_dict.copyright = Some(copyright);
                }
                Entry::IsFixedPitch(flag) => {
                    top_dict.is_fixed_pitch = Some(flag);
                }
                Entry::ItalicAngle(angle) => {
                    top_dict.italic_angle = Some(angle);
                }
                Entry::UnderlinePosition(position) => {
                    top_dict.underline_position = Some(position);
                }
                Entry::UnderlineThickness(thickness) => {
                    top_dict.underline_thickness = Some(thickness);
                }
                Entry::PaintType(paint_type) => {
                    top_dict.paint_type = Some(paint_type);
                }
                Entry::CharstringType(charstring_type) => {
                    top_dict.charstring_type = Some(charstring_type);
                }
                Entry::FontMatrix(matrix) => {
                    top_dict.font_matrix = Some(matrix);
                }
                Entry::StrokeWidth(stroke_width) => {
                    top_dict.stroke_width = Some(stroke_width);
                }
                Entry::FdArrayOffset(offset) => {
                    top_dict.fd_array_offset = Some(offset);
                }
                Entry::FdSelectOffset(offset) => {
                    top_dict.fd_select_offset = Some(offset);
                }
                Entry::Ros { registry, ordering, supplement } => {
                    top_dict.ros = Some(Ros {
                        registry: Some(registry),
                        ordering: Some(ordering),
                        supplement: Some(supplement),
                    });
                }
                Entry::PostScript(script) => {
                    top_dict.post_script = Some(script);
                }
                Entry::BaseFontName(name) => {
                    top_dict.base_font_name = Some(name);
                }
                Entry::CidFontVersion(version) => {
                    top_dict.cid_font_version = Some(version);
                }
                Entry::CidFontRevision(revision) => {
                    top_dict.cid_font_revision = Some(revision);
                }
                Entry::CidFontType(font_type) => {
                    top_dict.cid_font_type = Some(font_type);
                }
                Entry::CidCount(count) => {
                    top_dict.cid_count = Some(count);
                }
                Entry::UidBase(base) => {
                    top_dict.uid_base = Some(base);
                }
                Entry::FontName(name) => {
                    top_dict.font_name = Some(name);
                }
                Entry::Encoding(encoding) => {
                    top_dict.encoding = Some(encoding);
                }
                Entry::Charset(charset) => {
                    top_dict.charset = Some(charset);
                }
                Entry::UniqueId(id) => {
                    top_dict.unique_id = Some(id);
                }
                Entry::SyntheticBase(base) => {
                    top_dict.synthetic_base = Some(base);
                }
                _ => {}
            }
        }
        Ok(top_dict)
    }
}

// TODO
// #[cfg(test)]
// mod tests {
    
// }