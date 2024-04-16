use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// Trait for batch loading.
#[async_trait::async_trait]
pub trait Loader<K: Send + Sync + Hash + Eq + Clone + 'static>: Send + Sync + 'static {
    /// type of value.
    type Value: Debug + Send + Sync + Clone + 'static;

    /// Type of error.
    type Error: Debug + Send + Clone + 'static;

    /// Load the data set specified by the `keys`.
    async fn load(&self, keys: &[K]) -> Result<HashMap<K, Self::Value>, Self::Error>;
}
