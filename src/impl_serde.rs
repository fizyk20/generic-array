use {ArrayLength, GenericArray};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::impls::VecVisitor;

impl<T, N> Serialize for GenericArray<T, N>
    where T: Serialize,
          N: ArrayLength<T>
{
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.collect_seq(self.iter())
    }
}

impl<T, N> Deserialize for GenericArray<T, N>
    where T: Deserialize + Clone,
          N: ArrayLength<T>
{
    fn deserialize<D>(deserializer: D) -> Result<GenericArray<T, N>, D::Error>
        where D: Deserializer
    {
        // this implementation has the cost of allocating a new vector each time.
        // TODO: write a better 'allocationless' version
        deserializer
            .deserialize_seq(VecVisitor::new())
            .map(|vec| GenericArray::clone_from_slice(&vec))
    }
}
