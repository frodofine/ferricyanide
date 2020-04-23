use std::cell::RefCell;
use std::rc::Rc;

mod store;
pub use self::store::*;

/// Used to instantiate our application
pub struct App {
    pub store: Rc<RefCell<Store>>,
}

impl App {
    /// Create a new instance of the ferricyanide renderer
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            store: Rc::new(RefCell::new(Store::new(width, height))),
        }
    }
}
