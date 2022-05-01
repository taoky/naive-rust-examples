use std::cell::UnsafeCell;

pub struct Singleton {
    pub state: u64,
}

impl Singleton {
    pub fn new(state: u64) -> Self {
        Singleton { state }
    }

    pub fn next_state(&self) -> u64 {
        (self.state.wrapping_mul(self.state)) ^ self.state
    }

    pub fn get(&mut self) -> u64 {
        self.state = self.next_state();
        self.state
    }

    pub fn get_unsafe(&self) -> u64 {
        let state = self.next_state();
        let sptr = &self.state as *const u64 as *mut u64;
        unsafe {
            std::ptr::write(sptr, state);
        }
        state
    }
}

pub struct SingletonWithUnsafeCell {
    pub state: UnsafeCell<u64>,
}

impl SingletonWithUnsafeCell {
    pub fn new(state: u64) -> Self {
        SingletonWithUnsafeCell {
            state: UnsafeCell::new(state),
        }
    }

    pub fn next_state(&self, state: u64) -> u64 {
        (state.wrapping_mul(state)) ^ state
    }

    pub fn get(&self) -> u64 {
        unsafe {
            let state = &mut *self.state.get();
            *state = self.next_state(*state);
            *state
        }
    }
}
