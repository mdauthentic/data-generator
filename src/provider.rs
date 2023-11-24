use crate::model::Value;
use std::fmt::{Debug, Error, Formatter};

pub trait Provider: ProviderClone {
    /// A call to the `next()` function is meant to generate the next random value for the provider
    /// This function is implemented by all struct that implement the `Provider` trait.
    fn next(&mut self) -> Value;
}

/// The clone implementation part of this code is copied from:
/// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
/// I can also use https://github.com/dtolnay/dyn-clone as a replacement
pub trait ProviderClone {
    fn clone_box(&self) -> Box<dyn Provider>;
}

impl<T> ProviderClone for T
where
    T: 'static + Provider + Clone,
{
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

pub fn type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}

impl Debug for dyn Provider {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(self.to_string().as_str())
    }
}

impl ToString for dyn Provider {
    fn to_string(&self) -> String {
        format!("Provider[{:?}]", type_of(&self))
    }
}
