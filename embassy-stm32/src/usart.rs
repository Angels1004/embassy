#![macro_use]

use core::marker::PhantomData;

use embassy::util::Unborrow;
use embassy_extras::unborrow;

use crate::gpio::{NoPin, Pin};
use crate::pac::usart::{regs, vals, Usart};
use crate::peripherals;

#[non_exhaustive]
pub struct Config {
    pub baudrate: u32,
    pub data_bits: u8,
    pub stop_bits: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            baudrate: 115200,
            data_bits: 8,
            stop_bits: 1,
        }
    }
}

pub struct Uart<'d, T: Instance> {
    inner: T,
    phantom: PhantomData<&'d mut T>,
}

impl<'d, T: Instance> Uart<'d, T> {
    pub fn new(
        inner: impl Unborrow<Target = T>,
        rx: impl Unborrow<Target = impl RxPin<T>>,
        tx: impl Unborrow<Target = impl TxPin<T>>,
        cts: impl Unborrow<Target = impl CtsPin<T>>,
        rts: impl Unborrow<Target = impl RtsPin<T>>,
        config: Config,
    ) -> Self {
        unborrow!(inner, rx, tx, cts, rts);

        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

pub(crate) mod sealed {
    use crate::gpio::{OptionalPin, Pin};

    use super::*;
    pub trait Instance {
        fn regs(&self) -> Usart;
    }
    pub trait RxPin<T: Instance>: Pin {
        fn af_num(&self) -> u8;
    }
    pub trait TxPin<T: Instance>: Pin {
        fn af_num(&self) -> u8;
    }
    pub trait CtsPin<T: Instance>: Pin {
        fn af_num(&self) -> u8;
    }
    pub trait RtsPin<T: Instance>: Pin {
        fn af_num(&self) -> u8;
    }
    pub trait CkPin<T: Instance>: Pin {
        fn af_num(&self) -> u8;
    }
}
pub trait Instance: sealed::Instance {}
pub trait RxPin<T: Instance>: sealed::RxPin<T> {}
pub trait TxPin<T: Instance>: sealed::TxPin<T> {}
pub trait CtsPin<T: Instance>: sealed::CtsPin<T> {}
pub trait RtsPin<T: Instance>: sealed::RtsPin<T> {}
pub trait CkPin<T: Instance>: sealed::CkPin<T> {}

macro_rules! impl_usart {
    ($inst:ident) => {
        impl crate::usart::sealed::Instance for peripherals::$inst {
            fn regs(&self) -> crate::pac::usart::Usart {
                crate::pac::$inst
            }
        }
        impl crate::usart::Instance for peripherals::$inst {}
    };
}

macro_rules! impl_usart_pin {
    ($inst:ident, $func:ident, $pin:ident, $af:expr) => {
        impl crate::usart::sealed::$func<peripherals::$inst> for peripherals::$pin {
            fn af_num(&self) -> u8 {
                $af
            }
        }
        impl crate::usart::$func<peripherals::$inst> for peripherals::$pin {}
    };
}
