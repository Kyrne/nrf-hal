use crate::ppi::Event;

// Event impls
//
// To reproduce, in the pac crate, search
//   `rg 'type EVENTS_.*crate::Reg' --type rust`
// Find (regex):
//   `^src/(.*)\.rs:pub type (.*) = .*$`
// Replace (regex):
//   `impl Event for crate::pac::$1::$2 { }`
impl Event for crate::pac::rng::EVENTS_VALRDY {}
impl Event for crate::pac::timer0::EVENTS_COMPARE {}
impl Event for crate::pac::spis0::EVENTS_END {}
impl Event for crate::pac::spis0::EVENTS_ENDRX {}
impl Event for crate::pac::spis0::EVENTS_ACQUIRED {}
impl Event for crate::pac::gpiote::EVENTS_IN {}
impl Event for crate::pac::gpiote::EVENTS_PORT {}
impl Event for crate::pac::uart0::EVENTS_CTS {}
impl Event for crate::pac::uart0::EVENTS_NCTS {}
impl Event for crate::pac::uart0::EVENTS_RXDRDY {}
impl Event for crate::pac::uart0::EVENTS_TXDRDY {}
impl Event for crate::pac::uart0::EVENTS_ERROR {}
impl Event for crate::pac::uart0::EVENTS_RXTO {}
impl Event for crate::pac::clock::EVENTS_HFCLKSTARTED {}
impl Event for crate::pac::clock::EVENTS_LFCLKSTARTED {}
impl Event for crate::pac::clock::EVENTS_DONE {}
impl Event for crate::pac::clock::EVENTS_CTTO {}
impl Event for crate::pac::power::EVENTS_POFWARN {}
impl Event for crate::pac::power::EVENTS_SLEEPENTER {}
impl Event for crate::pac::power::EVENTS_SLEEPEXIT {}
impl Event for crate::pac::spim0::EVENTS_STOPPED {}
impl Event for crate::pac::spim0::EVENTS_ENDRX {}
impl Event for crate::pac::spim0::EVENTS_END {}
impl Event for crate::pac::spim0::EVENTS_ENDTX {}
impl Event for crate::pac::spim0::EVENTS_STARTED {}
impl Event for crate::pac::spi0::EVENTS_READY {}
impl Event for crate::pac::twim0::EVENTS_STOPPED {}
impl Event for crate::pac::twim0::EVENTS_ERROR {}
impl Event for crate::pac::twim0::EVENTS_SUSPENDED {}
impl Event for crate::pac::twim0::EVENTS_RXSTARTED {}
impl Event for crate::pac::twim0::EVENTS_TXSTARTED {}
impl Event for crate::pac::twim0::EVENTS_LASTRX {}
impl Event for crate::pac::twim0::EVENTS_LASTTX {}
impl Event for crate::pac::twi0::EVENTS_STOPPED {}
impl Event for crate::pac::twi0::EVENTS_RXDREADY {}
impl Event for crate::pac::twi0::EVENTS_TXDSENT {}
impl Event for crate::pac::twi0::EVENTS_ERROR {}
impl Event for crate::pac::twi0::EVENTS_BB {}
impl Event for crate::pac::twi0::EVENTS_SUSPENDED {}
impl Event for crate::pac::egu0::EVENTS_TRIGGERED {}
impl Event for crate::pac::ecb::EVENTS_ENDECB {}
impl Event for crate::pac::ecb::EVENTS_ERRORECB {}
impl Event for crate::pac::rtc0::EVENTS_TICK {}
impl Event for crate::pac::rtc0::EVENTS_OVRFLW {}
impl Event for crate::pac::rtc0::EVENTS_COMPARE {}
impl Event for crate::pac::wdt::EVENTS_TIMEOUT {}
impl Event for crate::pac::radio::EVENTS_READY {}
impl Event for crate::pac::radio::EVENTS_ADDRESS {}
impl Event for crate::pac::radio::EVENTS_PAYLOAD {}
impl Event for crate::pac::radio::EVENTS_END {}
impl Event for crate::pac::radio::EVENTS_DISABLED {}
impl Event for crate::pac::radio::EVENTS_DEVMATCH {}
impl Event for crate::pac::radio::EVENTS_DEVMISS {}
impl Event for crate::pac::radio::EVENTS_RSSIEND {}
impl Event for crate::pac::radio::EVENTS_BCMATCH {}
impl Event for crate::pac::radio::EVENTS_CRCOK {}
impl Event for crate::pac::radio::EVENTS_CRCERROR {}
impl Event for crate::pac::temp::EVENTS_DATARDY {}
impl Event for crate::pac::ccm::EVENTS_ENDKSGEN {}
impl Event for crate::pac::ccm::EVENTS_ENDCRYPT {}
impl Event for crate::pac::ccm::EVENTS_ERROR {}
impl Event for crate::pac::twis0::EVENTS_STOPPED {}
impl Event for crate::pac::twis0::EVENTS_ERROR {}
impl Event for crate::pac::twis0::EVENTS_RXSTARTED {}
impl Event for crate::pac::twis0::EVENTS_TXSTARTED {}
impl Event for crate::pac::twis0::EVENTS_WRITE {}
impl Event for crate::pac::twis0::EVENTS_READ {}
impl Event for crate::pac::uarte0::EVENTS_CTS {}
impl Event for crate::pac::uarte0::EVENTS_NCTS {}
impl Event for crate::pac::uarte0::EVENTS_RXDRDY {}
impl Event for crate::pac::uarte0::EVENTS_ENDRX {}
impl Event for crate::pac::uarte0::EVENTS_TXDRDY {}
impl Event for crate::pac::uarte0::EVENTS_ENDTX {}
impl Event for crate::pac::uarte0::EVENTS_ERROR {}
impl Event for crate::pac::uarte0::EVENTS_RXTO {}
impl Event for crate::pac::uarte0::EVENTS_RXSTARTED {}
impl Event for crate::pac::uarte0::EVENTS_TXSTARTED {}
impl Event for crate::pac::uarte0::EVENTS_TXSTOPPED {}
impl Event for crate::pac::qdec::EVENTS_SAMPLERDY {}
impl Event for crate::pac::qdec::EVENTS_REPORTRDY {}
impl Event for crate::pac::qdec::EVENTS_ACCOF {}
impl Event for crate::pac::qdec::EVENTS_DBLRDY {}
impl Event for crate::pac::qdec::EVENTS_STOPPED {}
impl Event for crate::pac::aar::EVENTS_END {}
impl Event for crate::pac::aar::EVENTS_RESOLVED {}
impl Event for crate::pac::aar::EVENTS_NOTRESOLVED {}
impl Event for crate::pac::saadc::EVENTS_STARTED {}
impl Event for crate::pac::saadc::EVENTS_END {}
impl Event for crate::pac::saadc::EVENTS_DONE {}
impl Event for crate::pac::saadc::EVENTS_RESULTDONE {}
impl Event for crate::pac::saadc::EVENTS_CALIBRATEDONE {}
impl Event for crate::pac::saadc::EVENTS_STOPPED {}
