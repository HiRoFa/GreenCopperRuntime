searchState.loadedDescShard("rayon", 0, "Rayon is a data-parallelism library that makes it easy to …\nProvides context to a closure called by <code>broadcast</code>.\nWork was found and executed.\nProvides the calling context to a closure called by …\nNo available work was found.\nRepresents a fork-join scope which can be used to spawn …\nRepresents a fork-join scope which can be used to spawn …\nThread builder used for customization via …\nRepresents a user created thread-pool.\nError when initializing a thread pool.\nUsed to create a new <code>ThreadPool</code> or to configure the global …\nResult of <code>yield_now()</code> or <code>yield_local()</code>.\nParallel iterator types for arrays (<code>[T; N]</code>)\n<strong>(DEPRECATED)</strong> Suggest to worker threads that they execute …\nExecutes <code>op</code> within every thread in the current threadpool. …\nExecutes <code>op</code> within every thread in the threadpool. Any …\nCreates a new <code>ThreadPool</code> initialized using this …\nInitializes the global thread pool. This initialization is …\nCreates a scoped <code>ThreadPool</code> initialized using this …\nParallel iterator types for standard collections\nReturns the number of threads in the current registry. If …\nReturns the (current) number of threads in the thread pool.\nReturns true if the current worker thread currently has “…\nIf called from a Rayon worker thread, returns the index of …\nIf called from a Rayon worker thread in this thread-pool, …\nSets a callback to be invoked on thread exit.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates a “fork-join” scope <code>s</code> and invokes the closure …\nCreates a scope that spawns work into this thread-pool.\nCreates a “fork-join” scope <code>s</code> with FIFO order, and …\nCreates a scope that spawns work into this thread-pool in …\nOur index amongst the broadcast threads (ranges from …\nGets the index of this thread in the pool, within …\nExecutes <code>op</code> within the threadpool. Any attempts to use <code>join</code>…\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nTraits for writing parallel programs using an …\nTakes two closures and <em>potentially</em> runs them in parallel. …\nExecute <code>oper_a</code> and <code>oper_b</code> in the thread-pool and return …\nIdentical to <code>join</code>, except that the closures have a …\nReturns the maximum number of threads that Rayon supports …\nReturns <code>true</code> if the closure was called from a different …\nGets the string that was specified by …\nDeprecated in favor of <code>ThreadPoolBuilder::build</code>.\nCreates and returns a valid rayon thread pool builder, but …\nThe number of threads receiving the broadcast in the …\nSets the number of threads to be used in the rayon …\nParallel iterator types for options\nNormally, whenever Rayon catches a panic, it tries to …\nThe rayon prelude imports the various <code>ParallelIterator</code> …\nParallel iterator types for ranges, the type for values …\nParallel iterator types for inclusive ranges, the type for …\nParallel iterator types for results\nExecutes the main loop for this thread. This will not …\nCreates a “fork-join” scope <code>s</code> and invokes the closure …\nCreates a scope that executes within this thread-pool. …\nCreates a “fork-join” scope <code>s</code> with FIFO order, and …\nCreates a scope that executes within this thread-pool. …\nParallel iterator types for slices\nPuts the task into the Rayon threadpool’s job queue in …\nSpawns a job into the fork-join scope <code>self</code>. This job will …\nSpawns an asynchronous task in this thread-pool. This task …\nSpawns an asynchronous task on every thread in this …\nSpawns a job into every thread of the fork-join scope <code>self</code>…\nSpawns a job into every thread of the fork-join scope <code>self</code>…\nSpawns an asynchronous task on every thread in this …\nFires off a task into the Rayon threadpool in the “static…\nSpawns a job into the fork-join scope <code>self</code>. This job will …\nSpawns an asynchronous task in this thread-pool. This task …\nSets a custom function for spawning threads.\nGets the value that was specified by …\nSets the stack size of the worker threads\nSets a callback to be invoked on thread start.\nParallel iterator types for strings\nThis module contains the parallel iterator types for owned …\nSets a closure which takes a thread index and returns the …\nUse the current thread as one of the threads in the pool.\nParallel iterator types for vectors (<code>Vec&lt;T&gt;</code>)\nCooperatively yields execution to local Rayon work.\nCooperatively yields execution to local Rayon work.\nCooperatively yields execution to Rayon.\nCooperatively yields execution to Rayon.\nParallel iterator that moves out of an array.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nThis module contains the parallel iterator types for heaps …\nThis module contains the parallel iterator types for …\nThis module contains the parallel iterator types for …\nThis module contains the parallel iterator types for hash …\nThis module contains the parallel iterator types for hash …\nThis module contains the parallel iterator types for …\nThis module contains the parallel iterator types for …\nDraining parallel iterator that moves out of a binary heap,\nParallel iterator over a binary heap\nParallel iterator over an immutable reference to a binary …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nParallel iterator over a B-Tree map\nParallel iterator over an immutable reference to a B-Tree …\nParallel iterator over a mutable reference to a B-Tree map\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nParallel iterator over a B-Tree set\nParallel iterator over an immutable reference to a B-Tree …\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nDraining parallel iterator that moves out of a hash map, …\nParallel iterator over a hash map\nParallel iterator over an immutable reference to a hash map\nParallel iterator over a mutable reference to a hash map\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nDraining parallel iterator that moves out of a hash set, …\nParallel iterator over a hash set\nParallel iterator over an immutable reference to a hash set\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nParallel iterator over a linked list\nParallel iterator over an immutable reference to a linked …\nParallel iterator over a mutable reference to a linked list\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nDraining parallel iterator that moves a range out of a …\nParallel iterator over a double-ended queue\nParallel iterator over an immutable reference to a …\nParallel iterator over a mutable reference to a …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\n<code>Chain</code> is an iterator that joins <code>b</code> after <code>a</code> in one …\n<code>Chunks</code> is an iterator that groups elements of an …\n<code>Cloned</code> is an iterator that clones the elements of an …\n<code>Copied</code> is an iterator that copies the elements of an …\nThe enum <code>Either</code> with variants <code>Left</code> and <code>Right</code> is a general …\nIterator adaptor for the <code>empty()</code> function.\n<code>Enumerate</code> is an iterator that returns the current count …\n<code>ExponentialBlocks</code> is a parallel iterator that consumes …\n<code>Filter</code> takes a predicate <code>filter_op</code> and filters out …\n<code>FilterMap</code> creates an iterator that uses <code>filter_op</code> to both …\n<code>FlatMap</code> maps each element to a parallel iterator, then …\n<code>FlatMapIter</code> maps each element to a serial iterator, then …\n<code>Flatten</code> turns each element to a parallel iterator, then …\n<code>FlattenIter</code> turns each element to a serial iterator, then …\n<code>Fold</code> is an iterator that applies a function over an …\n<code>FoldChunks</code> is an iterator that groups elements of an …\n<code>FoldChunksWith</code> is an iterator that groups elements of an …\n<code>FoldWith</code> is an iterator that applies a function over an …\n<code>FromParallelIterator</code> implements the creation of a …\nAn iterator that supports “random access” to its data, …\n<code>Inspect</code> is an iterator that calls a function with a …\n<code>Interleave</code> is an iterator that interleaves elements of …\n<code>InterleaveShortest</code> is an iterator that works similarly to …\n<code>Intersperse</code> is an iterator that inserts a particular item …\n<code>IntoParallelIterator</code> implements the conversion to a …\n<code>IntoParallelRefIterator</code> implements the conversion to a …\n<code>IntoParallelRefMutIterator</code> implements the conversion to a …\nThe type of item that the parallel iterator will produce.\nThe type of item that the parallel iterator will produce. …\nThe type of item that will be produced; this is typically …\nThe type of item that this parallel iterator produces. For …\nThe type of item that the parallel iterator will produce. …\nThe type of item that the parallel iterator will produce. …\nThe parallel iterator type that will be created.\nThe type of the parallel iterator that will be returned.\nThe type of iterator that will be created.\nThe draining parallel iterator type that will be created.\nThe draining parallel iterator type that will be created.\n<code>IterBridge</code> is a parallel iterator that wraps a sequential …\nA value of type <code>L</code>.\n<code>Map</code> is an iterator that transforms the elements of an …\n<code>MapInit</code> is an iterator that transforms the elements of an …\n<code>MapWith</code> is an iterator that transforms the elements of an …\n<code>MaxLen</code> is an iterator that imposes a maximum length on …\n<code>MinLen</code> is an iterator that imposes a minimum length on …\n<code>MultiZip</code> is an iterator that zips up a tuple of parallel …\nIterator adaptor for the <code>once()</code> function.\n<code>PanicFuse</code> is an adaptor that wraps an iterator with a fuse …\nConversion trait to convert an <code>Iterator</code> to a …\n<code>ParallelDrainFull</code> creates a parallel iterator that moves …\n<code>ParallelDrainRange</code> creates a parallel iterator that moves …\n<code>ParallelExtend</code> extends an existing collection with items …\nParallel version of the standard iterator trait.\n<code>Positions</code> takes a predicate <code>predicate</code> and filters out …\nIterator adaptor for the <code>repeat()</code> function.\nIterator adaptor for the <code>repeatn()</code> function.\n<code>Rev</code> is an iterator that produces elements in reverse …\nA value of type <code>R</code>.\n<code>Skip</code> is an iterator that skips over the first <code>n</code> elements. …\n<code>SkipAny</code> is an iterator that skips over <code>n</code> elements from …\n<code>SkipAnyWhile</code> is an iterator that skips over elements from …\n<code>Split</code> is a parallel iterator using arbitrary data and a …\n<code>StepBy</code> is an iterator that skips <code>n</code> elements between each …\n<code>Take</code> is an iterator that iterates over the first <code>n</code> …\n<code>TakeAny</code> is an iterator that iterates over <code>n</code> elements from …\n<code>TakeAnyWhile</code> is an iterator that iterates over elements …\n<code>TryFold</code> is an iterator that applies a function over an …\n<code>TryFoldWith</code> is an iterator that applies a function over an …\n<code>UniformBlocks</code> is a parallel iterator that consumes itself …\n<code>Update</code> is an iterator that mutates the elements of an …\nParallelIterator for arbitrary tree-shaped patterns. …\nParallelIterator for arbitrary tree-shaped patterns. …\nParallelIterator for arbitrary tree-shaped patterns. …\n<code>WhileSome</code> is an iterator that yields the <code>Some</code> elements of …\n<code>Zip</code> is an iterator that zips up <code>a</code> and <code>b</code> into a single …\nAn <code>IndexedParallelIterator</code> that iterates over two parallel …\nTests that every item in the parallel iterator matches the …\nSearches for <strong>some</strong> item in the parallel iterator that …\nConvert <code>&amp;mut Either&lt;L, R&gt;</code> to <code>Either&lt;&amp;mut L, &amp;mut R&gt;</code>.\nConvert <code>Pin&lt;&amp;mut Either&lt;L, R&gt;&gt;</code> to …\nConvert <code>Pin&lt;&amp;Either&lt;L, R&gt;&gt;</code> to <code>Either&lt;Pin&lt;&amp;L&gt;, Pin&lt;&amp;R&gt;&gt;</code>, …\nConvert <code>&amp;Either&lt;L, R&gt;</code> to <code>Either&lt;&amp;L, &amp;R&gt;</code>.\nDivides an iterator into sequential blocks of …\nDivides an iterator into sequential blocks of the given …\nTakes two iterators and creates a new iterator over both.\nSplits an iterator up into fixed-size chunks.\nMaps an <code>Either&lt;&amp;L, &amp;R&gt;</code> to an <code>Either&lt;L, R&gt;</code> by cloning the …\nMaps an <code>Either&lt;&amp;mut L, &amp;mut R&gt;</code> to an <code>Either&lt;L, R&gt;</code> by …\nCreates an iterator which clones all of its elements.  …\nLexicographically compares the elements of this …\nCreates a fresh collection containing all the elements …\nCollects the results of the iterator into the specified …\nCollects this iterator into a linked list of vectors.\nMaps an <code>Either&lt;&amp;mut L, &amp;mut R&gt;</code> to an <code>Either&lt;L, R&gt;</code> by …\nMaps an <code>Either&lt;&amp;L, &amp;R&gt;</code> to an <code>Either&lt;L, R&gt;</code> by copying the …\nCreates an iterator which copies all of its elements.  …\nCounts the number of items in this parallel iterator.\nInternal method used to define the behavior of this …\nInternal method used to define the behavior of this …\nApply one of two functions depending on contents, unifying …\nConvert the contained value into <code>T</code>\nLike <code>either</code>, but provide some context to whichever of the …\nCreates a parallel iterator that produces nothing.\nYields an index along with each item.\nDetermines if the elements of this <code>ParallelIterator</code> are …\nReturns the left value\nReturns the right value\nFactors out a homogenous type from an <code>Either</code> of <code>Result</code>.\nFactor out a homogeneous type from an either of pairs.\nConverts an <code>Either</code> of <code>Iterator</code>s to be an <code>Iterator</code> of <code>Either</code>…\nBorrows an <code>Either</code> of <code>Iterator</code>s to be an <code>Iterator</code> of <code>Either</code>s\nMutably borrows an <code>Either</code> of <code>Iterator</code>s to be an <code>Iterator</code> …\nFactors out <code>None</code> from an <code>Either</code> of <code>Option</code>.\nFactors out a homogenous type from an <code>Either</code> of <code>Result</code>.\nFactor out a homogeneous type from an either of pairs.\nApplies <code>filter_op</code> to each item of this iterator, producing …\nApplies <code>filter_op</code> to each item of this iterator to get an …\nSearches for <strong>some</strong> item in the parallel iterator that …\nSearches for the sequentially <strong>first</strong> item in the parallel …\nSearches for the sequentially <strong>last</strong> item in the parallel …\nApplies the given predicate to the items in the parallel …\nApplies the given predicate to the items in the parallel …\nApplies the given predicate to the items in the parallel …\nApplies <code>map_op</code> to each item of this iterator to get nested …\nApplies <code>map_op</code> to each item of this iterator to get nested …\nAn adaptor that flattens parallel-iterable <code>Item</code>s into one …\nAn adaptor that flattens serial-iterable <code>Item</code>s into one …\nConvert <code>Either&lt;L, R&gt;</code> to <code>Either&lt;R, L&gt;</code>.\nParallel fold is similar to sequential fold except that the\nSplits an iterator into fixed-size chunks, performing a …\nSplits an iterator into fixed-size chunks, performing a …\nApplies <code>fold_op</code> to the given <code>init</code> value with each item of …\nExecutes <code>OP</code> on each item produced by the iterator, in …\nExecutes <code>OP</code> on a value returned by <code>init</code> with each item …\nExecutes <code>OP</code> on the given <code>init</code> value with each item …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates an instance of the collection from the parallel …\nDetermines if the elements of this <code>ParallelIterator</code> are …\nDetermines if the elements of this <code>ParallelIterator</code> are …\nApplies <code>inspect_op</code> to a reference to each item of this …\nInterleaves elements of this iterator and the other given …\nInterleaves elements of this iterator and the other given …\nIntersperses clones of an element between items of this …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nExtract the value of an either over two equivalent types.\nConvert the inner value to an iterator.\nConverts <code>self</code> into a parallel iterator.\nReturn true if the value is the <code>Left</code> variant.\nReturn true if the value is the <code>Right</code> variant.\nBorrow the inner value as an iterator.\nMutably borrow the inner value as an iterator.\nDetermines if the elements of this <code>ParallelIterator</code> are …\nConvert the left side of <code>Either&lt;L, R&gt;</code> to an <code>Option&lt;L&gt;</code>.\nApply the function <code>f</code> on the value in the <code>Left</code> variant if …\nReturn left value or given value\nReturn left or a default\nReturns left value or computes it from a closure\nProduces an exact count of how many items this iterator …\nDetermines if the elements of this <code>ParallelIterator</code> are …\nMap <code>f</code> over the contained value and return the result in the\nApplies <code>map_op</code> to each item of this iterator, producing a …\nApply the functions <code>f</code> and <code>g</code> to the <code>Left</code> and <code>Right</code> variants …\nSimilar to <code>map_either</code>, with an added context <code>ctx</code> …\nApplies <code>map_op</code> to a value returned by <code>init</code> with each item …\nApply the function <code>f</code> on the value in the <code>Left</code> variant if …\nApply the function <code>f</code> on the value in the <code>Right</code> variant if …\nApplies <code>map_op</code> to the given <code>init</code> value with each item of …\nComputes the maximum of all the items in the iterator. If …\nComputes the maximum of all the items in the iterator with …\nComputes the item that yields the maximum value for the …\nComputes the minimum of all the items in the iterator. If …\nComputes the minimum of all the items in the iterator with …\nComputes the item that yields the minimum value for the …\nDetermines if the elements of this <code>ParallelIterator</code> are …\nCreates a parallel iterator that produces an element …\nInternal method used to define the behavior of this …\nWraps an iterator with a fuse in case of panics, to halt …\nCreates a bridge from this type to a <code>ParallelIterator</code>.\nReturns a draining parallel iterator over an entire …\nReturns a draining parallel iterator over a range of the …\nExtends an instance of the collection with the elements …\nConverts <code>self</code> into a parallel iterator.\nCreates the parallel iterator from <code>self</code>.\nLexicographically compares the elements of this …\nPartitions the items of a parallel iterator into a pair of …\nPartitions and maps the items of a parallel iterator into …\nTraits and functions used to implement parallel iteration. …\nSearches for <strong>some</strong> item in the parallel iterator that …\nSearches for the sequentially <strong>first</strong> item in the parallel …\nSearches for the sequentially <strong>last</strong> item in the parallel …\nSearches for items in the parallel iterator that match the …\nMultiplies all the items in the iterator.\nReduces the items in the iterator into one item using <code>op</code>. …\nReduces the items in the iterator into one item using <code>op</code>. …\nCreates a parallel iterator that endlessly repeats <code>elt</code> (by …\nCreates a parallel iterator that produces <code>n</code> repeats of <code>elt</code> …\nProduces a new iterator with the elements of this iterator …\nConvert the right side of <code>Either&lt;L, R&gt;</code> to an <code>Option&lt;R&gt;</code>.\nApply the function <code>f</code> on the value in the <code>Right</code> variant if …\nReturn right value or given value\nReturn right or a default\nReturns right value or computes it from a closure\nCreates an iterator that skips the first <code>n</code> elements.\nCreates an iterator that skips <code>n</code> elements from <em>anywhere</em> in …\nCreates an iterator that skips elements from <em>anywhere</em> in …\nThe <code>split</code> function takes arbitrary data and a closure that …\nCreates an iterator that steps by the given amount\nSums up the items in the iterator.\nCreates an iterator that yields the first <code>n</code> elements.\nTakes only <code>n</code> repeats of the element, similar to the general\nCreates an iterator that yields <code>n</code> elements from <em>anywhere</em> …\nCreates an iterator that takes elements from <em>anywhere</em> in …\nPerforms a fallible parallel fold.\nPerforms a fallible parallel fold with a cloneable <code>init</code> …\nExecutes a fallible <code>OP</code> on each item produced by the …\nExecutes a fallible <code>OP</code> on a value returned by <code>init</code> with …\nExecutes a fallible <code>OP</code> on the given <code>init</code> value with each …\nReduces the items in the iterator into one item using a …\nReduces the items in the iterator into one item using a …\nReturns the left value\nReturns the right value\nUnzips the items of a parallel iterator into a pair of …\nUnzips the results of the iterator into the specified …\nMutates each item of this iterator before yielding it.\nCreate a tree like parallel iterator from an initial root …\nCreate a tree like postfix parallel iterator from an …\nCreate a tree-like prefix parallel iterator from an …\nCreates an iterator over the <code>Some</code> items of this iterator, …\nSets the maximum length of iterators desired to process in …\nSets the minimum length of iterators desired to process in …\nInternal method used to define the behavior of this …\nIterates over tuples <code>(A, B)</code>, where the items <code>A</code> are from …\nIterates tuples, repeating the element with items from …\nThe same as <code>Zip</code>, but requires that both iterators have the …\nA consumer is effectively a generalized “fold” …\nThe <code>Folder</code> trait encapsulates the standard fold operation. …\nThe type of folder that this consumer can be converted …\nThe type of iterator we will become.\nThe type of item that will be produced by this producer …\nThe type of item returned by this producer.\nThe type of value returned by this callback. Analogous to …\nA <code>Producer</code> is effectively a “splittable <code>IntoIterator</code>”. …\nThe <code>ProducerCallback</code> trait is a kind of generic closure, …\nThe reducer is the final step of a <code>Consumer</code> – after a …\nThe type of reducer that is produced if this consumer is …\nThe type of result that this consumer will ultimately …\nThe type of result that will ultimately be produced by the …\nA stateless consumer can be freely copied. These consumers …\nA variant on <code>Producer</code> which does not know its exact length …\nThis helper function is used to “connect” a parallel …\nThis helper function is used to “connect” a producer …\nA variant of <code>bridge_producer_consumer</code> where the producer …\nInvokes the callback with the given producer as argument. …\nFinish consuming items, produce final result.\nConsume next item and return new sequential state.\nConsume items from the iterator until full, and return new …\nIterate the producer, feeding each element to <code>folder</code>, and …\nIterate the producer, feeding each element to <code>folder</code>, and …\nHint whether this <code>Consumer</code> would like to stop processing …\nHint whether this <code>Folder</code> would like to stop processing …\nConvert the consumer into a folder that can consume items …\nConvert <code>self</code> into an iterator; at this point, no more …\nThe maximum number of items that we will process …\nThe minimum number of items that we will process …\nReduce two final results into one; this is executed after a\nSplit midway into a new producer if possible, otherwise …\nSplit into two producers; one produces items <code>0..index</code>, the …\nDivide the consumer into two consumers, one processing …\nSplits off a “left” consumer and returns it. The <code>self</code> …\nCreates a reducer that can be used to combine the results …\nA parallel iterator over the value in <code>Some</code> variant of an …\nA parallel iterator over a reference to the <code>Some</code> variant …\nA parallel iterator over a mutable reference to the <code>Some</code> …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nParallel iterator over a range, implemented for all …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nParallel iterator over an inclusive range, implemented for …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nParallel iterator over a result\nParallel iterator over an immutable reference to a result\nParallel iterator over a mutable reference to a result\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nParallel iterator over slice in (non-overlapping) chunks …\nParallel iterator over slice in (non-overlapping) mutable …\nParallel iterator over immutable non-overlapping chunks of …\nParallel iterator over immutable non-overlapping chunks of …\nParallel iterator over mutable non-overlapping chunks of a …\nParallel iterator over mutable non-overlapping chunks of a …\nParallel iterator over immutable items in a slice\nParallel iterator over mutable items in a slice\nParallel extensions for slices.\nParallel extensions for mutable slices.\nParallel iterator over immutable non-overlapping chunks of …\nParallel iterator over immutable non-overlapping chunks of …\nParallel iterator over mutable non-overlapping chunks of a …\nParallel iterator over mutable non-overlapping chunks of a …\nParallel iterator over slices separated by a predicate\nParallel iterator over slices separated by a predicate, …\nParallel iterator over mutable slices separated by a …\nParallel iterator over mutable slices separated by a …\nParallel iterator over immutable overlapping windows of a …\nReturns a plain slice, which is used to implement the rest …\nReturns a plain mutable slice, which is used to implement …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturn the remainder of the original slice that is not …\nReturn the remainder of the original slice that is not …\nReturns a parallel iterator over the slice producing …\nReturns a parallel iterator over the slice producing …\nReturns a parallel iterator over at most <code>chunk_size</code> …\nReturns a parallel iterator over <code>chunk_size</code> elements of …\nReturns a parallel iterator over <code>chunk_size</code> elements of …\nReturns a parallel iterator over at most <code>chunk_size</code> …\nReturns a parallel iterator over at most <code>chunk_size</code> …\nReturns a parallel iterator over <code>chunk_size</code> elements of …\nReturns a parallel iterator over <code>chunk_size</code> elements of …\nReturns a parallel iterator over at most <code>chunk_size</code> …\nSorts the slice in parallel.\nSorts the slice in parallel with a comparator function.\nSorts the slice in parallel with a key extraction function.\nSorts the slice in parallel with a key extraction function.\nSorts the slice in parallel, but might not preserve the …\nSorts the slice in parallel with a comparator function, …\nSorts the slice in parallel with a key extraction …\nReturns a parallel iterator over subslices separated by …\nReturns a parallel iterator over subslices separated by …\nReturns a parallel iterator over mutable subslices …\nReturns a parallel iterator over mutable subslices …\nReturns a parallel iterator over all contiguous windows of …\nReturn the remainder of the original slice that is not …\nReturn the remainder of the original slice that is not …\nReturn the remainder of the original slice that is not …\nReturn the remainder of the original slice that is not …\nReturn the remainder of the original slice that is not …\nReturn the remainder of the original slice that is not …\nParallel iterator over the bytes of a string\nParallel iterator over the characters of a string, with …\nParallel iterator over the characters of a string\nParallel iterator over a string encoded as UTF-16\nParallel iterator over lines in a string\nParallel iterator over substrings that match a pattern, …\nParallel iterator over substrings that match a pattern\nParallel extensions for strings.\nParallel iterator over substrings separated by a pattern\nParallel iterator over substrings separated by ASCII …\nParallel iterator over substrings separated by a pattern\nParallel iterator over substrings separated by a …\nParallel iterator over substrings separated by whitespace\nReturns a plain string slice, which is used to implement …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns a parallel iterator over the bytes of a string.\nReturns a parallel iterator over the characters of a …\nReturns a parallel iterator over the characters of a …\nReturns a parallel iterator over a string encoded as …\nReturns a parallel iterator over the lines of a string, …\nReturns a parallel iterator over substrings that match a …\nReturns a parallel iterator over substrings that match a …\nReturns a parallel iterator over substrings separated by a …\nReturns a parallel iterator over the sub-slices of a …\nReturns a parallel iterator over substrings separated by a …\nReturns a parallel iterator over substrings terminated by a\nReturns a parallel iterator over the sub-slices of a …\nDraining parallel iterator that moves a range of …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nDraining parallel iterator that moves a range out of a …\nParallel iterator that moves out of a vector.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.")