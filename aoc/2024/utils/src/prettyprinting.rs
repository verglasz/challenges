use core::fmt;
use std::fmt::{Debug, Display, Formatter};

pub struct DisplayAsDebug<T>(pub T);

impl<T: Display> Debug for DisplayAsDebug<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

pub struct DebugAsDisplay<T>(pub T);

impl<T: Debug> Display for DebugAsDisplay<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}
