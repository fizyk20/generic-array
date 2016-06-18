use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::impls::SeqIteratorVisitor;
use serde::de::impls::VecVisitor;
use {ArrayLength, GenericArray};

impl<T, N> Serialize for GenericArray<T, N>
    where T: Serialize, N: ArrayLength<T> {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        // serializes this array just like a slice or a vector
        serializer.serialize_seq(SeqIteratorVisitor::new(self.iter(), Some(self.len())))
    }
}

impl<T, N> Deserialize for GenericArray<T, N>
    where T: Deserialize + Clone, N: ArrayLength<T>
{
    fn deserialize<D>(deserializer: &mut D) -> Result<GenericArray<T, N>, D::Error>
        where D: Deserializer,
    {
        // this implementation has the cost of allocating a new vector each time.
        // TODO: write a better 'allocationless' version
        deserializer.deserialize_seq(VecVisitor::new()).map(|vec| GenericArray::from_slice(&vec))
    }
}