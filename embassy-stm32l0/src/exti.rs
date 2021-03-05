use core::future::Future;
use core::mem;
use core::pin::Pin;

use embassy::interrupt::Interrupt;
use embassy::traits::gpio::{WaitForFallingEdge, WaitForRisingEdge};
use embassy::util::InterruptFuture;

use crate::hal::{
    exti::{Exti, ExtiLine, GpioLine, TriggerEdge},
    gpio,
    syscfg::SYSCFG,
};
use crate::interrupt;
use crate::pac::EXTI;

pub struct ExtiManager {
    syscfg: SYSCFG,
}

impl<'a> ExtiManager {
    pub fn new(_exti: Exti, syscfg: SYSCFG) -> Self {
        Self { syscfg }
    }

    pub fn new_pin<T, I>(&'static mut self, pin: T, interrupt: I) -> ExtiPin<T, I>
    where
        T: PinWithInterrupt<Interrupt = I>,
        I: Interrupt,
    {
        ExtiPin {
            pin,
            interrupt,
            mgr: self,
        }
    }
}

pub struct ExtiPin<T, I> {
    pin: T,
    interrupt: I,
    mgr: &'static mut ExtiManager,
}

impl<T: PinWithInterrupt<Interrupt = I> + 'static, I: Interrupt + 'static> WaitForRisingEdge
    for ExtiPin<T, I>
{
    type Future<'a> = impl Future<Output = ()> + 'a;

    fn wait_for_rising_edge<'a>(self: Pin<&'a mut Self>) -> Self::Future<'a> {
        let s = unsafe { self.get_unchecked_mut() };

        let line = s.pin.line();
        Exti::unpend(line);

        async move {
            let exti: EXTI = unsafe { mem::transmute(()) };
            let mut exti = Exti::new(exti);
            let fut = InterruptFuture::new(&mut s.interrupt);

            exti.listen_gpio(&mut s.mgr.syscfg, s.pin.port(), line, TriggerEdge::Rising);
            fut.await;

            Exti::unpend(line);
        }
    }
}

impl<T: PinWithInterrupt<Interrupt = I> + 'static, I: Interrupt + 'static> WaitForFallingEdge
    for ExtiPin<T, I>
{
    type Future<'a> = impl Future<Output = ()> + 'a;

    fn wait_for_falling_edge<'a>(self: Pin<&'a mut Self>) -> Self::Future<'a> {
        let s = unsafe { self.get_unchecked_mut() };

        let line = s.pin.line();
        Exti::unpend(line);

        async move {
            let exti: EXTI = unsafe { mem::transmute(()) };
            let mut exti = Exti::new(exti);
            let fut = InterruptFuture::new(&mut s.interrupt);

            exti.listen_gpio(&mut s.mgr.syscfg, s.pin.port(), line, TriggerEdge::Falling);
            fut.await;

            Exti::unpend(line);
        }
    }
}

mod private {
    pub trait Sealed {}
}

pub trait PinWithInterrupt: private::Sealed {
    type Interrupt;
    fn port(&self) -> gpio::Port;
    fn line(&self) -> GpioLine;
}

macro_rules! exti {
    ($($PER:ident => ($set:ident, $pin:ident),)+) => {
        $(
            impl<T> private::Sealed for gpio::$set::$pin<T> {}
            impl<T> PinWithInterrupt for gpio::$set::$pin<T> {
                type Interrupt = interrupt::$PER;
                fn port(&self) -> gpio::Port {
                    self.port()
                }
                fn line(&self) -> GpioLine {
                    GpioLine::from_raw_line(self.pin_number()).unwrap()
                }
            }
        )+
    }
}

exti! {
    EXTI0_1 => (gpioa, PA0),
    EXTI0_1 => (gpioa, PA1),
    EXTI2_3 => (gpioa, PA2),
    EXTI2_3 => (gpioa, PA3),
    EXTI4_15 => (gpioa, PA4),
    EXTI4_15 => (gpioa, PA5),
    EXTI4_15 => (gpioa, PA6),
    EXTI4_15 => (gpioa, PA7),
    EXTI4_15 => (gpioa, PA8),
    EXTI4_15 => (gpioa, PA9),
    EXTI4_15 => (gpioa, PA10),
    EXTI4_15 => (gpioa, PA11),
    EXTI4_15 => (gpioa, PA12),
    EXTI4_15 => (gpioa, PA13),
    EXTI4_15 => (gpioa, PA14),
    EXTI4_15 => (gpioa, PA15),
}

exti! {
    EXTI0_1 => (gpiob, PB0),
    EXTI0_1 => (gpiob, PB1),
    EXTI2_3 => (gpiob, PB2),
    EXTI2_3 => (gpiob, PB3),
    EXTI4_15 => (gpiob, PB4),
    EXTI4_15 => (gpiob, PB5),
    EXTI4_15 => (gpiob, PB6),
    EXTI4_15 => (gpiob, PB7),
    EXTI4_15 => (gpiob, PB8),
    EXTI4_15 => (gpiob, PB9),
    EXTI4_15 => (gpiob, PB10),
    EXTI4_15 => (gpiob, PB11),
    EXTI4_15 => (gpiob, PB12),
    EXTI4_15 => (gpiob, PB13),
    EXTI4_15 => (gpiob, PB14),
    EXTI4_15 => (gpiob, PB15),
}

exti! {
    EXTI0_1 => (gpioc, PC0),
    EXTI0_1 => (gpioc, PC1),
    EXTI2_3 => (gpioc, PC2),
    EXTI2_3 => (gpioc, PC3),
    EXTI4_15 => (gpioc, PC4),
    EXTI4_15 => (gpioc, PC5),
    EXTI4_15 => (gpioc, PC6),
    EXTI4_15 => (gpioc, PC7),
    EXTI4_15 => (gpioc, PC8),
    EXTI4_15 => (gpioc, PC9),
    EXTI4_15 => (gpioc, PC10),
    EXTI4_15 => (gpioc, PC11),
    EXTI4_15 => (gpioc, PC12),
    EXTI4_15 => (gpioc, PC13),
    EXTI4_15 => (gpioc, PC14),
    EXTI4_15 => (gpioc, PC15),
}

exti! {
    EXTI0_1 => (gpiod, PD0),
    EXTI0_1 => (gpiod, PD1),
    EXTI2_3 => (gpiod, PD2),
    EXTI2_3 => (gpiod, PD3),
    EXTI4_15 => (gpiod, PD4),
    EXTI4_15 => (gpiod, PD5),
    EXTI4_15 => (gpiod, PD6),
    EXTI4_15 => (gpiod, PD7),
    EXTI4_15 => (gpiod, PD8),
    EXTI4_15 => (gpiod, PD9),
    EXTI4_15 => (gpiod, PD10),
    EXTI4_15 => (gpiod, PD11),
    EXTI4_15 => (gpiod, PD12),
    EXTI4_15 => (gpiod, PD13),
    EXTI4_15 => (gpiod, PD14),
    EXTI4_15 => (gpiod, PD15),
}

exti! {
    EXTI0_1 => (gpioe, PE0),
    EXTI0_1 => (gpioe, PE1),
    EXTI2_3 => (gpioe, PE2),
    EXTI2_3 => (gpioe, PE3),
    EXTI4_15 => (gpioe, PE4),
    EXTI4_15 => (gpioe, PE5),
    EXTI4_15 => (gpioe, PE6),
    EXTI4_15 => (gpioe, PE7),
    EXTI4_15 => (gpioe, PE8),
    EXTI4_15 => (gpioe, PE9),
    EXTI4_15 => (gpioe, PE10),
    EXTI4_15 => (gpioe, PE11),
    EXTI4_15 => (gpioe, PE12),
    EXTI4_15 => (gpioe, PE13),
    EXTI4_15 => (gpioe, PE14),
    EXTI4_15 => (gpioe, PE15),
}

exti! {
    EXTI0_1 => (gpioh, PH0),
    EXTI0_1 => (gpioh, PH1),
    EXTI4_15 => (gpioh, PH9),
    EXTI4_15 => (gpioh, PH10),
}
