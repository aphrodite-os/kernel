//! Architecture-independent output functions.

use paste::paste;
use crate::display::COLOR_DEFAULT;

static mut OUTPUT_TERM_POSITION: (u32, u32) = (0, 0);

macro_rules! message_funcs {
    ($func_name:ident, $prefix:literal, $level:ident) => {
        paste! {
            /// Outputs a message &str to the terminal.
            pub fn [< t $func_name s >](s: &str, info: &dyn crate::display::TextDisplay) -> Result<(), crate::Error<'static>> {
                unsafe {
                    if cfg!($level = "false") {
                        return Ok(());
                    }
                    OUTPUT_TERM_POSITION = info.write_str(OUTPUT_TERM_POSITION, $prefix, COLOR_DEFAULT)?;
                    OUTPUT_TERM_POSITION = info.write_str(OUTPUT_TERM_POSITION, s, COLOR_DEFAULT)?;
                }
                Ok(())
            }
            /// Outputs a message &str and a newline to the terminal.
            pub fn [< t $func_name sln >](s: &str, info: &dyn crate::display::TextDisplay) -> Result<(), crate::Error<'static>> {
                unsafe {
                    if cfg!($level = "false") {
                        return Ok(());
                    }
                    OUTPUT_TERM_POSITION = info.write_str(OUTPUT_TERM_POSITION, $prefix, COLOR_DEFAULT)?;
                    OUTPUT_TERM_POSITION = info.write_str(OUTPUT_TERM_POSITION, s, COLOR_DEFAULT)?;
                    OUTPUT_TERM_POSITION.1 += 1;
                    OUTPUT_TERM_POSITION.0 = 0;
                }
                Ok(())
            }

            /// Outputs a message &\[u8] to the terminal.
            pub fn [< t $func_name b >](s: &[u8], info: &dyn crate::display::TextDisplay) -> Result<(), crate::Error<'static>> {
                unsafe {
                    if cfg!($level = "false") {
                        return Ok(());
                    }
                    OUTPUT_TERM_POSITION = info.write_str(OUTPUT_TERM_POSITION, $prefix, COLOR_DEFAULT)?;
                    OUTPUT_TERM_POSITION = info.write_bytes(OUTPUT_TERM_POSITION, s, COLOR_DEFAULT)?;
                }
                Ok(())
            }
            /// Outputs a message &\[u8] and a newline to the terminal.
            pub fn [< t $func_name bln >](s: &[u8], info: &dyn crate::display::TextDisplay) -> Result<(), crate::Error<'static>> {
                unsafe {
                    if cfg!($level = "false") {
                        return Ok(());
                    }
                    OUTPUT_TERM_POSITION = info.write_str(OUTPUT_TERM_POSITION, $prefix, COLOR_DEFAULT)?;
                    OUTPUT_TERM_POSITION = info.write_bytes(OUTPUT_TERM_POSITION, s, COLOR_DEFAULT)?;
                    OUTPUT_TERM_POSITION.1 += 1;
                    OUTPUT_TERM_POSITION.0 = 0;
                }
                Ok(())
            }

            /// Outputs a message u8 to the terminal.
            pub fn [< t $func_name u >](s: u8, info: &dyn crate::display::TextDisplay) -> Result<(), crate::Error<'static>> {
                let (width, _) = info.get_size();
                unsafe {
                    if cfg!($level = "false") {
                        return Ok(());
                    }
                    OUTPUT_TERM_POSITION = info.write_str(OUTPUT_TERM_POSITION, $prefix, COLOR_DEFAULT)?;
                    info.write_char(OUTPUT_TERM_POSITION, s, COLOR_DEFAULT)?;
                    OUTPUT_TERM_POSITION.0 += 1;
                    while OUTPUT_TERM_POSITION.0 > width {
                        OUTPUT_TERM_POSITION.0 -= width;
                        OUTPUT_TERM_POSITION.1 += 1;
                    }
                }
                Ok(())
            }

            ///////////////////////////////////////////////////////////////

            /// Outputs a message &str to the terminal without a prefix.
            pub fn [< t $func_name snp >](s: &str, info: &dyn crate::display::TextDisplay) -> Result<(), crate::Error<'static>> {
                unsafe {
                    if cfg!($level = "false") {
                        return Ok(());
                    }
                    OUTPUT_TERM_POSITION = info.write_str(OUTPUT_TERM_POSITION, s, COLOR_DEFAULT)?;
                }
                Ok(())
            }
            /// Outputs a message &str and a newline to the terminal without a prefix.
            pub fn [< t $func_name snpln >](s: &str, info: &dyn crate::display::TextDisplay) -> Result<(), crate::Error<'static>> {
                unsafe {
                    if cfg!($level = "false") {
                        return Ok(());
                    }
                    OUTPUT_TERM_POSITION = info.write_str(OUTPUT_TERM_POSITION, s, COLOR_DEFAULT)?;
                    OUTPUT_TERM_POSITION.1 += 1;
                    OUTPUT_TERM_POSITION.0 = 0;
                }
                Ok(())
            }

            /// Outputs a message &\[u8] to the terminal without a prefix.
            pub fn [< t $func_name bnp >](s: &[u8], info: &dyn crate::display::TextDisplay) -> Result<(), crate::Error<'static>> {
                unsafe {
                    if cfg!($level = "false") {
                        return Ok(());
                    }
                    OUTPUT_TERM_POSITION = info.write_bytes(OUTPUT_TERM_POSITION, s, COLOR_DEFAULT)?;
                }
                Ok(())
            }
            /// Outputs a message &\[u8] and a newline to the terminal without a prefix.
            pub fn [< t $func_name bnpln >](s: &[u8], info: &dyn crate::display::TextDisplay) -> Result<(), crate::Error<'static>> {
                unsafe {
                    if cfg!($level = "false") {
                        return Ok(());
                    }
                    OUTPUT_TERM_POSITION = info.write_bytes(OUTPUT_TERM_POSITION, s, COLOR_DEFAULT)?;
                    OUTPUT_TERM_POSITION.1 += 1;
                    OUTPUT_TERM_POSITION.0 = 0;
                }
                Ok(())
            }

            /// Outputs a message u8 to the terminal without a prefix.
            pub fn [< t $func_name unp >](s: u8, info: &dyn crate::display::TextDisplay) -> Result<(), crate::Error<'static>> {
                let (width, _) = info.get_size();
                unsafe {
                    if cfg!($level = "false") {
                        return Ok(());
                    }
                    info.write_char(OUTPUT_TERM_POSITION, s, COLOR_DEFAULT)?;
                    OUTPUT_TERM_POSITION.0 += 1;
                    while OUTPUT_TERM_POSITION.0 > width {
                        OUTPUT_TERM_POSITION.0 -= width;
                        OUTPUT_TERM_POSITION.1 += 1;
                    }
                }
                Ok(())
            }
        }
    }
}

message_funcs!(debug, "[DEBUG] ", CONFIG_PREUSER_OUTPUT_DEBUG);
message_funcs!(info, "[INFO] ", CONFIG_PREUSER_OUTPUT_INFO);
message_funcs!(warning, "[WARN] ", CONFIG_PREUSER_OUTPUT_WARN);
message_funcs!(error, "[ERROR] ", CONFIG_PREUSER_OUTPUT_ERROR);
message_funcs!(fatal, "[FATAL] ", CONFIG_PREUSER_OUTPUT_FATAL);
message_funcs!(output, "", NONE);

/// Resets the position of output to the screen.
pub fn sreset() {
    unsafe {
        OUTPUT_TERM_POSITION = (0, 0);
    }
}