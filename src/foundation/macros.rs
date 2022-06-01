/// A macro to create new `NSArray`s.
pub macro ns_array {
    () => {
        $crate::foundation::NSArray::new()
    },
    ($($x:expr),*) => {
        $crate::foundation::NSArray::from(vec![$($x),*])
    },
}

/// A macro to create new `NSDictionary`s.
pub macro NSDictionary {
    (@single $($x:tt)*) => (()),
    (@count $($rest:expr),*) => (<[()]>::len(&[$(NSDictionary!(@single $rest)),*])),

    ($($key:expr => $value:expr,)+) => { NSDictionary!($($key => $value),+) },
    ($($key:expr => $value:expr),*) => {
        {
            use $crate::foundation::traits::INSMutableDictionary;
            let capacity = NSDictionary!(@count $($key),*);
            let mut map = $crate::foundation::NSMutableDictionary::from(capacity as $crate::foundation::UInt);
            $(
                let _ = map.setObject($key, $value);
            )*
            $crate::foundation::NSDictionary::from(map)
        }
    },
}

/// Respond to problem situations in your interactions with APIs, and fine-tune your app for better debugging.
pub macro ns_log {
    ($format:expr) => {
        use $crate::foundation::NSLog;
        unsafe {
            NSLog($format.into());
        }
    },
    ($format:expr, $($arg:expr),+) => {
        use $crate::foundation::NSLog;
        unsafe {
            NSLog($format.into(), $($arg),+);
        }
    },
}
