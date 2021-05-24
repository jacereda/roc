#![warn(clippy::dbg_macro)]
// See github.com/rtfeldman/roc/issues/800 for discussion of the large_enum_variant check.
#![allow(clippy::large_enum_variant, clippy::upper_case_acronyms)]

pub mod borrow;
pub mod expand_rc;
pub mod inc_dec;
pub mod ir;
pub mod layout;
pub mod tail_recursion;

// Temporary, while we can build up test cases and optimize the exhaustiveness checking.
// For now, following this warning's advice will lead to nasty type inference errors.
//#[allow(clippy::ptr_arg)]
//pub mod decision_tree;
pub mod decision_tree;
#[allow(clippy::ptr_arg)]
pub mod exhaustive;
