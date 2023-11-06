
use std::fmt::{Debug, Error, Formatter};
use crate::model::Value;

pub trait Provider: ProviderClone {
    fn next(&mut self) -> Value;
}

/// The clone implementation part of this code is copied from:
/// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
/// I can also use https://github.com/dtolnay/dyn-clone as a replacement
trait ProviderClone {
    fn clone_box(&self) -> Box<dyn Provider>;
}

impl<T> ProviderClone for T
    where T: 'static + Provider + Clone {
    fn clone_box(&self) -> Box<dyn Provider> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Provider> {
    fn clone(&self) -> Box<dyn Provider> {
        self.clone_box()
    }
}

/// for logging purposes
pub fn print_type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

impl Debug for dyn Provider {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(self.to_string().as_str())
    }
}

impl ToString for dyn Provider {
    fn to_string(&self) -> String {
        format!("Provider[{:?}]", print_type_of(&self))
    }
}