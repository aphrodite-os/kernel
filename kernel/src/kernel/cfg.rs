//! Config-related stuff.

/// Get configurations as a certain type
#[macro_export]
macro_rules! cfg_int {
    ($cfg:literal, $type:ident) => {
        paste::paste! {
            {
                let cfg = env!($cfg).as_bytes();
                $crate::[< str_as_ $type >](cfg)
            }
        }
    };
}
