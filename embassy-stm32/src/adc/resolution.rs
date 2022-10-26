#[cfg(any(adc_v2, adc_v3, adc_g0))]
pub enum Resolution {
    TwelveBit,
    TenBit,
    EightBit,
    SixBit,
}

#[cfg(adc_v4)]
pub enum Resolution {
    SixteenBit,
    FourteenBit,
    TwelveBit,
    TenBit,
    EightBit,
}

impl Default for Resolution {
    fn default() -> Self {
        #[cfg(any(adc_v2, adc_v3, adc_g0))]
        {
            Self::TwelveBit
        }
        #[cfg(adc_v4)]
        {
            Self::SixteenBit
        }
    }
}

impl Resolution {
    pub(super) fn res(&self) -> crate::pac::adc::vals::Res {
        match self {
            #[cfg(adc_v4)]
            Resolution::SixteenBit => crate::pac::adc::vals::Res::SIXTEENBIT,
            #[cfg(adc_v4)]
            Resolution::FourteenBit => crate::pac::adc::vals::Res::FOURTEENBITV,
            Resolution::TwelveBit => crate::pac::adc::vals::Res::TWELVEBIT,
            Resolution::TenBit => crate::pac::adc::vals::Res::TENBIT,
            Resolution::EightBit => crate::pac::adc::vals::Res::EIGHTBIT,
            #[cfg(any(adc_v2, adc_v3, adc_g0))]
            Resolution::SixBit => crate::pac::adc::vals::Res::SIXBIT,
        }
    }

    pub fn to_max_count(&self) -> u32 {
        match self {
            #[cfg(adc_v4)]
            Resolution::SixteenBit => (1 << 16) - 1,
            #[cfg(adc_v4)]
            Resolution::FourteenBit => (1 << 14) - 1,
            Resolution::TwelveBit => (1 << 12) - 1,
            Resolution::TenBit => (1 << 10) - 1,
            Resolution::EightBit => (1 << 8) - 1,
            #[cfg(any(adc_v2, adc_v3, adc_g0))]
            Resolution::SixBit => (1 << 6) - 1,
        }
    }
}
