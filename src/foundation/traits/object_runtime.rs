use crate::objective_c_runtime::{id, traits::PNSObject};

/// An object that supports Cocoa’s reference-counted memory management system.
pub trait INSAutoreleasePool: PNSObject {
    /* Managing a Pool
     */

    /// In a reference-counted environment, releases and pops the receiver; in
    /// a garbage-collected environment, triggers garbage collection if the
    /// memory allocated since the last collection is greater than the current
    /// threshold.
    fn im_drain(&mut self);

    /// Adds a given object to the active autorelease pool in the current thread.
    ///
    /// # Arguments
    ///
    /// * `object` - The object to be added to the pool in the current thread.
    fn im_add_object(&mut self, object: id);
}
