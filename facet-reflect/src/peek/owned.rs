use crate::trace;
use core::marker::PhantomData;
use facet_core::{PtrMut, Shape};

use super::Peek;

/// Owns a converted serialization proxy while retaining any source borrow.
///
/// Keep this guard alive until serialization of the proxy is complete.
/// Construction rejects proxy shapes whose lifetimes cannot safely shrink.
pub struct OwnedPeek<'mem> {
    pub(crate) data: PtrMut,
    pub(crate) shape: &'static Shape,
    pub(crate) _phantom: PhantomData<&'mem ()>,
}

impl<'mem> OwnedPeek<'mem> {
    /// Returns the proxy's reflected shape.
    pub const fn shape(&self) -> &'static Shape {
        self.shape
    }

    /// Inspect the proxy within a callback that cannot export its borrows.
    ///
    /// A proxy may borrow its source. Its erased shape does not establish a
    /// concrete lifetime for typed extraction, so the callback receives a fresh
    /// invariant Facet lifetime. Construction has already checked that this
    /// proxy is covariant or bivariant, so shrinking to that lifetime is safe.
    ///
    /// Return owned output from the callback when retaining its result.
    /// A borrowed path proxy must never become a purported static string:
    ///
    /// ```compile_fail
    /// use facet_reflect::Peek;
    /// use std::{borrow::Cow, path::PathBuf};
    ///
    /// let escaped: Cow<'static, str> = {
    ///     let path = PathBuf::from("temporary workspace");
    ///     let proxy = Peek::new(&path)
    ///         .custom_serialization_from_shape().unwrap().unwrap();
    ///     proxy.with_peek(|value| value.get::<Cow<'static, str>>().unwrap().clone())
    /// };
    /// println!("{escaped}");
    /// ```
    pub fn with_peek<R>(&self, visit: impl for<'proxy> FnOnce(Peek<'proxy, 'proxy>) -> R) -> R {
        // SAFETY: this guard retains the initialized proxy and its source borrow.
        // Construction checked that the proxy shape permits lifetime shrinking.
        // The higher-ranked callback cannot export the freshly branded memory
        // or Facet lifetime, or substitute a chosen lifetime such as 'static.
        visit(unsafe { Peek::unchecked_new(self.data.as_const(), self.shape) })
    }
}

impl<'mem> Drop for OwnedPeek<'mem> {
    fn drop(&mut self) {
        trace!("Dropping owned peek of shape '{}'", self.shape);
        unsafe { self.shape.call_drop_in_place(self.data) };
        let _ = unsafe { self.shape.deallocate_mut(self.data) };
    }
}
