use std::collections::HashMap;
use std::hash::Hash;

/// A map that's intended to contain `StatefulDebugDisplay` states associated
/// with game objects.
///
/// This has two properties that make it particularly suited to this purpose:
///
/// * It always returns a value for any key; if it doesn't have one yet, it
///   creates a default value and inserts it. In other words, it always runs the
///   equivalent of `hash_map.entry(...).or_default()`.
///
/// * It can track all accesses within a given frame using
///   [track_reads](Self::track_reads), then remove all values that weren't
///   accessed using [remove_unread](Self::remove_unread). This ensures it
///   doesn't leak memory as game objectsenter and leave the game world.
pub struct StateMap<K, V>
where
    K: Eq + Hash,
    V: Default,
{
    /// A map from keys to values, as well as the `track_reads_counter` value at
    /// the point the values were last accessed.
    inner: HashMap<K, (V, u64)>,

    /// A counter indicating the number of times
    /// [track_reads](Self::track_reads) has been called. Used to determine
    /// which values have been read since the last call.
    track_reads_counter: u64,
}

impl<K, V> StateMap<K, V>
where
    K: Eq + Hash,
    V: Default,
{
    /// Returns the value at `key` in the map, inserting `V::default()` if none
    /// exists yet.
    pub fn get(&mut self, key: K) -> &mut V {
        let entry = self.inner.entry(key).or_default();
        entry.1 = self.track_reads_counter;
        &mut entry.0
    }

    /// Tracks all reads on this map until the next call to
    /// [remove_unread](Self::remove_unread), at which point all values whose
    /// keys weren't read in that period are removed from the map.
    ///
    /// If this is called when another call is already active, all
    /// previously-tracked reads are considered untracked.
    pub fn track_reads(&mut self) {
        self.track_reads_counter += 1
    }

    /// Removes all entries from the map that weren't read since the last call
    /// to [track_reads](Self::track_reads).
    pub fn remove_unread(&mut self) {
        self.inner
            .retain(|_, (_, c)| *c == self.track_reads_counter);
    }
}

impl<K, V> Default for StateMap<K, V>
where
    K: Eq + Hash,
    V: Default,
{
    fn default() -> Self {
        StateMap {
            inner: HashMap::new(),
            track_reads_counter: 0,
        }
    }
}
