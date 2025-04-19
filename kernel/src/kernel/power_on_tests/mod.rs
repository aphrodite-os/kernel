#![cfg(not(CONFIG_POWERON_TESTS = "false"))]

use crate::display::TextDisplay;

mod display;
mod memmapalloc;

pub fn run(display: &dyn TextDisplay) {
    #[cfg(not(CONFIG_POWERON_TEST_DISPLAY = "false"))]
    display::run(display);

    #[cfg(not(CONFIG_POWERON_TEST_ALLOC = "false"))]
    memmapalloc::run(display);
}
