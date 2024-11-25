searchState.loadedDescShard("generic_array", 0, "This crate implements a structure that can be used as a …\nTrait used to define the number of elements in a …\nThe associated <code>ArrayLength</code>\nAssociated type representing the underlying contiguous …\nAssociated <code>ArrayLength</code> for one <code>Const&lt;N&gt;</code>\nStruct representing a generic array - <code>GenericArray&lt;T, N&gt;</code> …\nAn iterator that moves out of a <code>GenericArray</code>\nImplemented for types which can have an associated …\nError for <code>TryFrom</code> and <code>try_from_iter</code>\nMacro allowing for easy construction of Generic Arrays.\nReturns the remaining items of this iterator as a mutable …\nExtracts a mutable slice containing the entire array.\nReturns the remaining items of this iterator as a slice\nExtracts a slice containing the entire array.\nExtracts the values from a generic array of <code>MaybeUninit</code> …\nLike <code>arr!</code>, but returns a <code>Box&lt;GenericArray&lt;T, N&gt;&gt;</code>\nConverts a slice of <code>T</code> elements into a slice of …\nConverts a mutable slice of <code>T</code> elements into a mutable …\nReturns the constant “default value” for an array …\nAlternative to <code>Box::&lt;GenericArray&lt;T, N&gt;&gt;::default()</code> that …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConvert a native array into <code>GenericArray</code> of the same …\nConvert a slice of native arrays into a slice of …\nConvert a mutable slice of native arrays into a mutable …\nCreate a <code>GenericArray</code> from an iterator.\nConverts a mutable slice to a mutable generic array …\nConverts a slice to a generic array reference with …\nFunctional programming with generic sequences\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConvert the <code>GenericArray</code> into a native array of the same …\nConverts a <code>Box&lt;GenericArray&lt;T, N&gt;&gt;</code> into <code>Box&lt;[T]&gt;</code> without …\nConverts a slice <code>GenericArray&lt;T, N&gt;</code> into a slice of <code>[T; N]</code>\nConverts a mutable slice <code>GenericArray&lt;T, N&gt;</code> into a mutable …\nConverts a <code>Box&lt;GenericArray&lt;T, N&gt;&gt;</code> into <code>Vec&lt;T&gt;</code> without …\nReturns the number of elements in the array.\nUseful traits for manipulating sequences of data stored in …\nConvert a slice of <code>GenericArray&lt;T, N&gt;</code> into a slice of <code>T</code>, …\nConvert a slice of <code>GenericArray&lt;T, N&gt;</code> into a slice of <code>T</code>, …\nLike <code>GenericArray::try_from_iter</code> but returns a …\nAttempts to convert a <code>Box&lt;[T]&gt;</code> into <code>Box&lt;GenericArray&lt;T, N&gt;&gt;</code>…\nFallible equivalent of <code>FromIterator::from_iter</code>\nConverts a mutable slice to a mutable generic array …\nConverts a slice to a generic array reference with …\nAttempts to convert a <code>Vec&lt;T&gt;</code> into <code>Box&lt;GenericArray&lt;T, N&gt;&gt;</code> …\nCreate a new array of <code>MaybeUninit&lt;T&gt;</code> items, in an …\nDefines functional programming methods for generic …\nMapped sequence type\nDefines the relationship between one generic sequence and …\nAccessor type for a mapped generic sequence\nFolds (or reduces) a sequence of data into a single value.\nMaps a <code>GenericSequence</code> to another <code>GenericSequence</code>.\nCombines two <code>GenericSequence</code> instances and iterates …\nDefines <code>GenericSequence</code>s which can be joined together, …\nFirst part of the resulting split array\nDefines a <code>GenericSequence</code> of <code>GenericArray</code>s which can be …\nDefines some sequence with an associated length and …\n<code>GenericArray</code> associated length\nDefines any <code>GenericSequence</code> which can be lengthened or …\n<code>GenericSequence</code> that has one more element than <code>Self</code>\nResulting sequence formed by the concatenation.\nResulting sequence formed by removing an element at the …\nFlattened sequence type\nUnflattened sequence type\nDefines a <code>GenericSequence</code> which can be shortened by …\nSequence to be concatenated with <code>self</code>\nSecond part of the resulting split array\nOwned sequence type used in conjunction with reference …\nAccessor for <code>GenericSequence</code> item type, which is really …\nDefines a <code>GenericSequence</code> which can be shortened by …\n<code>GenericSequence</code> that has one less element than <code>Self</code>\nDefines a <code>GenericSequence</code> that can be split into two parts …\nDefines a <code>GenericSequence</code> of <code>T</code> which can be split evenly …\nReturns a new array with the given element appended to the …\nConcatenate, or join, two sequences.\nFlattens the sequence into a single <code>GenericArray</code>.\nInitializes a new sequence instance using the given …\nReturns a new array without the last element, and the last …\nReturns a new array without the first element, and the …\nReturns a new array with the given element prepended to …\nRemoves an element at the given index, shifting elements …\nRemoves an element at the given index without bounds …\nSplits an array at the given index, returning the separate …\nRemoves an element at the given index, swapping it with …\nRemoves an element at the given index without bounds …\nUnflattens the sequence into a sequence of <code>GenericArray</code>s.")