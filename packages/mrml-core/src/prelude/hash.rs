use std::{
    hash::Hash,
    iter::FromIterator,
    ops::{Deref, DerefMut},
};

#[cfg(feature = "orderedmap")]
use indexmap::{IndexMap, IndexSet};
#[cfg(feature = "orderedmap")]
use rustc_hash::FxHasher;
use serde::{Deserialize, Serialize};

#[cfg(feature = "orderedmap")]
type HashImpl = std::hash::BuildHasherDefault<FxHasher>;

#[cfg(feature = "orderedmap")]
pub type MapImpl<K, V> = IndexMap<K, V, HashImpl>;

#[cfg(not(feature = "orderedmap"))]
pub type MapImpl<K, V> = std::collections::HashMap<K, V>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Map<K, V>(MapImpl<K, V>)
where
    K: Hash + Eq;

impl<K, V> Map<K, V>
where
    K: Hash + Eq,
{
    #[cfg(feature = "orderedmap")]
    pub fn new() -> Self {
        Map(MapImpl::with_hasher(HashImpl::default()))
    }

    #[cfg(not(feature = "orderedmap"))]
    pub fn new() -> Self {
        Map(MapImpl::new())
    }
}

impl<K, V> Deref for Map<K, V>
where
    K: Hash + Eq,
{
    type Target = MapImpl<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> DerefMut for Map<K, V>
where
    K: Hash + Eq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V> FromIterator<(K, V)> for Map<K, V>
where
    K: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Map(MapImpl::from_iter(iter))
    }
}

#[cfg(feature = "orderedmap")]
pub type SetImpl<V> = IndexSet<V, HashImpl>;

#[cfg(not(feature = "orderedmap"))]
pub type SetImpl<V> = std::collections::HashSet<V>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Set<V>(SetImpl<V>)
where
    V: Hash + Eq;

impl<V> Set<V>
where
    V: Hash + Eq,
{
    #[cfg(feature = "orderedmap")]
    pub fn new() -> Self {
        Set(SetImpl::with_hasher(HashImpl::default()))
    }

    #[cfg(not(feature = "orderedmap"))]
    pub fn new() -> Self {
        Set(SetImpl::new())
    }
}

impl<V> Deref for Set<V>
where
    V: Hash + Eq,
{
    type Target = SetImpl<V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V> DerefMut for Set<V>
where
    V: Hash + Eq,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<V> FromIterator<V> for Set<V>
where
    V: Hash + Eq,
{
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        Set(SetImpl::from_iter(iter))
    }
}
