//! Dark theme tokens as 0xRRGGBB. Zenith's brand colors replace these when the brand exists.

pub struct Theme {
    pub background: u32,
    pub surface: u32,
    pub surface_hover: u32,
    pub text: u32,
    pub text_muted: u32,
}

pub const DARK: Theme = Theme {
    background: 0x121317,
    surface: 0x1b1d23,
    surface_hover: 0x252832,
    text: 0xe8eaf0,
    text_muted: 0x8b90a0,
};

#[cfg(test)]
mod tests {
    use super::DARK;

    #[test]
    fn tokens_are_24_bit_colors() {
        for c in [
            DARK.background,
            DARK.surface,
            DARK.surface_hover,
            DARK.text,
            DARK.text_muted,
        ] {
            assert!(c <= 0xff_ffff);
        }
    }
}
