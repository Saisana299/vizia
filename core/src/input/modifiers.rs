
use bitflags::bitflags;


bitflags! {
    pub struct Modifiers: u8 {
        const SHIFT = 1;
        const CTRL = 1<<1;
        const ALT = 1<<2;
        const LOGO = 1<<3;
    }
}