#[derive(Clone, Copy)]
pub enum BColor {
    Off,
    On
}

impl BColor {
    #[inline]
    pub fn flip(self) -> Self {
        match self {
            BColor::Off => BColor::On,
            BColor::On => BColor::Off,
        }
    }

    #[inline]
    pub fn to_bits_rhs(&self) -> u8 {
        match self {
            BColor::Off => 0b00000001,
            BColor::On => 0b00000010,
        }
    }

    #[inline]
    pub fn to_bits_lhs(&self) -> u8 {
        match self {
            BColor::Off => 0b00010000,
            BColor::On => 0b00100000,
        }
    }
}