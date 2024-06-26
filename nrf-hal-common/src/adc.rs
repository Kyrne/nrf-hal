//! API for the Analog to Digital converter.

use crate::{
    gpio::{Floating, Input},
    pac::{
        adc::config::{INPSEL_A as InputSelection, REFSEL_A as Reference, RES_A as Resolution},
        ADC,
    },
};

#[cfg(feature = "embedded-hal-02")]
pub trait Channel: embedded_hal_02::adc::Channel<Adc, ID = u8> {}

#[cfg(not(feature = "embedded-hal-02"))]
pub trait Channel {
    fn channel() -> u8;
}

pub struct Adc(ADC);

impl Adc {
    pub fn new(adc: ADC, config: AdcConfig) -> Self {
        while adc.busy.read().busy().is_busy() {}

        adc.config.write(|w| {
            let w1 = match config.resolution {
                Resolution::_8BIT => w.res()._8bit(),
                Resolution::_9BIT => w.res()._9bit(),
                Resolution::_10BIT => w.res()._10bit(),
            };

            let w2 = match config.input_selection {
                InputSelection::ANALOG_INPUT_NO_PRESCALING => {
                    w1.inpsel().analog_input_no_prescaling()
                }
                InputSelection::ANALOG_INPUT_TWO_THIRDS_PRESCALING => {
                    w1.inpsel().analog_input_two_thirds_prescaling()
                }
                InputSelection::ANALOG_INPUT_ONE_THIRD_PRESCALING => {
                    w1.inpsel().analog_input_one_third_prescaling()
                }
                InputSelection::SUPPLY_TWO_THIRDS_PRESCALING => {
                    w1.inpsel().supply_two_thirds_prescaling()
                }
                InputSelection::SUPPLY_ONE_THIRD_PRESCALING => {
                    w1.inpsel().supply_one_third_prescaling()
                }
            };

            let w3 = match config.reference {
                Reference::VBG => w2.refsel().vbg(),
                Reference::EXTERNAL => w2.refsel().external(),
                Reference::SUPPLY_ONE_HALF_PRESCALING => w2.refsel().supply_one_half_prescaling(),
                Reference::SUPPLY_ONE_THIRD_PRESCALING => w2.refsel().supply_one_third_prescaling(),
            };

            w3
        });

        adc.enable.write(|w| w.enable().enabled());

        Self(adc)
    }

    /// Samples the given channel of the ADC and returns the value read.
    pub fn read_channel<PIN: Channel>(&mut self, _pin: &PIN) -> i16 {
        let original_inpsel = self.0.config.read().inpsel();
        match PIN::channel() {
            0 => self.0.config.modify(|_, w| w.psel().analog_input0()),
            1 => self.0.config.modify(|_, w| w.psel().analog_input1()),
            2 => self.0.config.modify(|_, w| w.psel().analog_input2()),
            3 => self.0.config.modify(|_, w| w.psel().analog_input3()),
            4 => self.0.config.modify(|_, w| w.psel().analog_input4()),
            5 => self.0.config.modify(|_, w| w.psel().analog_input5()),
            6 => self.0.config.modify(|_, w| w.psel().analog_input6()),
            7 => self.0.config.modify(|_, w| w.psel().analog_input7()),
            8 => self
                .0
                .config
                .modify(|_, w| w.inpsel().supply_one_third_prescaling()),
            9 => self
                .0
                .config
                .modify(|_, w| w.inpsel().supply_two_thirds_prescaling()),
            // This can never happen with the `Channel` implementations provided, as the only analog
            // pins have already been covered.
            _ => panic!("Invalid channel"),
        }

        self.0.events_end.write(|w| unsafe { w.bits(0) });
        self.0.tasks_start.write(|w| unsafe { w.bits(1) });

        while self.0.events_end.read().bits() == 0 {}

        self.0.events_end.write(|w| unsafe { w.bits(0) });
        // Restore original input selection
        self.0
            .config
            .modify(|_, w| w.inpsel().variant(original_inpsel.variant().unwrap()));

        // Max resolution is 10 bits so casting is always safe
        self.0.result.read().result().bits() as i16
    }
}

pub struct AdcConfig {
    pub resolution: Resolution,
    pub input_selection: InputSelection,
    pub reference: Reference,
}

// 0 volts reads as 0, VDD volts reads as 2^10.
impl Default for AdcConfig {
    fn default() -> Self {
        Self {
            resolution: Resolution::_10BIT,
            input_selection: InputSelection::ANALOG_INPUT_ONE_THIRD_PRESCALING,
            reference: Reference::SUPPLY_ONE_THIRD_PRESCALING,
        }
    }
}

#[cfg(feature = "embedded-hal-02")]
impl<PIN> embedded_hal_02::adc::OneShot<Adc, i16, PIN> for Adc
where
    PIN: Channel,
{
    type Error = ();

    fn read(&mut self, pin: &mut PIN) -> nb::Result<i16, Self::Error> {
        Ok(self.read_channel(pin))
    }
}

macro_rules! channel_mappings {
    ($($n:expr => $pin:path),*) => {
        $(
            #[cfg(feature = "embedded-hal-02")]
            impl embedded_hal_02::adc::Channel<Adc> for $pin {
                type ID = u8;

                fn channel() -> u8 {
                    $n
                }
            }

            impl Channel for $pin {
                #[cfg(not(feature = "embedded-hal-02"))]
                fn channel() -> u8 {
                    $n
                }
            }
        )*
    };
}

channel_mappings! {
    0 => crate::gpio::p0::P0_26<Input<Floating>>,
    1 => crate::gpio::p0::P0_27<Input<Floating>>,
    2 => crate::gpio::p0::P0_01<Input<Floating>>,
    3 => crate::gpio::p0::P0_02<Input<Floating>>,
    4 => crate::gpio::p0::P0_03<Input<Floating>>,
    5 => crate::gpio::p0::P0_04<Input<Floating>>,
    6 => crate::gpio::p0::P0_05<Input<Floating>>,
    7 => crate::gpio::p0::P0_06<Input<Floating>>,
    8 => crate::adc::InternalVddOneThird,
    9 => crate::adc::InternalVddTwoThirds
}

pub struct InternalVddOneThird;
pub struct InternalVddTwoThirds;
