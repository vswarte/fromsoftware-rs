use std::{fmt, iter::FusedIterator, mem::MaybeUninit, ptr::NonNull};

/// A wrapper type to represent an instance of a type [T] that may be in a
/// well-known "empty" state where its usual guarantees aren't upheld.
///
/// This is similar to [MaybeUninit] in that it ensures that the underlying
/// structure isn't accessed until we're confident it's valid, but it's
/// different in that the memory is actually initialized. It's just initialized
/// to a known pattern that indicates an empty or null value. The specific
/// pattern differs from type to type; individual types implement the [IsEmpty]
/// trait to determine when the structure is valid.
///
/// This is also similar to [Option]. The main difference is that this is usable
/// for complex structs defined in C++, while [Option] is only usable for
/// pointers and Rust structs.
#[repr(transparent)]
// Ideally this would be a union, but transparent unions aren't stable yet, so
// we use the std-defined MaybeUninit instead as a workaround.
pub struct MaybeEmpty<T>(MaybeUninit<T>)
where
    T: IsEmpty;

/// Implement this trait for a type to allow it to be used with [MaybeEmpty].
///
/// ## Safety
///
/// Implementors must guarantee that if [is_empty] returns true for a given
/// `MaybeEmpty<T>`, that value meets all the requirements for a value of type
/// `T`.
pub unsafe trait IsEmpty: Sized {
    /// Returns whether the given [value] is "empty" according to its own
    /// internal logic.
    fn is_empty(value: &MaybeEmpty<Self>) -> bool;
}

impl<T> MaybeEmpty<T>
where
    T: IsEmpty,
{
    /// Returns whether this struct is "empty" according to its own internal
    /// logic.
    pub fn is_empty(&self) -> bool {
        IsEmpty::is_empty(self)
    }

    /// Gets a pointer to the contained value. Reading from this pointer is
    /// unsafe but well-defined, since the underlying memory will either be
    /// a valid [T] *or* match the well-known empty pattern.
    pub fn as_non_null(&self) -> NonNull<T> {
        NonNull::from_ref(self).cast::<T>()
    }

    /// If this isn't empty, returns it. Otherwise, returns `None`.
    pub fn as_option(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            // Safety: IsEmpty guarantees that this is safe.
            Some(unsafe { self.as_non_null().as_ref() })
        }
    }

    /// If this isn't empty, returns it. Otherwise, returns `None`.
    pub fn as_option_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            // Safety: IsEmpty guarantees that this is safe.
            Some(unsafe { self.as_non_null().as_mut() })
        }
    }
}

impl<T> fmt::Debug for MaybeEmpty<T>
where
    T: IsEmpty + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = self.as_option() {
            value.fmt(f)
        } else {
            write!(f, "<empty>")
        }
    }
}

/// An iterator adapter that omits empty elements.
pub struct NonEmptyIter<'a, E, I>(I)
where
    E: IsEmpty + 'a,
    I: Iterator<Item = &'a MaybeEmpty<E>>;

impl<'a, E, I> Iterator for NonEmptyIter<'a, E, I>
where
    E: IsEmpty + 'a,
    I: Iterator<Item = &'a MaybeEmpty<E>>,
{
    type Item = &'a E;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(entry) => {
                    if let Some(entry) = entry.as_option() {
                        return Some(entry);
                    }
                }
                None => return None,
            }
        }
    }
}

impl<'a, E, I> FusedIterator for NonEmptyIter<'a, E, I>
where
    E: IsEmpty + 'a,
    I: Iterator<Item = &'a MaybeEmpty<E>> + FusedIterator,
{
}

/// An extension trait to add the [non_empty] method to iterators of
/// [MaybeEmpty] values.
pub trait NonEmptyIteratorExt<'a, E>: Iterator<Item = &'a MaybeEmpty<E>>
where
    E: IsEmpty + 'a,
    Self: Sized,
{
    /// Filters out any empty values from this iterator.
    fn non_empty(self) -> NonEmptyIter<'a, E, Self>;
}

impl<'a, E, I> NonEmptyIteratorExt<'a, E> for I
where
    E: IsEmpty + 'a,
    I: Iterator<Item = &'a MaybeEmpty<E>> + Sized,
{
    fn non_empty(self) -> NonEmptyIter<'a, E, Self> {
        NonEmptyIter(self)
    }
}

/// A mutable iterator adapter that omits empty elements.
pub struct NonEmptyIterMut<'a, E, I>(I)
where
    E: IsEmpty + 'a,
    I: Iterator<Item = &'a mut MaybeEmpty<E>>;

impl<'a, E, I> Iterator for NonEmptyIterMut<'a, E, I>
where
    E: IsEmpty + 'a,
    I: Iterator<Item = &'a mut MaybeEmpty<E>>,
{
    type Item = &'a mut E;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                Some(entry) => {
                    if let Some(entry) = entry.as_option_mut() {
                        return Some(entry);
                    }
                }
                None => return None,
            }
        }
    }
}

/// An extension trait to add the [non_empty] method to iterators of
/// [MaybeEmpty] values.
pub trait NonEmptyIteratorMutExt<'a, E>: Iterator<Item = &'a mut MaybeEmpty<E>>
where
    E: IsEmpty + 'a,
    Self: Sized,
{
    /// Filters out any empty values from this iterator.
    fn non_empty(self) -> NonEmptyIterMut<'a, E, Self>;
}

impl<'a, E, I> NonEmptyIteratorMutExt<'a, E> for I
where
    E: IsEmpty + 'a,
    I: Iterator<Item = &'a mut MaybeEmpty<E>> + Sized,
{
    fn non_empty(self) -> NonEmptyIterMut<'a, E, Self> {
        NonEmptyIterMut(self)
    }
}
