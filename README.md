# `with-cell`

A cell-like wrapper which provides a [`with`](WithCell::with)
and [`map`](WithCell::map) method.

It makes it more convenient to use mutable data structures in a shared manner
without the overhead of `RefCell`.

## Why?

Ever written code like this?

```should_panic
use core::cell::RefCell;

let vec = RefCell::new(vec![1, 2, 3]);
if let Some(x) = vec.borrow_mut().pop() {
    vec.borrow_mut().push(x); // ¡ay caramba!
}
```

Annoying, isn't it? Easy enough to work around but also easy to forget.

## Example

```rust
use with_cell::WithCell;

let vec = WithCell::new(vec![]);
vec.with(|v| v.push(1337));
```

## How it works

The API is:

```ignore
impl<T> WithCell<T>
where
    T: Default,
{
    pub fn with<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R;

    pub fn map<F>(&self, f: F)
    where
        F: FnOnce(T) -> T;
}
```

When [`with`](WithCell::with) is called,
the original value is replaced with a stub `Default` value.
After the closure finishes, it is replaced with the original value again.

This does require two extra memory copies in the worst case,
which might be suboptimal for large structures.
The copy might be avoided if the compiler can prove the function does not panic.

[`map`](WithCell::map) is very similar to [`Cell::update`],
except it replaces the inner value with a stub.

Care must be taken when dealing with panics: the stub will remain in place!

[cell update]: https://doc.rust-lang.org/stable/std/cell/struct.Cell.html#method.update
