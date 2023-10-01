---
marp: true
theme: gaia
_class: lead
paginate: true
backgroundColor: #ccc
backgroundImage: url('https://marp.app/assets/hero-background.svg')
footer: "Jönsson Ivar & Lam Johnny"
math: mathjax
size: 16:9
---


<style>
div.twocols {
  margin-top: 35px;
  column-count: 2;
}
div.twocols p:first-child,
div.twocols h1:first-child,
div.twocols h2:first-child,
div.twocols ul:first-child,
div.twocols ul li:first-child,
div.twocols ul li p:first-child {
  margin-top: 0 !important;
}
div.twocols p.break {
  break-before: column;
  margin-top: 0;
}
table {
    font-size: 25px;
}
</style>

![bg left:40% 80%](https://upload.wikimedia.org/wikipedia/commons/thumb/2/2c/Nordic_Semiconductor_Company_Logo.svg/1197px-Nordic_Semiconductor_Company_Logo.svg.png)

# **NRF-HAL**

Native implementation of rtic monotonic timers for the nrf-52840

---

# What is rtic

- Real-Time Interrupt driven Concurrency

<!-- 
In contrast to other RTOS implementations it has no real kernel 
or software task queue. 

It is managed in hardware and therefore has next to no task switching 
overhead, (real real-time)

Fully open-source and used in real embedded projects, 
Some local examples:
- Zparks EV-chargers
- Mobilaris/Widefind positioning tags
-->
- LTU
<!-- 
It started here at ltu as a research project.
-->
- ARM cortex-m
<!-- 
The only practically adoptable version of rtic currently is
cortex-m-rtic

Du to the NVIC ( Nested Vectorized Interrupt Controller )
which is responsible for the hardware managmanent of tasks.

It can be implemented on Risc-v cores
if the cores have a CLIC ( Core Local Interrupt Controller).
-->

---

# What is a HAL

![bg right:40% 80%](images/archmap.png)

- Hardware Abstraction Layer
<!--
It provides safe and tested interfaces for the hardware

Ensures that register accesses are done as the hardware expects.

In rust they typically use the type system to ensure that
singletons of hardware peripherals as well as modes.

Generally speaking a hal should not introduce run time overhead
where ever it is avoidable.
-->
- Implements strategies
<!--
In rust the hals implement strategies defined in the 
embedded-hal crate allowing hardware agnostic drivers.

example of such a strategy
-->

```rust
/// A serial interface
pub trait Serial {
    /// Error type associated to this serial interface
    type Error;

    /// Reads a single byte
    fn read(&mut self) -> nb::Result<u8, Self::Error>;

    /// Writes a single byte
    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error>;
}
```

---

# NRF-HAL

![bg right:60% 80%](images/highlevel.drawio.png)

- Type driven
<!-- 
Each hardware peripheral is represented as a type. 
This allows usage 
-->
- Safe and fast
<!--
Uses the type system to ensure that hardware peripherals
are used in a safe manner.
-->
---

# Timers and RTCs

<div class="twocols">

- Similarities
<!-- 
Both the Timer and RTC are COUNTING peripherals

They have a consistent rate of increase ( monotonic )

-->
- Differences
<!-- 
Describe the table I guess
-->

<p class="break"></p>

|                  | TIMER              | RTC                 |
| ---------------- | ------------------ | ------------------- |
| Base $f$         | 16MHz/1MHz         | ~32 KHz             |
| Counter width    | 32 bit             | 24 bit              |
| $T$              | 19 hours           | 24 days             |
| $\Delta T$ $[s]$ | $63 \cdot 10^{-9}$ | $0.03\cdot 10^{-3}$ |

</div>

---

# Attempt one

- Generic interface for both rtc and timer
<!--
Ease of use and modification

Top level type disconnected from actual implementation
( strategy pattern )
-->
- Compile time guarantee of correctness
<!--
Trait constrained construction
to ensure that freq is valid.
-->
```rust
pub struct MonotonicTimer<T: Instance<RegBlock = TimerRegBlock0>, const FREQ: u32> {
    instance:PhantomData<T>,
}
```

---

# Register access by trait system

<!--
The type level peripheral would be responsible
for register access.

No runtime space cost.

This actually worked and is still present in the final version
-->

```rust
/// A trait that ensures register access for the [`pac`](`crate::pac`)
/// abstractions
pub trait Instance {
    /// The type of the underlying register block
    type RegBlock;
    /// Returns a pointer to the underlying register block
    ///
    /// Allows modification of the registers at a type level rather than
    /// by storing the [`Instance`] at run-time.
    fn reg<'a>() -> &'a Self::RegBlock;
        const DISABLE_INTERRUPT_ON_EMPTY_QUEUE: bool = true;
}
pub trait RtcInstance: Instance<RegBlock = super::RtcRegBlock> {}
pub trait TimerInstance: Instance<RegBlock = super::TimerRegBlock0> {}
```

---

# Compiler bugs and headaches

<!--
When implementing the strategy for the hardware peripherals
the trait system broke...
These two $T$ types are not the same and can't be implemented on the same type.
But the compiler cannot know this.

-->
- Mutually exclusive traits

```rust
impl<T:RtcInstance, const FREQ:u32> Monotonic for MonotonicTimer<T,FREQ>{...}
impl<T:TimerInstance, const FREQ:u32> Monotonic for MonotonicTimer<T,FREQ>{...}
```

- Aha! Bug/ not yet implemented in rustc
<!--
Turns out that this is a well known bug in rustc.

And there is work being done in the compiler to support mutually exclusive traits.
-->
---

# Back to the drawing board

- Two distinct types

<!--
Splitting the implementations to distinct types to help the compiler

Implementation now works great!
-->

```rust
pub struct MonotonicTimer<T: Instance<RegBlock = TimerRegBlock0>, const FREQ: u32> {
    ...
}
pub struct MonotonicRtc<T: Instance<RegBlock = RtcRegBlock>, const FREQ: u32> {
    ...
}
```

- Yet more issues...

---

# Frequency gating construction

- Timer
9 different prescalers
- Rtc
$2^{12}-1$ prescalers

<!--
Turns out that our initial idea of gating the construction
of the abstraction is somewhat problematic

While it is reasonable to do this for the timer
it is not reasonable to have $2^{12}$ different new functions
for the rtc abstraction.

The solution to this problem is to at runtime, specifically 
init time the rtc throws an error if the frequency is invalid.
-->
---

# Final interface

##### Timer

<!--
Simple interface for any timer

Compile time guaranteed correctness
-->
```rust
#[monotonic(binds = TIMER3, default = true)]
type MyMono = MonotonicTimer<TIMER3,16_000_000>;
/// new only exists on valid frequencies
let mono = MyMono::new(ctx.device.TIMER3);
```

---

# Final interface

##### Rtc

<!--
This interface is equally simple,
but it cannot guarantee that the rtc is
correct at compile time. 

It can, however guarantee that the rtc is
correct when the init function is completed.
-->
```rust
#[monotonic(binds = RTC0, default = true)]
type MyMono = MonotonicRtc<RTC0, 32_768>;
let clocks = hal::clocks::Clocks::new(cx.device.CLOCK);
let clocks = clocks.start_lfclk();
/// Will throw error if freq is invalid
let mono = MyMono::new(cx.device.RTC0,&clocks).unwrap();
```

---

# References and futher reading

- rtic <https://rtic.rs/2/book/en/>
- nrf-hal <https://docs.rs/nrf52840-hal/latest/nrf52840_hal/>
- embedded-hal <https://docs.rs/embedded-hal/latest/embedded_hal/>
