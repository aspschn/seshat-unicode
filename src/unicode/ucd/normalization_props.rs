pub(self) mod normalization_props_data;

use crate::unicode::CodePoint;
use crate::unicode::Ucd;
use crate::unicode::normalization;

pub(crate) fn comp_ex(cp: u32) -> bool {
    let code_point = CodePoint::new(cp).unwrap();
    code_point.ce()
        || normalization::singleton_decomposition(cp)
        || normalization::non_starter_decomposition(cp)
}