#![allow(non_camel_case_types)]

//! This module contains types that may be used to define Rust struct types that match the GLSL
//! std140 memory layout.
//!
//! Std140 is a standardized memory layout for GLSL shader interface blocks (e.g. uniform blocks).
//! An interface block is a group op typed GLSL variables. For details on the layout rules for
//! std140, please refer to the section 2.12.6.4 "Standard Uniform Block Layout" of the
//! [OpenGL ES 3.0 Specification](https://www.khronos.org/registry/OpenGL/specs/es/3.0/es_spec_3.0.pdf).
//!
//! This module aims to make it easy to construct and manipulate a block of std140 compatible memory
//! as a Rust struct, such that the struct's memory layout will match a GLSL interface block if
//! every block member is pairwise type-compatible with the struct field in the corresponding
//! position. Position here relates to the order in which block members and struct fields are
//! declared, e.g.: the 1st block member must be compatible with the 1st struct field, the 2nd block
//! member must be compatible with the 2nd struct field, etc. The struct itself has to be marked
//! with the [`#[repr_std140]`][repr_std140] attribute: this ensures the rust compiler will
//! correctly order and align the fields.
//!
//! For GLSL primitive types, compatibility is defined by the following mapping from GLSL types to
//! `std140` types:
//!
//! - `float`: [float]
//! - `vec2`: [vec2]
//! - `vec3`: [vec3]
//! - `vec4`: [vec4]
//! - `mat2`: [mat2x2][struct@mat2x2]
//! - `mat3`: [mat3x3][struct@mat3x3]
//! - `mat4`: [mat4x4][struct@mat4x4]
//! - `mat2x2`: [mat2x2][struct@mat2x2]
//! - `mat2x3`: [mat2x3][struct@mat2x3]
//! - `mat2x4`: [mat2x4][struct@mat2x4]
//! - `mat3x2`: [mat3x2][struct@mat3x2]
//! - `mat3x3`: [mat3x3][struct@mat3x3]
//! - `mat3x4`: [mat3x4][struct@mat3x4]
//! - `mat4x2`: [mat4x2][struct@mat4x2]
//! - `mat4x3`: [mat4x3][struct@mat4x3]
//! - `mat4x4`: [mat4x4][struct@mat4x4]
//! - `int`: [int]
//! - `ivec2`: [ivec2]
//! - `ivec3`: [ivec3]
//! - `ivec4`: [ivec4]
//! - `uint`: [uint]
//! - `uvec2`: [uvec2]
//! - `uvec3`: [uvec3]
//! - `uvec4`: [uvec4]
//! - `bool`: [boolean]
//! - `bvec2`: [bvec2]
//! - `bvec3`: [bvec3]
//! - `bvec4`: [bvec4]
//!
//! A GLSL struct type is compatible with a field if this field's type is a Rust struct marked with
//! [`#[repr_std140]`][repr_std140] and this struct's fields are pair-wise compatible with the GLSL
//! struct field in the corresponding position.
//!
//! A GLSL array of GLSL type `T` with compatible type `T_c` (as defined above) and length `N` is
//! compatible with a field of type `std140::array<T_c, N>`.
//!
//! # Example
//!
//! Given the following GLSL declaration of an (uniform) interface block:
//!
//! ```glsl
//! struct PointLight {
//!     vec3 position;
//!     float intensity;
//! }
//!
//! layout(std140) uniform Uniforms {
//!     mat4 transform;
//!     vec3 ambient_light_color;
//!     PointLight lights[2];
//! }
//! ```
//!
//! The following will produce a Rust struct instance with a compatible memory layout:
//!
//! ```rust
//! #[std140::repr_std140]
//! struct PointLight {
//!     position: std140::vec3,
//!     intensity: std140::float,
//! }
//!
//! #[std140::repr_std140]
//! struct Uniforms {
//!     transform: std140::mat4x4,
//!     ambient_light_color: std140::vec3,
//!     lights: std140::array<PointLight, 2>
//! }
//!
//! let instance = Uniforms {
//!     transform: std140::mat4x4(
//!         std140::vec4(1.0, 0.0, 0.0, 0.0),
//!         std140::vec4(0.0, 1.0, 0.0, 0.0),
//!         std140::vec4(0.0, 0.0, 1.0, 0.0),
//!         std140::vec4(0.0, 0.0, 0.0, 1.0),
//!     ),
//!     ambient_light_color: std140::vec3(0.2, 0.2, 0.2),
//!     lights: std140::array![
//!         PointLight {
//!             position: std140::vec3(10.0, 0.0, 10.0),
//!             intensity: std140::float(0.5)
//!         },
//!         PointLight {
//!             position: std140::vec3(0.0, 10.0, 10.0),
//!             intensity: std140::float(0.8)
//!         },
//!     ]
//! };
//! ```
//!
//! Note that although the field names match the block member names in this example, this is not
//! strictly necessary: only pairwise field-type compatibility is required.
//!
//! [repr_std140]: attr.repr_std140.html

use std::fmt;
use std::ops::{Deref, DerefMut, Index, IndexMut};

/// Attribute macro that can be applied to a struct to ensure its representation is compatible with
/// the std140 memory layout convention.
///
/// Can only be applied to a struct if all of its fields implement [ReprStd140].
///
/// Any struct marked with this attribute will automatically implement [Std140Struct]
///
/// # Example
///
/// ```rust
/// #[std140::repr_std140]
/// struct PointLight {
///     position: std140::vec3,
///     intensity: std140::float,
/// }
/// ```
pub use std140_macros::repr_std140;

/// Marker trait for types that can be used as fields in structs marked with
/// [`#[repr_std140]`][repr_std140].
///
/// [repr_std140]: attr.repr_std140.html
pub unsafe trait ReprStd140 {}

/// Marker trait for types that can be used as the element type for std140 [array][struct@array]s.
pub unsafe trait Std140ArrayElement: ReprStd140 {}

/// Marker trait for struct types that were marked with [`#[repr_std140]`][repr_std140].
///
/// [repr_std140]: attr.repr_std140.html
pub unsafe trait Std140Struct {}

unsafe impl<T> ReprStd140 for T where T: Std140Struct {}
unsafe impl<T> Std140ArrayElement for T where T: Std140Struct {}

/// Represents an std140 compatible array.
///
/// All elements in an std140 array are aligned to at least 16 bytes.
///
/// The [array!][macro@array] macro may be used to initialize an array.
///
/// # Example
///
/// ```
/// let std140_array: std140::array<std140::vec2, 2> = std140::array![
///     std140::vec2(1.0, 0.0),
///     std140::vec2(0.0, 1.0),
/// ];
/// ```
#[derive(Clone, Copy)]
pub struct array<T, const LEN: usize>
where
    T: Std140ArrayElement,
{
    internal: [ArrayElementWrapper<T>; LEN],
}

impl<T, const LEN: usize> array<T, { LEN }>
where
    T: Std140ArrayElement,
{
    #[doc(hidden)]
    pub fn from_wrapped(wrapped: [ArrayElementWrapper<T>; LEN]) -> Self {
        array { internal: wrapped }
    }
}

impl<T, const LEN: usize> PartialEq for array<T, { LEN }>
where
    T: Std140ArrayElement + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        for i in 0..LEN {
            if self.internal[i] != other.internal[i] {
                return false;
            }
        }

        true
    }
}

impl<T, const LEN: usize> fmt::Debug for array<T, { LEN }> where T: Std140ArrayElement + fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.internal.iter()).finish()
    }
}

// TODO: something like this? (if that ever becomes possible)
//impl<T, const LEN: usize> Unsize<slice<T>> for array<T, {LEN}> {}
//
//pub struct slice<T> where T: Std140ArrayElement {
//    internal: *mut [ArrayElementWrapper<T>]
//}

#[doc(hidden)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C, align(16))]
pub struct ArrayElementWrapper<T>
where
    T: Std140ArrayElement,
{
    pub element: T,
}

impl<T> fmt::Debug for ArrayElementWrapper<T> where T: Std140ArrayElement + fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <T as fmt::Debug>::fmt(&self.element, f)
    }
}

/// Initializes a `std140` [array][struct@array].
///
/// # Example
///
/// ```
/// let std140_array: std140::array<std140::vec2, 2> = std140::array![
///     std140::vec2(1.0, 0.0),
///     std140::vec2(0.0, 1.0),
/// ];
/// ```
#[macro_export]
macro_rules! array {
    ($elem:expr; $n:expr) => {
        $crate::array::from_wrapped([$crate::ArrayElementWrapper {
            element: $elem
        }; $n])
    };
    ($($x:expr),*) => {
        $crate::array::from_wrapped([
            $(
                $crate::ArrayElementWrapper {
                    element: $x
                }
            ),*
        ])
    };
    ($($x:expr,)*) => ($crate::array![$($x),*])
}

unsafe impl<T, const LEN: usize> ReprStd140 for array<T, { LEN }> where T: Std140ArrayElement {}

/// A 32-bit floating point value.
///
/// # Example
///
/// ```
/// let value = std140::float(0.5);
/// ```
#[repr(C, align(4))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct float(pub f32);

unsafe impl ReprStd140 for float {}
unsafe impl Std140ArrayElement for float {}

/// A column vector of 2 [float] values.
///
/// # Example
///
/// ```
/// let value = std140::vec2(0.0, 1.0);
/// ```
#[repr(C, align(8))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct vec2(pub f32, pub f32);

impl vec2 {
    /// Creates a new [vec2] with zeros in all positions.
    pub fn zero() -> Self {
        vec2(0.0, 0.0)
    }
}

unsafe impl ReprStd140 for vec2 {}
unsafe impl Std140ArrayElement for vec2 {}

impl Index<usize> for vec2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 3 [float] values.
///
/// # Example
///
/// ```
/// let value = std140::vec3(0.0, 0.0, 1.0);
/// ```
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct vec3(pub f32, pub f32, pub f32);

impl vec3 {
    /// Creates a new [vec3] with zeros in all positions.
    pub fn zero() -> Self {
        vec3(0.0, 0.0, 0.0)
    }
}

unsafe impl ReprStd140 for vec3 {}
unsafe impl Std140ArrayElement for vec3 {}

impl Index<usize> for vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 4 [float] values.
///
/// # Example
///
/// ```
/// let value = std140::vec4(0.0, 0.0, 0.0, 1.0);
/// ```
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct vec4(pub f32, pub f32, pub f32, pub f32);

impl vec4 {
    /// Creates a new [vec4] with zeros in all positions.
    pub fn zero() -> Self {
        vec4(0.0, 0.0, 0.0, 0.0)
    }
}

unsafe impl ReprStd140 for vec4 {}
unsafe impl Std140ArrayElement for vec4 {}

impl Index<usize> for vec4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A 32-bit signed integer value.
///
/// # Example
///
/// ```
/// let value = std140::int(1);
/// ```
#[repr(C, align(4))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct int(pub i32);

unsafe impl ReprStd140 for int {}
unsafe impl Std140ArrayElement for int {}

/// A column vector of 2 [int] values.
///
/// # Example
///
/// ```
/// let value = std140::ivec2(0, 1);
/// ```
#[repr(C, align(8))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ivec2(pub i32, pub i32);

impl ivec2 {
    /// Creates a new [ivec2] with zeros in all positions.
    pub fn zero() -> Self {
        ivec2(0, 0)
    }
}

unsafe impl ReprStd140 for ivec2 {}
unsafe impl Std140ArrayElement for ivec2 {}

impl Index<usize> for ivec2 {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for ivec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 3 [int] values.
///
/// # Example
///
/// ```
/// let value = std140::ivec3(0, 0, 1);
/// ```
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ivec3(pub i32, pub i32, pub i32);

impl ivec3 {
    /// Creates a new [ivec3] with zeros in all positions.
    pub fn zero() -> Self {
        ivec3(0, 0, 0)
    }
}

unsafe impl ReprStd140 for ivec3 {}
unsafe impl Std140ArrayElement for ivec3 {}

impl Index<usize> for ivec3 {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for ivec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 4 [int] values.
///
/// # Example
///
/// ```
/// let value = std140::ivec4(0, 0, 0, 1);
/// ```
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ivec4(pub i32, pub i32, pub i32, pub i32);

impl ivec4 {
    /// Creates a new [ivec4] with zeros in all positions.
    pub fn zero() -> Self {
        ivec4(0, 0, 0, 0)
    }
}

unsafe impl ReprStd140 for ivec4 {}
unsafe impl Std140ArrayElement for ivec4 {}

impl Index<usize> for ivec4 {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for ivec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A 32-bit unsigned integer value.
///
/// # Example
///
/// ```
/// let value = std140::uint(1);
/// ```
#[repr(C, align(4))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct uint(pub u32);

unsafe impl ReprStd140 for uint {}
unsafe impl Std140ArrayElement for uint {}

/// A column vector of 2 [uint] values.
///
/// # Example
///
/// ```
/// let value = std140::uvec2(0, 1);
/// ```
#[repr(C, align(8))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct uvec2(pub u32, pub u32);

impl uvec2 {
    /// Creates a new [uvec2] with zeros in all positions.
    pub fn zero() -> Self {
        uvec2(0, 0)
    }
}

unsafe impl ReprStd140 for uvec2 {}
unsafe impl Std140ArrayElement for uvec2 {}

impl Index<usize> for uvec2 {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for uvec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 3 [uint] values.
///
/// # Example
///
/// ```
/// let value = std140::uvec3(0, 0, 1);
/// ```
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct uvec3(pub u32, pub u32, pub u32);

impl uvec3 {
    /// Creates a new [uvec3] with zeros in all positions.
    pub fn zero() -> Self {
        uvec3(0, 0, 0)
    }
}

unsafe impl ReprStd140 for uvec3 {}
unsafe impl Std140ArrayElement for uvec3 {}

impl Index<usize> for uvec3 {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for uvec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 4 [uint] values.
///
/// # Example
///
/// ```
/// let value = std140::uvec4(0, 0, 0, 1);
/// ```
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct uvec4(pub u32, pub u32, pub u32, pub u32);

impl uvec4 {
    /// Creates a new [uvec4] with zeros in all positions.
    pub fn zero() -> Self {
        uvec4(0, 0, 0, 0)
    }
}

unsafe impl ReprStd140 for uvec4 {}
unsafe impl Std140ArrayElement for uvec4 {}

impl Index<usize> for uvec4 {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for uvec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A 32-bit boolean value.
///
/// [boolean::False] is stored identically to a [uint] of `0`; [boolean::True] is stored identically
/// to a [uint] of `1`.
///
/// # Example
///
/// ```
/// let value = std140::uint(1);
/// ```
#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum boolean {
    True = 1,
    False = 0,
}

unsafe impl ReprStd140 for boolean {}
unsafe impl Std140ArrayElement for boolean {}

impl From<bool> for boolean {
    fn from(value: bool) -> Self {
        match value {
            true => boolean::True,
            false => boolean::False,
        }
    }
}

/// A column vector of 2 [boolean] values.
///
/// # Example
///
/// ```
/// let value = std140::bvec2(std140::boolean::False, std140::boolean::True);
/// ```
#[repr(C, align(8))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct bvec2(pub boolean, pub boolean);

unsafe impl ReprStd140 for bvec2 {}
unsafe impl Std140ArrayElement for bvec2 {}

impl Index<usize> for bvec2 {
    type Output = boolean;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for bvec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 3 [boolean] values.
///
/// # Example
///
/// ```
/// let value = std140::bvec3(std140::boolean::False, std140::boolean::False, std140::boolean::True);
/// ```
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct bvec3(pub boolean, pub boolean, pub boolean);

unsafe impl ReprStd140 for bvec3 {}
unsafe impl Std140ArrayElement for bvec3 {}

impl Index<usize> for bvec3 {
    type Output = boolean;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for bvec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A column vector of 4 [boolean] values.
///
/// # Example
///
/// ```
/// let value = std140::bvec4(
///     std140::boolean::False,
///     std140::boolean::False,
///     std140::boolean::False,
///     std140::boolean::True
/// );
/// ```
#[repr(C, align(16))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct bvec4(pub boolean, pub boolean, pub boolean, pub boolean);

unsafe impl ReprStd140 for bvec4 {}
unsafe impl Std140ArrayElement for bvec4 {}

impl Index<usize> for bvec4 {
    type Output = boolean;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for bvec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!("Index out of bounds"),
        }
    }
}

/// A matrix with 2 columns and 2 rows, represented by 2 [vec2] vectors.
///
/// # Example
///
/// ```
/// let value = std140::mat2x2(
///     std140::vec2(0.0, 1.0),
///     std140::vec2(0.0, 1.0),
/// );
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct mat2x2 {
    columns: array<vec2, 2>,
}

impl mat2x2 {
    /// Creates a new [mat2x2] with zeros in all positions.
    pub fn zero() -> Self {
        mat2x2(vec2::zero(), vec2::zero())
    }
}

/// Initializes a [mat2x2][struct@mat2x2]
///
/// # Example
///
/// See [mat2x2][struct@mat2x2].
pub fn mat2x2(c0: vec2, c1: vec2) -> mat2x2 {
    mat2x2 {
        columns: array![c0, c1],
    }
}

unsafe impl ReprStd140 for mat2x2 {}
unsafe impl Std140ArrayElement for mat2x2 {}

impl Deref for mat2x2 {
    type Target = array<vec2, 2>;

    fn deref(&self) -> &Self::Target {
        &self.columns
    }
}

impl DerefMut for mat2x2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.columns
    }
}

impl fmt::Debug for mat2x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("mat2x2{:?}", &self.columns))
    }
}

/// A matrix with 2 columns and 3 rows, represented by 2 [vec3] vectors.
///
/// # Example
///
/// ```
/// let value = std140::mat2x3(
///     std140::vec3(0.0, 0.0, 1.0),
///     std140::vec3(0.0, 0.0, 1.0),
/// );
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct mat2x3 {
    columns: array<vec3, 2>,
}

impl mat2x3 {
    /// Creates a new [mat2x3] with zeros in all positions.
    pub fn zero() -> Self {
        mat2x3(vec3::zero(), vec3::zero())
    }
}

/// Initializes a [mat2x3][struct@mat2x3]
///
/// # Example
///
/// See [mat2x3][struct@mat2x3].
pub fn mat2x3(c0: vec3, c1: vec3) -> mat2x3 {
    mat2x3 {
        columns: array![c0, c1],
    }
}

unsafe impl ReprStd140 for mat2x3 {}
unsafe impl Std140ArrayElement for mat2x3 {}

impl Deref for mat2x3 {
    type Target = array<vec3, 2>;

    fn deref(&self) -> &Self::Target {
        &self.columns
    }
}

impl DerefMut for mat2x3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.columns
    }
}

impl fmt::Debug for mat2x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("mat2x3{:?}", &self.columns))
    }
}

/// A matrix with 2 columns and 4 rows, represented by 2 [vec4] vectors.
///
/// # Example
///
/// ```
/// let value = std140::mat2x4(
///     std140::vec4(0.0, 0.0, 0.0, 1.0),
///     std140::vec4(0.0, 0.0, 0.0, 1.0),
/// );
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct mat2x4 {
    columns: array<vec4, 2>,
}

impl mat2x4 {
    /// Creates a new [mat2x4] with zeros in all positions.
    pub fn zero() -> Self {
        mat2x4(vec4::zero(), vec4::zero())
    }
}

/// Initializes a [mat2x4][struct@mat2x4]
///
/// # Example
///
/// See [mat2x4][struct@mat2x4].
pub fn mat2x4(c0: vec4, c1: vec4) -> mat2x4 {
    mat2x4 {
        columns: array![c0, c1],
    }
}

unsafe impl ReprStd140 for mat2x4 {}
unsafe impl Std140ArrayElement for mat2x4 {}

impl Deref for mat2x4 {
    type Target = array<vec4, 2>;

    fn deref(&self) -> &Self::Target {
        &self.columns
    }
}

impl DerefMut for mat2x4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.columns
    }
}

impl fmt::Debug for mat2x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("mat2x4{:?}", &self.columns))
    }
}

/// A matrix with 3 columns and 2 rows, represented by 3 [vec2] vectors.
///
/// # Example
///
/// ```
/// let value = std140::mat3x2(
///     std140::vec2(0.0, 1.0),
///     std140::vec2(0.0, 1.0),
///     std140::vec2(0.0, 1.0),
/// );
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct mat3x2 {
    columns: array<vec2, 3>,
}

impl mat3x2 {
    /// Creates a new [mat3x2] with zeros in all positions.
    pub fn zero() -> Self {
        mat3x2(vec2::zero(), vec2::zero(), vec2::zero())
    }
}

/// Initializes a [mat3x2][struct@mat3x2]
///
/// # Example
///
/// See [mat3x2][struct@mat3x2].
pub fn mat3x2(c0: vec2, c1: vec2, c2: vec2) -> mat3x2 {
    mat3x2 {
        columns: array![c0, c1, c2],
    }
}

unsafe impl ReprStd140 for mat3x2 {}
unsafe impl Std140ArrayElement for mat3x2 {}

impl Deref for mat3x2 {
    type Target = array<vec2, 3>;

    fn deref(&self) -> &Self::Target {
        &self.columns
    }
}

impl DerefMut for mat3x2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.columns
    }
}

impl fmt::Debug for mat3x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("mat3x2{:?}", &self.columns))
    }
}

/// A matrix with 3 columns and 3 rows, represented by 3 [vec3] vectors.
///
/// # Example
///
/// ```
/// let value = std140::mat3x3(
///     std140::vec3(0.0, 0.0, 1.0),
///     std140::vec3(0.0, 0.0, 1.0),
///     std140::vec3(0.0, 0.0, 1.0),
/// );
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct mat3x3 {
    columns: array<vec3, 3>,
}

impl mat3x3 {
    /// Creates a new [mat3x3] with zeros in all positions.
    pub fn zero() -> Self {
        mat3x3(vec3::zero(), vec3::zero(), vec3::zero())
    }
}

/// Initializes a [mat3x3][struct@mat3x3]
///
/// # Example
///
/// See [mat3x3][struct@mat3x3].
pub fn mat3x3(c0: vec3, c1: vec3, c2: vec3) -> mat3x3 {
    mat3x3 {
        columns: array![c0, c1, c2],
    }
}

unsafe impl ReprStd140 for mat3x3 {}
unsafe impl Std140ArrayElement for mat3x3 {}

impl Deref for mat3x3 {
    type Target = array<vec3, 3>;

    fn deref(&self) -> &Self::Target {
        &self.columns
    }
}

impl DerefMut for mat3x3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.columns
    }
}

impl fmt::Debug for mat3x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("mat3x3{:?}", &self.columns))
    }
}

/// A matrix with 3 columns and 4 rows, represented by 3 [vec4] vectors.
///
/// # Example
///
/// ```
/// let value = std140::mat3x4(
///     std140::vec4(0.0, 0.0, 0.0, 1.0),
///     std140::vec4(0.0, 0.0, 0.0, 1.0),
///     std140::vec4(0.0, 0.0, 0.0, 1.0),
/// );
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct mat3x4 {
    columns: array<vec4, 3>,
}

impl mat3x4 {
    /// Creates a new [mat3x4] with zeros in all positions.
    pub fn zero() -> Self {
        mat3x4(vec4::zero(), vec4::zero(), vec4::zero())
    }
}

/// Initializes a [mat3x4][struct@mat3x4]
///
/// # Example
///
/// See [mat3x4][struct@mat3x4].
pub fn mat3x4(c0: vec4, c1: vec4, c2: vec4) -> mat3x4 {
    mat3x4 {
        columns: array![c0, c1, c2],
    }
}

unsafe impl ReprStd140 for mat3x4 {}
unsafe impl Std140ArrayElement for mat3x4 {}

impl Deref for mat3x4 {
    type Target = array<vec4, 3>;

    fn deref(&self) -> &Self::Target {
        &self.columns
    }
}

impl DerefMut for mat3x4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.columns
    }
}

impl fmt::Debug for mat3x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("mat3x4{:?}", &self.columns))
    }
}

/// A matrix with 4 columns and 2 rows, represented by 4 [vec2] vectors.
///
/// # Example
///
/// ```
/// let value = std140::mat4x2(
///     std140::vec2(0.0, 1.0),
///     std140::vec2(0.0, 1.0),
///     std140::vec2(0.0, 1.0),
///     std140::vec2(0.0, 1.0),
/// );
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct mat4x2 {
    columns: array<vec2, 4>,
}

impl mat4x2 {
    /// Creates a new [mat4x2] with zeros in all positions.
    pub fn zero() -> Self {
        mat4x2(vec2::zero(), vec2::zero(), vec2::zero(), vec2::zero())
    }
}

/// Initializes a [mat4x2][struct@mat4x2]
///
/// # Example
///
/// See [mat4x2][struct@mat4x2].
pub fn mat4x2(c0: vec2, c1: vec2, c2: vec2, c3: vec2) -> mat4x2 {
    mat4x2 {
        columns: array![c0, c1, c2, c3],
    }
}

unsafe impl ReprStd140 for mat4x2 {}
unsafe impl Std140ArrayElement for mat4x2 {}

impl Deref for mat4x2 {
    type Target = array<vec2, 4>;

    fn deref(&self) -> &Self::Target {
        &self.columns
    }
}

impl DerefMut for mat4x2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.columns
    }
}

impl fmt::Debug for mat4x2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("mat4x2{:?}", &self.columns))
    }
}

/// A matrix with 4 columns and 3 rows, represented by 4 [vec3] vectors.
///
/// # Example
///
/// ```
/// let value = std140::mat4x3(
///     std140::vec3(0.0, 0.0, 1.0),
///     std140::vec3(0.0, 0.0, 1.0),
///     std140::vec3(0.0, 0.0, 1.0),
///     std140::vec3(0.0, 0.0, 1.0),
/// );
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct mat4x3 {
    columns: array<vec3, 4>,
}

impl mat4x3 {
    /// Creates a new [mat4x3] with zeros in all positions.
    pub fn zero() -> Self {
        mat4x3(vec3::zero(), vec3::zero(), vec3::zero(), vec3::zero())
    }
}

/// Initializes a [mat4x3][struct@mat4x3]
///
/// # Example
///
/// See [mat4x3][struct@mat4x3].
pub fn mat4x3(c0: vec3, c1: vec3, c2: vec3, c3: vec3) -> mat4x3 {
    mat4x3 {
        columns: array![c0, c1, c2, c3],
    }
}

unsafe impl ReprStd140 for mat4x3 {}
unsafe impl Std140ArrayElement for mat4x3 {}

impl Deref for mat4x3 {
    type Target = array<vec3, 4>;

    fn deref(&self) -> &Self::Target {
        &self.columns
    }
}

impl DerefMut for mat4x3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.columns
    }
}

impl fmt::Debug for mat4x3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("mat4x3{:?}", &self.columns))
    }
}

/// A matrix with 4 columns and 4 rows, represented by 4 [vec4] vectors.
///
/// # Example
///
/// ```
/// let value = std140::mat4x4(
///     std140::vec4(0.0, 0.0, 0.0, 1.0),
///     std140::vec4(0.0, 0.0, 0.0, 1.0),
///     std140::vec4(0.0, 0.0, 0.0, 1.0),
///     std140::vec4(0.0, 0.0, 0.0, 1.0),
/// );
/// ```
#[derive(Clone, Copy, PartialEq)]
pub struct mat4x4 {
    columns: array<vec4, 4>,
}

impl mat4x4 {
    /// Creates a new [mat4x4] with zeros in all positions.
    pub fn zero() -> Self {
        mat4x4(vec4::zero(), vec4::zero(), vec4::zero(), vec4::zero())
    }
}

/// Initializes a [mat4x4][struct@mat4x4]
///
/// # Example
///
/// See [mat4x4][struct@mat4x4].
pub fn mat4x4(c0: vec4, c1: vec4, c2: vec4, c3: vec4) -> mat4x4 {
    mat4x4 {
        columns: array![c0, c1, c2, c3],
    }
}

unsafe impl ReprStd140 for mat4x4 {}
unsafe impl Std140ArrayElement for mat4x4 {}

impl Deref for mat4x4 {
    type Target = array<vec4, 4>;

    fn deref(&self) -> &Self::Target {
        &self.columns
    }
}

impl DerefMut for mat4x4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.columns
    }
}

impl fmt::Debug for mat4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("mat4x4{:?}", &self.columns))
    }
}
