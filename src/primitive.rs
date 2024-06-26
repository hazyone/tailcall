use crate::merge_right::MergeRight;

pub trait Primitive {}

impl Primitive for u64 {}

impl Primitive for u32 {}

impl Primitive for u16 {}

impl Primitive for u8 {}

impl Primitive for usize {}

impl Primitive for i64 {}

impl Primitive for i32 {}

impl Primitive for i16 {}

impl Primitive for i8 {}

impl Primitive for f64 {}

impl Primitive for f32 {}

impl Primitive for bool {}

impl Primitive for char {}

impl Primitive for String {}

impl<A: Primitive> MergeRight for A {
    fn merge_right(self, other: Self) -> Self {
        other
    }
}
