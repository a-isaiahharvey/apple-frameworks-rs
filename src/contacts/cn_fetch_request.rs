use objective_c_runtime_proc_macros::interface_impl;

use crate::objective_c_runtime::{macros::object, traits::PNSObject};

object! {
    ///
    unsafe pub struct CNFetchRequest;
}

#[interface_impl(NSObject)]
impl CNFetchRequest {}
