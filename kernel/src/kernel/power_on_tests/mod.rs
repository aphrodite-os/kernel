use crate::display::TextDisplay;

mod memmapalloc;

pub fn run(display: &dyn TextDisplay) { memmapalloc::run(display); }
