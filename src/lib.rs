#![no_std]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)] // :)
#![deny(missing_docs)]

use core::{cell::Cell, fmt, ops};

/// `Cell`-like container for making shared structures with mutable methods more convenient to use.
///
/// Unlike [`Cell`], this wrapper does not require [`Copy`] in the general case.
/// Instead, it relies on [`Default`].
///
/// Internally, it uses [`Cell`].
#[derive(Default)]
pub struct WithCell<T>(Cell<T>);

impl<T> WithCell<T> {
    /// Create a new `WithCell` containing the given value.
    pub const fn new(value: T) -> Self {
        Self(Cell::new(value))
    }
}

impl<T> WithCell<T>
where
    T: Default,
{
    /// Perform an operation on the contained value.
    ///
    /// This takes the value out of the cell and replaces it with its [`Default`] variant.
    /// This value is then passed by reference to `f`.
    /// When `f` returns, the value is put back in the cell,
    /// discarding the stub variant.
    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut v = self.0.take();
        let ret = (f)(&mut v);
        self.0.set(v);
        ret
    }

    /// Perform an operation on the contained value.
    ///
    /// This takes the value out of the cell and replaces it with its [`Default`] variant.
    /// This value is then passed by reference to `f`.
    /// When `f` returns, the value is put back in the cell,
    /// discarding the stub variant.
    ///
    /// This method is almost identical to [`with`](Self::with),
    /// except it returns a reference to `self`.
    pub fn inspect<F>(&self, f: F) -> &Self
    where
        F: FnOnce(&mut T),
    {
        self.with(f);
        self
    }

    /// Perform an operation on the contained value.
    ///
    /// Like [`with`](Self::with), it replaces the value with its [`Default`] variant.
    /// This value is then passed by value to `f`.
    /// The value returned from `f` is put in the cell,
    /// discarding the stub variant.
    ///
    /// This function can be chained:
    /// ```
    /// let abcd = with_cell::WithCell::new(String::new());
    ///
    /// abcd.map(|x| x + "a")
    ///     .map(|x| x + "b")
    ///     .map(|x| x + "cd")
    ///     .map(|x| dbg!(x))
    ///     .with(|x| x.clear());
    /// ```
    pub fn map<F>(&self, f: F) -> &Self
    where
        F: FnOnce(T) -> T,
    {
        self.0.set((f)(self.0.take()));
        self
    }
}

impl<T> ops::Deref for WithCell<T> {
    type Target = Cell<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> ops::DerefMut for WithCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> fmt::Debug for WithCell<T>
where
    T: Default + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.with(|x| x.fmt(f))
    }
}

impl<T> Clone for WithCell<T>
where
    T: Default + Clone,
{
    fn clone(&self) -> Self {
        self.with(|x| x.clone()).into()
    }
}

impl<T> From<T> for WithCell<T> {
    fn from(x: T) -> Self {
        Self(x.into())
    }
}
