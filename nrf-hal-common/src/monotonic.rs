trait MonotonicInstance {
    impl<T: Mono, const FREQ: u32> Monotonic for MonotonicTimer<T, FREQ> {
        type Instant = fugit::TimerInstantU32<FREQ>;
        type Duration = fugit::TimerDurationU32<FREQ>;
    
        fn now(&mut self) -> Self::Instant {
            unsafe { T::now(&mut self.overflow) }
        }
    
        fn set_compare(&mut self, instant: Self::Instant) {
            unsafe {
                T::set_compare(instant, &mut self.overflow);
            }
        }
    
        fn clear_compare_flag(&mut self) {
            unsafe {
                T::clear_compare_flag();
            }
        }
    
        fn zero() -> Self::Instant {
            unsafe { T::zero() }
        }
    
        unsafe fn reset(&mut self) {
            T::reset()
        }
    }
    }