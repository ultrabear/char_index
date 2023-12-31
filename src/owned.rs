//! Module containing [`OwnedIndexedChars`] and its trait implementations

use alloc::string::String;
use core::{
    borrow::Borrow,
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    fmt,
    hash::{Hash, Hasher},
    ops::Deref,
};

use crate::IndexedCharsInner;

/// A string whose char indices have been cached for ~O(1) char lookup. Owned variant.
///
/// This structure allocates 1 additional bytes per unicode scalar value,
/// which in the case of ascii will only use 2 total bytes for a
/// single char (as opposed to the 4 bytes required in `Vec<char>`).
///
/// As the number of non ascii characters increases, the data density will worsen, until the potential worst case of 5 bytes per character.
///
/// The internal representation of this type allows for up to 255 bytes of non ascii unicode chars before an internal rollover occurs (thus tending the complexity towards O(log n)), this is the tradeoff made to reduce memory usage. See the section [`How it Works`](index.html#how-it-works) for details on why char indexing worst case is O(log n), and why in practical cases it appears to be O(1).
///
/// This type mimics a `String` with its trait impls, including `Debug`, `Display`, `PartialEq` with `&str` `PartialOrd` with `&str`, `Hash`, and `AsRef`/`Borrow`.
pub struct OwnedIndexedChars {
    /// Backing string allocation
    buf: String,
    /// Char offsets index
    inner: IndexedCharsInner,
}

impl OwnedIndexedChars {
    /// Constructs a new [`OwnedIndexedChars`] instance from a [`String`]. This is O(n), but the cost should only be paid once ideally.
    ///
    /// # Examples
    /// ```rust
    /// # use char_index::OwnedIndexedChars;
    /// let index = OwnedIndexedChars::new(String::from("foo"));
    ///
    /// // we can still access str methods through deref
    /// _ = index.chars();
    /// # assert_eq!(index.get_char(0), Some('f'));
    /// ```
    #[must_use]
    pub fn new(s: String) -> Self {
        let inner = IndexedCharsInner::new(&s);

        Self { buf: s, inner }
    }

    /// Indexes into the backing string to retrieve the nth codepoint.
    ///
    /// This operation has an average case of O(1), and a worst case of O(log n).
    ///
    /// # Examples
    /// ```rust
    /// # use char_index::OwnedIndexedChars;
    /// let s = OwnedIndexedChars::new(String::from("foo"));
    ///
    /// assert_eq!(s.get_char(1), Some('o'));
    /// ```
    #[must_use]
    pub fn get_char(&self, index: usize) -> Option<char> {
        self.inner.get_char(&self.buf, index)
    }

    /// Returns the number of chars present in the backing string, this operation is free thanks to
    /// how [`OwnedIndexedChars`] is constructed
    #[must_use]
    pub fn char_count(&self) -> usize {
        self.inner.char_count(&self.buf)
    }

    /// Drops index data and returns backing `String` allocation.
    #[must_use]
    pub fn into_string(self) -> String {
        self.buf
    }

    /// Returns a reference to the backing `String` allocation.
    ///
    /// Generally you don't want this, and should instead use [`as_str`][OwnedIndexedChars::as_str] or [`Deref`]
    #[must_use]
    pub fn as_string(&self) -> &String {
        &self.buf
    }

    /// Returns a reference to the backing `String` as a `&str`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.buf.as_str()
    }
}

// The following lines are all trait implementations made to mirror what str does, and be compatible with str

impl Deref for OwnedIndexedChars {
    type Target = str;

    fn deref(&self) -> &str {
        self.buf.as_str()
    }
}

impl AsRef<str> for OwnedIndexedChars {
    fn as_ref(&self) -> &str {
        self
    }
}

impl Borrow<str> for OwnedIndexedChars {
    fn borrow(&self) -> &str {
        self
    }
}

impl fmt::Debug for OwnedIndexedChars {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <String as fmt::Debug>::fmt(&self.buf, f)
    }
}

impl fmt::Display for OwnedIndexedChars {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <String as fmt::Display>::fmt(&self.buf, f)
    }
}

impl Eq for OwnedIndexedChars {}

impl PartialEq for OwnedIndexedChars {
    fn eq(&self, other: &Self) -> bool {
        self.buf.eq(&other.buf)
    }
}

impl PartialEq<str> for OwnedIndexedChars {
    fn eq(&self, other: &str) -> bool {
        self.buf.eq(other)
    }
}

impl PartialEq<OwnedIndexedChars> for str {
    fn eq(&self, other: &OwnedIndexedChars) -> bool {
        self.eq(&other.buf)
    }
}

impl Ord for OwnedIndexedChars {
    fn cmp(&self, other: &Self) -> Ordering {
        self.buf.cmp(&other.buf)
    }
}

impl PartialOrd for OwnedIndexedChars {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<str> for OwnedIndexedChars {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        Some((*self.buf).cmp(other))
    }
}

impl PartialOrd<OwnedIndexedChars> for str {
    fn partial_cmp(&self, other: &OwnedIndexedChars) -> Option<Ordering> {
        Some(self.cmp(&other.buf))
    }
}

impl Hash for OwnedIndexedChars {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.buf.hash(state);
    }
}
