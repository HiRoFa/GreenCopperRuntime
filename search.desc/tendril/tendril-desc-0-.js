searchState.loadedDescShard("tendril", 0, "A marker of an atomic (and hence concurrent) tendril.\nThe multithreadedness of a tendril.\n<code>Tendril</code> for storing binary data.\nA marker of a non-atomic tendril.\nExtension trait for <code>io::Read</code>.\nA simple wrapper to make <code>Tendril</code> <code>Send</code>.\n<code>Tendril</code>-related methods for Rust slices.\n<code>Tendril</code> for storing native Rust strings.\nErrors that can occur when slicing a <code>Tendril</code>.\nCompact string type for zero-copy parsing.\nView as uninterpreted bytes.\nView as a superset format, for free.\nTruncate to length 0 without discarding any owned storage.\nMarker types for formats.\nHelper for the <code>format_tendril!</code> macro.\nCreate a <code>StrTendril</code> through string formatting.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nBuild a <code>Tendril</code> by copying a byte slice, without …\nCreate a <code>Tendril</code> from a single character.\nBuild a <code>Tendril</code> by copying a slice.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConvert into uninterpreted bytes.\nConvert <code>self</code> into a type which is <code>Send</code>.\nConvert into a superset format, for free.\nIs the backing buffer shared?\nIs the backing buffer shared with this other <code>Tendril</code>?\nGet the length of the <code>Tendril</code>.\nCreate a new, empty <code>Tendril</code> in any format.\nDrop <code>n</code> bytes from the back.\nDrop <code>n</code> bytes from the front.\nRemove and return the first character, if any.\nRemove and return a run of characters at the front of the …\nPush some bytes onto the end of the <code>Tendril</code>, without …\nPush a character onto the end.\nPush a slice onto the end of the <code>Tendril</code>.\nPush another <code>Tendril</code> onto the end of this one.\nPush “uninitialized bytes” onto the end.\nView as another format, without validating.\nConvert into another format, without validating.\nReserve space for additional bytes.\nStreams of tendrils.\nSlice this <code>Tendril</code> as a new <code>Tendril</code>.\nMake a <code>Tendril</code> from this slice.\nMake a <code>Tendril</code> from this slice.\nView as a subset format, if the <code>Tendril</code> conforms to that …\nBuild a <code>Tendril</code> by copying a byte slice, if it conforms to …\nConvert into a subset format, if the <code>Tendril</code> conforms to …\nDrop <code>n</code> bytes from the back.\nTry to drop <code>n</code> bytes from the front.\nPush some bytes onto the end of the <code>Tendril</code>, if they …\nPush a character, if it can be represented in this format.\nConvert into another format, if the <code>Tendril</code> conforms to …\nView as another format, if the bytes of the <code>Tendril</code> are …\nAttempt to slice this <code>Tendril</code> as a new <code>Tendril</code>.\nDrop <code>n</code> bytes from the back.\nDrop <code>n</code> bytes from the front.\nSlice this <code>Tendril</code> as a new <code>Tendril</code>.\nCreate a new, empty <code>Tendril</code> with a specified capacity.\nMarker type for ASCII text.\nMarker type for uninterpreted bytes.\nIndicates a format which contains characters from Unicode …\nTrait for format marker types.\nIterator for characters and their byte indices.\nMarker type for the single-byte encoding of the first 256 …\nIndicates a Rust slice type that is represented in memory …\nIndicates a format which corresponds to a Rust slice type, …\nIndicates that one format is a subset of another.\nMarker type for UTF-8 text.\nMarker type for WTF-8 text.\nAccess the raw bytes of the slice.\nIterate over the characters of the string and their byte …\nEncode the character as bytes and pass them to a …\nCompute any fixup needed when concatenating buffers.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConvert a byte slice to this kind of slice.\nConvert a byte slice to this kind of slice.\nImplementation details.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nValidate the <em>other</em> direction of conversion; check if this …\nCheck whether the buffer is valid for this format.\nCheck whether the buffer is valid for this format.\nCheck whether the buffer is valid for this format.\nCheck whether the buffer is valid for this format.\nDescribes how to fix up encodings when concatenating.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nWhat the overall result of processing is.\nTrait for types that can process a tendril.\nA <code>TendrilSink</code> adaptor that takes bytes, decodes them as …\nIndicates that an error has occurred.\nIndicates the end of the stream.\nReturns the argument unchanged.\nRead from the file at the given path and process …\nConsume an iterator of tendrils, processing each item, …\nCalls <code>U::from(self)</code>.\nCreate a new incremental UTF-8 decoder.\nProcess one tendril and finish.\nProcess this tendril.\nRead from the given stream of bytes until exhaustion and …")