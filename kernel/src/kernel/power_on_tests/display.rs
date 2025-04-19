#![cfg(all(
    not(CONFIG_POWERON_TESTS = "false"),
    not(CONFIG_POWERON_TEST_DISPLAY = "false")
))]

use crate::display::{TextDisplay, COLOR_BLACK, COLOR_DEFAULT};
use crate::output::{toutputsln, sreset};

pub fn run(display: &dyn TextDisplay) {
    display.clear_screen(COLOR_DEFAULT).unwrap();
    sreset();
    toutputsln("Testing display...", display).unwrap();
    toutputsln("Testing display...", display).unwrap();
    toutputsln("Testing display...", display).unwrap();
    display.clear_screen(COLOR_BLACK).unwrap();
    sreset();
    display.clear_screen(COLOR_DEFAULT).unwrap();
    sreset();
}