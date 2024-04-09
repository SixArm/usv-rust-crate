//! Convert USV data from various representations into other various representations.
//!
//! Currently, this code focuses on the two most-common use cases:
//!
//! * Convert from USV data that uses vectors with &str content into a USV String.
//!
//! * Convert from USV data that uses vectors with String content into a USV String.
//!
//! For developers:
//!
//! * We intend for this module to be able to add more conversions in the future.
//!
//! * Each conversion function is deliberately in its own file.

pub trait IntoUSVString {
    fn into_usv_string(&self) -> String;
}

pub mod vec_str;
pub mod vec2_str;
pub mod vec3_str;
pub mod vec4_str;

pub mod vec_string;
pub mod vec2_string;
pub mod vec3_string;
pub mod vec4_string;
