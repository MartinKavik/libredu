// @TODO: Missing module in the repo - I've crated a dummy one.

use seed::{*, prelude::*};

use crate::Context;

// ------ ------
//     Init
// ------ ------

pub fn init() -> Model {
    Model
}

// ------ ------
//     Model
// ------ ------

pub struct Model;

// ------ ------
//    Update
// ------ ------

#[derive(Debug)]
pub struct Msg;

pub fn update(_: Msg, _: &mut crate::Page, _: &mut impl Orders<Msg>, _: &mut Context) {}

// ------ ------
//     View
// ------ ------

pub fn view(_: &Model)-> Node<Msg>{
    div!["sign in"]
}

