#![feature(arbitrary_self_types)]
#![allow(non_snake_case)]

use std::pin::Pin;

struct Struct { }

impl Struct {
    // Test using `&Struct` explicitly:

    fn ref_Struct(self: &Struct, f: &u32) -> &u32 {
        f //~ ERROR lifetime mismatch
    }

    fn box_ref_Struct(self: Box<&Struct>, f: &u32) -> &u32 {
        f //~ ERROR lifetime mismatch
    }

    fn pin_ref_Struct(self: Pin<&Struct>, f: &u32) -> &u32 {
        f //~ ERROR lifetime mismatch
    }

    fn box_box_ref_Struct(self: Box<Box<&Struct>>, f: &u32) -> &u32 {
        f //~ ERROR lifetime mismatch
    }

    fn box_pin_Struct(self: Box<Pin<&Struct>>, f: &u32) -> &u32 {
        f //~ ERROR lifetime mismatch
    }
}

fn main() { }
