use crate::{object, objective_c_runtime::{
    macros::{interface_impl},
    traits::PNSObject,
}};

object! {
    /// An immutable collection of unique integer values that represent indexes in another collection.
    unsafe pub struct NSIndexSet;
}

#[interface_impl(NSObject)]
impl NSIndexSet {}
