#[macro_use]
extern crate thiserror;

#[macro_use]
pub mod error;

#[macro_use]
pub mod any_element;

#[macro_use]
pub mod core;

pub mod namespace;
pub mod xs;

pub trait Hancom {
    //
}

pub trait Core: Hancom {
    const NAME: &'static str;
}

pub trait Arbitrary: Hancom {
    const NAME: &'static str;
}

pub trait Enumeration: Hancom {
    fn enumeration(&self) -> &'static str;
}
