#![cfg(all(
    not(CONFIG_POWERON_TESTS = "false"),
    not(CONFIG_POWERON_TEST_DISPLAY = "false")
))]

use crate::arch::output::*;
use crate::display::{COLOR_BLACK, COLOR_DEFAULT, TextDisplay};
use crate::output::{sreset, toutputsln};

pub fn run(display: &dyn TextDisplay) {
    sinfosln("Running display power-on test...");

    display.clear_screen(COLOR_DEFAULT).unwrap();
    sreset();

    sinfosln("Screen cleared; attempting to write text to display");

    toutputsln("Testing display...", display).unwrap();
    toutputsln("Testing display...", display).unwrap();
    toutputsln("Testing display...", display).unwrap();

    sinfosln("Success! Clearing display with COLOR_BLACK.");

    display.clear_screen(COLOR_BLACK).unwrap();
    sreset();

    sinfosln("Clearing display with COLOR_DEFAULT");

    display.clear_screen(COLOR_DEFAULT).unwrap();
    sreset();

    sinfosln("Done running display power-on test");
}
