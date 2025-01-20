//! Functions to output to various things
#![cfg(any(target_arch = "x86"))]

use super::ports;

use paste::paste;

macro_rules! message_funcs {
    ($func_name:ident, $prefix:literal) => {
        paste! {
            /// Outputs a $func_name message &str to the debug serial port.
            pub fn [< s $func_name s >](s: &str) {
                ports::outbs(super::DEBUG_PORT, $prefix.as_bytes());
                ports::outbs(super::DEBUG_PORT, s.as_bytes());
            }
            /// Outputs a $func_name message &\[u8] to the debug serial port.
            pub fn [< s $func_name b >](s: &[u8]) {
                ports::outbs(super::DEBUG_PORT, $prefix.as_bytes());
                ports::outbs(super::DEBUG_PORT, s);
            }
        }
    }
}

message_funcs!(debug, "[DEBUG] ");
message_funcs!(info, "[INFO] ");
message_funcs!(warning, "[WARN] ");
message_funcs!(error, "[ERROR] ");
message_funcs!(fatal, "[FATAL] ");

