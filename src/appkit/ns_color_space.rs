use std::fmt;

use objc::{
    class, msg_send,
    runtime::{Class, Object, Protocol, Sel},
    sel, sel_impl,
};
use objc_id::Id;

use crate::{
    foundation::{NSString, UInt},
    objective_c_runtime::{
        id,
        traits::{FromId, PNSObject, ToId},
    },
    utils::to_bool,
};

/// An object that represents a custom color space.
pub struct NSColorSpace {
    ptr: Id<Object>,
}

impl ToId for NSColorSpace {
    fn to_id(mut self) -> id {
        &mut *self.ptr
    }
}

impl FromId for NSColorSpace {
    unsafe fn from_id(ptr: id) -> Self {
        Self {
            ptr: Id::from_ptr(ptr),
        }
    }
}

impl PNSObject for NSColorSpace {
    fn im_class<'a>() -> &'a Class {
        class!(NSColorSpace)
    }

    fn im_is_equal(&self, object: &Self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isEqual: object]) }
    }

    fn ip_hash(&self) -> UInt {
        unsafe { msg_send![&*self.ptr, hash] }
    }

    fn im_is_kind_of_class(&self, class: Class) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isKindOfClass: class]) }
    }

    fn im_is_member_of_class(&self, class: Class) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isMemberOfClass: class]) }
    }

    fn im_responds_to_selector(&self, selector: Sel) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, respondsToSelector: selector]) }
    }

    fn im_conforms_to_protocol(&self, protocol: Protocol) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, conformsToProtocol: protocol]) }
    }

    fn ip_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, description]) }
    }

    fn ip_debug_description(&self) -> NSString {
        unsafe { NSString::from_id(msg_send![&*self.ptr, debugDescription]) }
    }

    fn im_perform_selector(&self, selector: Sel) -> id {
        unsafe { msg_send![&*self.ptr, performSelector: selector] }
    }

    fn im_perform_selector_with_object(&self, selector: Sel, with_object: id) -> id {
        unsafe { msg_send![&*self.ptr, performSelector: selector withObject: with_object] }
    }

    fn im_is_proxy(&self) -> bool {
        unsafe { to_bool(msg_send![&*self.ptr, isProxy]) }
    }
}

impl fmt::Debug for NSColorSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_debug_description())
    }
}

impl fmt::Display for NSColorSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ip_description())
    }
}
