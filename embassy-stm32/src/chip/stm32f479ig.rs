use embassy_extras::peripherals;
peripherals!(
    EXTI0, EXTI1, EXTI2, EXTI3, EXTI4, EXTI5, EXTI6, EXTI7, EXTI8, EXTI9, EXTI10, EXTI11, EXTI12,
    EXTI13, EXTI14, EXTI15, ADC1, ADC2, ADC3, CAN1, CAN2, CRYP, DAC, DCMI, DMA2D, ETH, PA0, PA1,
    PA2, PA3, PA4, PA5, PA6, PA7, PA8, PA9, PA10, PA11, PA12, PA13, PA14, PA15, PB0, PB1, PB2, PB3,
    PB4, PB5, PB6, PB7, PB8, PB9, PB10, PB11, PB12, PB13, PB14, PB15, PC0, PC1, PC2, PC3, PC4, PC5,
    PC6, PC7, PC8, PC9, PC10, PC11, PC12, PC13, PC14, PC15, PD0, PD1, PD2, PD3, PD4, PD5, PD6, PD7,
    PD8, PD9, PD10, PD11, PD12, PD13, PD14, PD15, PE0, PE1, PE2, PE3, PE4, PE5, PE6, PE7, PE8, PE9,
    PE10, PE11, PE12, PE13, PE14, PE15, PF0, PF1, PF2, PF3, PF4, PF5, PF6, PF7, PF8, PF9, PF10,
    PF11, PF12, PF13, PF14, PF15, PG0, PG1, PG2, PG3, PG4, PG5, PG6, PG7, PG8, PG9, PG10, PG11,
    PG12, PG13, PG14, PG15, PH0, PH1, PH2, PH3, PH4, PH5, PH6, PH7, PH8, PH9, PH10, PH11, PH12,
    PH13, PH14, PH15, PI0, PI1, PI2, PI3, PI4, PI5, PI6, PI7, PI8, PI9, PI10, PI11, PI12, PI13,
    PI14, PI15, PJ0, PJ1, PJ2, PJ3, PJ4, PJ5, PJ6, PJ7, PJ8, PJ9, PJ10, PJ11, PJ12, PJ13, PJ14,
    PJ15, PK0, PK1, PK2, PK3, PK4, PK5, PK6, PK7, PK8, PK9, PK10, PK11, PK12, PK13, PK14, PK15,
    HASH, I2C1, I2C2, I2C3, IWDG, LTDC, QUADSPI, RCC, RNG, RTC, SAI1, SDIO, SPI1, SPI2, SPI3, SPI4,
    SPI5, SPI6, SYSCFG, TIM1, TIM10, TIM11, TIM12, TIM13, TIM14, TIM2, TIM3, TIM4, TIM5, TIM6,
    TIM7, TIM8, TIM9, UART4, UART5, UART7, UART8, USART1, USART2, USART3, USART6, USB_OTG_FS,
    USB_OTG_HS, WWDG
);
pub const GPIO_BASE: usize = 0x40020000;
pub const GPIO_STRIDE: usize = 0x400;
impl_gpio_pin!(PA0, 0, 0, EXTI0);
impl_gpio_pin!(PA1, 0, 1, EXTI1);
impl_gpio_pin!(PA2, 0, 2, EXTI2);
impl_gpio_pin!(PA3, 0, 3, EXTI3);
impl_gpio_pin!(PA4, 0, 4, EXTI4);
impl_gpio_pin!(PA5, 0, 5, EXTI5);
impl_gpio_pin!(PA6, 0, 6, EXTI6);
impl_gpio_pin!(PA7, 0, 7, EXTI7);
impl_gpio_pin!(PA8, 0, 8, EXTI8);
impl_gpio_pin!(PA9, 0, 9, EXTI9);
impl_gpio_pin!(PA10, 0, 10, EXTI10);
impl_gpio_pin!(PA11, 0, 11, EXTI11);
impl_gpio_pin!(PA12, 0, 12, EXTI12);
impl_gpio_pin!(PA13, 0, 13, EXTI13);
impl_gpio_pin!(PA14, 0, 14, EXTI14);
impl_gpio_pin!(PA15, 0, 15, EXTI15);
impl_gpio_pin!(PB0, 1, 0, EXTI0);
impl_gpio_pin!(PB1, 1, 1, EXTI1);
impl_gpio_pin!(PB2, 1, 2, EXTI2);
impl_gpio_pin!(PB3, 1, 3, EXTI3);
impl_gpio_pin!(PB4, 1, 4, EXTI4);
impl_gpio_pin!(PB5, 1, 5, EXTI5);
impl_gpio_pin!(PB6, 1, 6, EXTI6);
impl_gpio_pin!(PB7, 1, 7, EXTI7);
impl_gpio_pin!(PB8, 1, 8, EXTI8);
impl_gpio_pin!(PB9, 1, 9, EXTI9);
impl_gpio_pin!(PB10, 1, 10, EXTI10);
impl_gpio_pin!(PB11, 1, 11, EXTI11);
impl_gpio_pin!(PB12, 1, 12, EXTI12);
impl_gpio_pin!(PB13, 1, 13, EXTI13);
impl_gpio_pin!(PB14, 1, 14, EXTI14);
impl_gpio_pin!(PB15, 1, 15, EXTI15);
impl_gpio_pin!(PC0, 2, 0, EXTI0);
impl_gpio_pin!(PC1, 2, 1, EXTI1);
impl_gpio_pin!(PC2, 2, 2, EXTI2);
impl_gpio_pin!(PC3, 2, 3, EXTI3);
impl_gpio_pin!(PC4, 2, 4, EXTI4);
impl_gpio_pin!(PC5, 2, 5, EXTI5);
impl_gpio_pin!(PC6, 2, 6, EXTI6);
impl_gpio_pin!(PC7, 2, 7, EXTI7);
impl_gpio_pin!(PC8, 2, 8, EXTI8);
impl_gpio_pin!(PC9, 2, 9, EXTI9);
impl_gpio_pin!(PC10, 2, 10, EXTI10);
impl_gpio_pin!(PC11, 2, 11, EXTI11);
impl_gpio_pin!(PC12, 2, 12, EXTI12);
impl_gpio_pin!(PC13, 2, 13, EXTI13);
impl_gpio_pin!(PC14, 2, 14, EXTI14);
impl_gpio_pin!(PC15, 2, 15, EXTI15);
impl_gpio_pin!(PD0, 3, 0, EXTI0);
impl_gpio_pin!(PD1, 3, 1, EXTI1);
impl_gpio_pin!(PD2, 3, 2, EXTI2);
impl_gpio_pin!(PD3, 3, 3, EXTI3);
impl_gpio_pin!(PD4, 3, 4, EXTI4);
impl_gpio_pin!(PD5, 3, 5, EXTI5);
impl_gpio_pin!(PD6, 3, 6, EXTI6);
impl_gpio_pin!(PD7, 3, 7, EXTI7);
impl_gpio_pin!(PD8, 3, 8, EXTI8);
impl_gpio_pin!(PD9, 3, 9, EXTI9);
impl_gpio_pin!(PD10, 3, 10, EXTI10);
impl_gpio_pin!(PD11, 3, 11, EXTI11);
impl_gpio_pin!(PD12, 3, 12, EXTI12);
impl_gpio_pin!(PD13, 3, 13, EXTI13);
impl_gpio_pin!(PD14, 3, 14, EXTI14);
impl_gpio_pin!(PD15, 3, 15, EXTI15);
impl_gpio_pin!(PE0, 4, 0, EXTI0);
impl_gpio_pin!(PE1, 4, 1, EXTI1);
impl_gpio_pin!(PE2, 4, 2, EXTI2);
impl_gpio_pin!(PE3, 4, 3, EXTI3);
impl_gpio_pin!(PE4, 4, 4, EXTI4);
impl_gpio_pin!(PE5, 4, 5, EXTI5);
impl_gpio_pin!(PE6, 4, 6, EXTI6);
impl_gpio_pin!(PE7, 4, 7, EXTI7);
impl_gpio_pin!(PE8, 4, 8, EXTI8);
impl_gpio_pin!(PE9, 4, 9, EXTI9);
impl_gpio_pin!(PE10, 4, 10, EXTI10);
impl_gpio_pin!(PE11, 4, 11, EXTI11);
impl_gpio_pin!(PE12, 4, 12, EXTI12);
impl_gpio_pin!(PE13, 4, 13, EXTI13);
impl_gpio_pin!(PE14, 4, 14, EXTI14);
impl_gpio_pin!(PE15, 4, 15, EXTI15);
impl_gpio_pin!(PF0, 5, 0, EXTI0);
impl_gpio_pin!(PF1, 5, 1, EXTI1);
impl_gpio_pin!(PF2, 5, 2, EXTI2);
impl_gpio_pin!(PF3, 5, 3, EXTI3);
impl_gpio_pin!(PF4, 5, 4, EXTI4);
impl_gpio_pin!(PF5, 5, 5, EXTI5);
impl_gpio_pin!(PF6, 5, 6, EXTI6);
impl_gpio_pin!(PF7, 5, 7, EXTI7);
impl_gpio_pin!(PF8, 5, 8, EXTI8);
impl_gpio_pin!(PF9, 5, 9, EXTI9);
impl_gpio_pin!(PF10, 5, 10, EXTI10);
impl_gpio_pin!(PF11, 5, 11, EXTI11);
impl_gpio_pin!(PF12, 5, 12, EXTI12);
impl_gpio_pin!(PF13, 5, 13, EXTI13);
impl_gpio_pin!(PF14, 5, 14, EXTI14);
impl_gpio_pin!(PF15, 5, 15, EXTI15);
impl_gpio_pin!(PG0, 6, 0, EXTI0);
impl_gpio_pin!(PG1, 6, 1, EXTI1);
impl_gpio_pin!(PG2, 6, 2, EXTI2);
impl_gpio_pin!(PG3, 6, 3, EXTI3);
impl_gpio_pin!(PG4, 6, 4, EXTI4);
impl_gpio_pin!(PG5, 6, 5, EXTI5);
impl_gpio_pin!(PG6, 6, 6, EXTI6);
impl_gpio_pin!(PG7, 6, 7, EXTI7);
impl_gpio_pin!(PG8, 6, 8, EXTI8);
impl_gpio_pin!(PG9, 6, 9, EXTI9);
impl_gpio_pin!(PG10, 6, 10, EXTI10);
impl_gpio_pin!(PG11, 6, 11, EXTI11);
impl_gpio_pin!(PG12, 6, 12, EXTI12);
impl_gpio_pin!(PG13, 6, 13, EXTI13);
impl_gpio_pin!(PG14, 6, 14, EXTI14);
impl_gpio_pin!(PG15, 6, 15, EXTI15);
impl_gpio_pin!(PH0, 7, 0, EXTI0);
impl_gpio_pin!(PH1, 7, 1, EXTI1);
impl_gpio_pin!(PH2, 7, 2, EXTI2);
impl_gpio_pin!(PH3, 7, 3, EXTI3);
impl_gpio_pin!(PH4, 7, 4, EXTI4);
impl_gpio_pin!(PH5, 7, 5, EXTI5);
impl_gpio_pin!(PH6, 7, 6, EXTI6);
impl_gpio_pin!(PH7, 7, 7, EXTI7);
impl_gpio_pin!(PH8, 7, 8, EXTI8);
impl_gpio_pin!(PH9, 7, 9, EXTI9);
impl_gpio_pin!(PH10, 7, 10, EXTI10);
impl_gpio_pin!(PH11, 7, 11, EXTI11);
impl_gpio_pin!(PH12, 7, 12, EXTI12);
impl_gpio_pin!(PH13, 7, 13, EXTI13);
impl_gpio_pin!(PH14, 7, 14, EXTI14);
impl_gpio_pin!(PH15, 7, 15, EXTI15);
impl_gpio_pin!(PI0, 8, 0, EXTI0);
impl_gpio_pin!(PI1, 8, 1, EXTI1);
impl_gpio_pin!(PI2, 8, 2, EXTI2);
impl_gpio_pin!(PI3, 8, 3, EXTI3);
impl_gpio_pin!(PI4, 8, 4, EXTI4);
impl_gpio_pin!(PI5, 8, 5, EXTI5);
impl_gpio_pin!(PI6, 8, 6, EXTI6);
impl_gpio_pin!(PI7, 8, 7, EXTI7);
impl_gpio_pin!(PI8, 8, 8, EXTI8);
impl_gpio_pin!(PI9, 8, 9, EXTI9);
impl_gpio_pin!(PI10, 8, 10, EXTI10);
impl_gpio_pin!(PI11, 8, 11, EXTI11);
impl_gpio_pin!(PI12, 8, 12, EXTI12);
impl_gpio_pin!(PI13, 8, 13, EXTI13);
impl_gpio_pin!(PI14, 8, 14, EXTI14);
impl_gpio_pin!(PI15, 8, 15, EXTI15);
impl_gpio_pin!(PJ0, 9, 0, EXTI0);
impl_gpio_pin!(PJ1, 9, 1, EXTI1);
impl_gpio_pin!(PJ2, 9, 2, EXTI2);
impl_gpio_pin!(PJ3, 9, 3, EXTI3);
impl_gpio_pin!(PJ4, 9, 4, EXTI4);
impl_gpio_pin!(PJ5, 9, 5, EXTI5);
impl_gpio_pin!(PJ6, 9, 6, EXTI6);
impl_gpio_pin!(PJ7, 9, 7, EXTI7);
impl_gpio_pin!(PJ8, 9, 8, EXTI8);
impl_gpio_pin!(PJ9, 9, 9, EXTI9);
impl_gpio_pin!(PJ10, 9, 10, EXTI10);
impl_gpio_pin!(PJ11, 9, 11, EXTI11);
impl_gpio_pin!(PJ12, 9, 12, EXTI12);
impl_gpio_pin!(PJ13, 9, 13, EXTI13);
impl_gpio_pin!(PJ14, 9, 14, EXTI14);
impl_gpio_pin!(PJ15, 9, 15, EXTI15);
impl_gpio_pin!(PK0, 10, 0, EXTI0);
impl_gpio_pin!(PK1, 10, 1, EXTI1);
impl_gpio_pin!(PK2, 10, 2, EXTI2);
impl_gpio_pin!(PK3, 10, 3, EXTI3);
impl_gpio_pin!(PK4, 10, 4, EXTI4);
impl_gpio_pin!(PK5, 10, 5, EXTI5);
impl_gpio_pin!(PK6, 10, 6, EXTI6);
impl_gpio_pin!(PK7, 10, 7, EXTI7);
impl_gpio_pin!(PK8, 10, 8, EXTI8);
impl_gpio_pin!(PK9, 10, 9, EXTI9);
impl_gpio_pin!(PK10, 10, 10, EXTI10);
impl_gpio_pin!(PK11, 10, 11, EXTI11);
impl_gpio_pin!(PK12, 10, 12, EXTI12);
impl_gpio_pin!(PK13, 10, 13, EXTI13);
impl_gpio_pin!(PK14, 10, 14, EXTI14);
impl_gpio_pin!(PK15, 10, 15, EXTI15);
impl_usart!(USART1, 0x40011000);
impl_usart_pin!(USART1, RxPin, PA10, 7);
impl_usart_pin!(USART1, CtsPin, PA11, 7);
impl_usart_pin!(USART1, RtsPin, PA12, 7);
impl_usart_pin!(USART1, CkPin, PA8, 7);
impl_usart_pin!(USART1, TxPin, PA9, 7);
impl_usart_pin!(USART1, TxPin, PB6, 7);
impl_usart_pin!(USART1, RxPin, PB7, 7);
impl_usart!(USART2, 0x40004400);
impl_usart_pin!(USART2, CtsPin, PA0, 7);
impl_usart_pin!(USART2, RtsPin, PA1, 7);
impl_usart_pin!(USART2, TxPin, PA2, 7);
impl_usart_pin!(USART2, RxPin, PA3, 7);
impl_usart_pin!(USART2, CkPin, PA4, 7);
impl_usart_pin!(USART2, CtsPin, PD3, 7);
impl_usart_pin!(USART2, RtsPin, PD4, 7);
impl_usart_pin!(USART2, TxPin, PD5, 7);
impl_usart_pin!(USART2, RxPin, PD6, 7);
impl_usart_pin!(USART2, CkPin, PD7, 7);
impl_usart!(USART3, 0x40004800);
impl_usart_pin!(USART3, TxPin, PB10, 7);
impl_usart_pin!(USART3, RxPin, PB11, 7);
impl_usart_pin!(USART3, CkPin, PB12, 7);
impl_usart_pin!(USART3, CtsPin, PB13, 7);
impl_usart_pin!(USART3, RtsPin, PB14, 7);
impl_usart_pin!(USART3, TxPin, PC10, 7);
impl_usart_pin!(USART3, RxPin, PC11, 7);
impl_usart_pin!(USART3, CkPin, PC12, 7);
impl_usart_pin!(USART3, CkPin, PD10, 7);
impl_usart_pin!(USART3, CtsPin, PD11, 7);
impl_usart_pin!(USART3, RtsPin, PD12, 7);
impl_usart_pin!(USART3, TxPin, PD8, 7);
impl_usart_pin!(USART3, RxPin, PD9, 7);
impl_usart!(USART6, 0x40011400);
impl_usart_pin!(USART6, TxPin, PC6, 8);
impl_usart_pin!(USART6, RxPin, PC7, 8);
impl_usart_pin!(USART6, CkPin, PC8, 8);
impl_usart_pin!(USART6, RtsPin, PG12, 8);
impl_usart_pin!(USART6, CtsPin, PG13, 8);
impl_usart_pin!(USART6, TxPin, PG14, 8);
impl_usart_pin!(USART6, CtsPin, PG15, 8);
impl_usart_pin!(USART6, CkPin, PG7, 8);
impl_usart_pin!(USART6, RtsPin, PG8, 8);
impl_usart_pin!(USART6, RxPin, PG9, 8);
