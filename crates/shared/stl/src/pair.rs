#[repr(C)]
/// Implementation of MSVC C++ [`std::pair`]
///
/// [`std::pair`]: https://en.cppreference.com/w/cpp/utility/pair.html
pub struct Pair<K, V> {
    pub first: K,
    pub second: V,
}

impl<K, V> From<Pair<K, V>> for (K, V) {
    fn from(p: Pair<K, V>) -> Self {
        (p.first, p.second)
    }
}

impl<K, V> From<(K, V)> for Pair<K, V> {
    fn from((first, second): (K, V)) -> Self {
        Pair { first, second }
    }
}

impl<'a, K, V> From<&'a Pair<K, V>> for (&'a K, &'a V) {
    fn from(p: &'a Pair<K, V>) -> Self {
        (&p.first, &p.second)
    }
}
