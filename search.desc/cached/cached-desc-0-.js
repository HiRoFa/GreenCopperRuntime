searchState.loadedDescShard("cached", 0, "Build Status crates.io docs\nCache operations\nExtra cache operations for types that implement <code>Clone</code>\nCache operations on an io-connected store\nUsed to wrap a function result so callers can see whether …\nReturn the cache capacity\nRemove all cached values. Keeps the allocated memory for …\nAttempt to retrieve a cached value\nAttempt to retrieve a cached value\nAttempt to retrieve a cached value and indicate whether …\nAttempt to retrieve a cached value with mutable access\nGet or insert a key, value pair\nReturn the number of times a cached value was successfully …\nReturn the lifespan of cached values (time to eviction)\nReturn the lifespan of cached values (time to eviction)\nReturn the number of times a cached value was unable to be …\nRemove a cached value\nRemove a cached value\nRemove all cached values. Free memory and return to …\nReset misses/hits counters\nInsert a key, value pair and return the previous value\nInsert a key, value pair and return the previous value\nSet the lifespan of cached values, returns the old value\nSet the lifespan of cached values, returns the old value.\nSet the flag to control whether cache hits refresh the ttl …\nReturn the current cache size (number of elements)\nGet or insert a key, value pair with error handling\nRemove the lifespan for cached values, returns the old …\nRemove the lifespan for cached values, returns the old …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nDeclarative macros for defining functions that wrap a …\nProcedural macros for defining functions that wrap a …\nUsed to wrap a function result so callers can see whether …\nDefine a memoized function using a cache store that …\nDefine a memoized function using a cache store that …\nDefine a memoized function using a cache store that …\nThe <code>CanExpire</code> trait defines a function for implementations …\nA cache enforcing time expiration and an optional maximum …\nExpiring Value Cache\nLeast Recently Used / <code>Sized</code> Cache\nCache store bound by time\nTimed LRU Cache\nDefault unbounded cache\nClear all cache entries. Does not release underlying …\nEvict values that have expired. Returns number of dropped …\nRemove any expired values from the cache\nRemove any expired values from the cache\nRemove any expired values from the cache\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nRetrieve unexpired entry\nRetrieve unexpired entry, accepting <code>&amp;[T]</code> to check against …\nRetrieve unexpired entry, accepting <code>&amp;str</code> to check against …\nReturns a reference to the cache’s <code>order</code>\nReturns a reference to the cache’s <code>store</code>\nReturns a reference to the cache’s <code>store</code>\nReturns a reference to the cache’s <code>store</code>\nInsert k/v pair without running eviction logic. See …\nInsert k/v pair and run eviction logic. See …\nInsert k/v pair with explicit ttl. See <code>.insert_ttl_evict</code>\nOptionally run eviction logic before inserting a k/v pair …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\n<code>is_expired</code> returns whether the value has expired.\nReturn an iterator of keys in the current order from most …\nReturn an iterator of keys in the current order from most …\nReturn cache size. Note, this does not evict so may return …\nCreates an empty <code>UnboundCache</code>\nReturns if the lifetime is refreshed when the value is …\nReturns if the lifetime is refreshed when the value is …\nRemove an entry, returning an unexpired value if it was …\nIncrease backing stores with enough capacity to store <code>more</code>\nRetain only the latest <code>count</code> values, dropping the next …\nSets if the lifetime is refreshed when the value is …\nSets if the lifetime is refreshed when the value is …\nSet a size limit. When reached, the next entries to expire …\nCreates a new <code>SizedCache</code> with a given size limit and …\nCreates a new <code>TimedSizedCache</code> with a specified lifespan …\nSet ttl millis, return previous value\nReturn an iterator of values in the current order from most\nReturn an iterator of timestamped values in the current …\nCreates an empty <code>UnboundCache</code> with a given pre-allocated …\nCreates a new <code>TimedCache</code> with a specified lifespan\nCreates a new <code>TimedCache</code> with a specified lifespan and …\nCreates a new <code>TimedCache</code> with a specified lifespan which …\nCreates a new <code>ExpiringValueCache</code> with a given size limit …\nCreates a new <code>SizedCache</code> with a given size limit and …\nCreates a new <code>SizedCache</code> with a given size limit and …\nCreates a new <code>SizedCache</code> with a given size limit and …")