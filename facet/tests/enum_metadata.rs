//! Nongeneric enum metadata should stay shared when used by generic consumers.

#![deny(warnings)]

use facet::{Facet, Shape, Type, UserType};

#[derive(Facet)]
#[repr(u8)]
#[allow(dead_code)] // Accessed through reflection rather than ordinary field reads.
#[allow(non_camel_case_types)]
enum Message {
    READY,
    Text(String),
    Retry { attempts: u32 },
}

mod first_consumer {
    use super::*;

    #[inline(never)]
    pub fn shape<T>() -> &'static Shape {
        std::hint::black_box(std::mem::size_of::<T>());
        Message::SHAPE
    }
}

mod second_consumer {
    use super::*;

    #[inline(never)]
    pub fn shape<T>() -> &'static Shape {
        std::hint::black_box(std::mem::size_of::<T>());
        Message::SHAPE
    }
}

#[test]
fn enum_metadata_is_shared_across_generic_consumers() {
    let first = first_consumer::shape::<u8>();
    let second = second_consumer::shape::<[u8; 32]>();
    assert!(std::ptr::eq(first, second));

    let Type::User(UserType::Enum(first)) = first.ty else {
        panic!("expected enum metadata");
    };
    let Type::User(UserType::Enum(second)) = second.ty else {
        panic!("expected enum metadata");
    };
    assert!(std::ptr::eq(first.variants, second.variants));
    for (first, second) in first.variants.iter().zip(second.variants) {
        assert!(std::ptr::eq(first.name, second.name));
        assert!(std::ptr::eq(first.data.fields, second.data.fields));
    }
}
