//! # Cache module
//!
//! this module can be used as a machine local cache (caches are shared between runtimes (threads))
//!
//! # Example
//!
//! ## cacheMod.getRegion(id: string, options?: object): greco.util.cache.Region
//!
//! Gets or initializes a region
//!
//! The options object may contain the following params (please note that if the region for the id already exists these are ignored)
//!
//! * items: number // default = 100.000
//!   maximum number of items to cache (when more items become present the least recently used will be removed even if withing its ttl)
//! * idle: number // default = 3.600.000 (one hour)
//!   max idle (unused) time for an object in milliseconds
//! * ttl: number // default = 86.400.000 (one day)
//!   max age for an object (the entry will be invalidated after this time even if recently used)
//!
//! # Example
//!
//! ```javascript
//! import * as grecoCache from 'greco://cache';
//! let options = {
//!     items: 100000
//! };
//! const cacheRegion = grecoCache.getRegion('my_cache_region_id', options);
//! ```
//!
//! ## cacheRegion.get(key: string, init: Function<string | Promise<string>>): string | Promise<string>
//! gets or returns an item based on a key
//! it may return the result (as string) directly or it may return a Promise
//! if an item does not exist in the cache the init function is invoked
//!
//! ```javascript
//! import * as grecoCache from 'greco://cache';
//! const cacheRegion = grecoCache.getRegion('my_cache_region_id');
//! export async function load(key) {
//!      return cacheRegion.get(key, async () => {
//!          return "largeLoadedThing_" + key;
//!      });
//! }
//! ```
//!
//! ## cacheRegion.remove(key: string): void
//! removes an item from the cache
//!

use hirofa_utils::auto_id_map::AutoIdMap;
use hirofa_utils::debug_mutex::DebugMutex;
use hirofa_utils::js_utils::adapters::proxies::JsProxy;
use hirofa_utils::js_utils::adapters::{JsRealmAdapter, JsValueAdapter};
use hirofa_utils::js_utils::facades::values::JsValueFacade;
use hirofa_utils::js_utils::facades::JsRuntimeBuilder;
use hirofa_utils::js_utils::modules::NativeModuleLoader;
use hirofa_utils::js_utils::JsError;
use lru::LruCache;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Sub;
use std::sync::{Arc, Weak};
use std::thread;
use std::time::{Duration, Instant};

struct CacheEntry {
    val: JsValueFacade,
    created: Instant,
    last_used: Instant,
}

struct CacheRegion {
    lru_cache: LruCache<String, CacheEntry>,
    ttl: Duration,
    max_idle: Duration,
}

impl CacheRegion {
    pub fn get(&mut self, key: &str) -> Option<&CacheEntry> {
        if let Some(ce) = self.lru_cache.get_mut(key) {
            ce.last_used = Instant::now();
        }
        self.lru_cache.get(key)
    }
    pub fn remove(&mut self, key: &str) -> Option<CacheEntry> {
        self.lru_cache.pop(key)
    }
    pub fn put(&mut self, key: &str, val: JsValueFacade) {
        let ce = CacheEntry {
            val,
            created: Instant::now(),
            last_used: Instant::now(),
        };
        self.lru_cache.put(key.to_string(), ce);
    }
    fn invalidate_stale(&mut self) {
        let min_last_used = Instant::now().sub(self.max_idle);
        let min_created = Instant::now().sub(self.ttl);
        loop {
            if let Some(lru) = self.lru_cache.peek_lru() {
                if lru.1.last_used.lt(&min_last_used) || lru.1.created.lt(&min_created) {
                    // invalidate
                    let _ = self.lru_cache.pop_lru();
                } else {
                    // oldest was still valid, break of search
                    break;
                }
            }
        }
    }
}

struct ManagedCache {
    regions: HashMap<(String, String), Weak<DebugMutex<CacheRegion>>>,
}

impl ManagedCache {
    fn new() -> Self {
        Self {
            regions: HashMap::new(),
        }
    }

    pub fn get_or_create_region(
        &mut self,
        realm_id: &str,
        cache_id: &str,
        max_idle: Duration,
        ttl: Duration,
        max_items: usize,
    ) -> Arc<DebugMutex<CacheRegion>> {
        let key = (realm_id.to_string(), cache_id.to_string());
        if let Some(weak) = self.regions.get(&key) {
            if let Some(arc) = weak.upgrade() {
                return arc;
            }
        }
        // new
        let region = CacheRegion {
            lru_cache: LruCache::new(max_items),
            ttl,
            max_idle,
        };
        let region_arc = Arc::new(DebugMutex::new(region, "region_mutex"));
        self.regions.insert(key, Arc::downgrade(&region_arc));
        region_arc
    }
}

fn cache_cleanup() {
    let mut to_clean = vec![];
    {
        let lock: &mut ManagedCache = &mut *CACHE.lock("cache_cleanup").unwrap();
        let keys: Vec<(String, String)> = lock.regions.keys().cloned().collect();
        for key in keys {
            let weak_opt = lock.regions.get(&key);
            if let Some(weak) = weak_opt {
                if let Some(cache_arc) = weak.upgrade() {
                    to_clean.push(cache_arc.clone());
                } else {
                    lock.regions.remove(&key);
                }
            } else {
                lock.regions.remove(&key);
            }
        }
    }
    for cache_to_clean in to_clean {
        let cache_lock = &mut *cache_to_clean.lock("cache_cleanup").unwrap();
        cache_lock.invalidate_stale();
    }
}

lazy_static! {
    static ref CACHE: Arc<DebugMutex<ManagedCache>> = {
        // start cleanup thread
        thread::spawn(|| loop {
            thread::sleep(Duration::from_secs(60));
            cache_cleanup();
        });
        Arc::new(DebugMutex::new(ManagedCache::new(), "CACHE"))

    };
}

// todo better to impl regions in Cache with a CacheHandle which can be used as our per_thread instance (drop region when all handles drop?)

thread_local! {
    static CACHES: RefCell<AutoIdMap<Arc<DebugMutex<CacheRegion>>>> = RefCell::new(AutoIdMap::new());
}

fn with_cache_region<C: FnOnce(&mut CacheRegion) -> R, R>(id: usize, consumer: C) -> R {
    CACHES.with(|rc| {
        let caches = &mut *rc.borrow_mut();
        let cache_mtx = caches.get(&id).expect("invalid cache id");
        let cache_locked = &mut *cache_mtx.lock("with_cache_region").unwrap();
        consumer(cache_locked)
    })
}

struct CacheModuleLoader {
    //
}

impl<R: JsRealmAdapter + 'static> NativeModuleLoader<R> for CacheModuleLoader {
    fn has_module(&self, _realm: &R, module_name: &str) -> bool {
        module_name.eq("greco://cache")
    }

    fn get_module_export_names(&self, _realm: &R, _module_name: &str) -> Vec<&str> {
        vec!["getRegion"]
    }

    fn get_module_exports(
        &self,
        realm: &R,
        _module_name: &str,
    ) -> Vec<(&str, R::JsValueAdapterType)> {
        init_region_proxy(realm)
            .ok()
            .expect("init cache region failed");

        init_exports(realm).ok().expect("init cache exports failed")
    }
}

fn cache_add<R: JsRealmAdapter + 'static>(
    realm: &R,
    key: &str,
    value: &R::JsValueAdapterType,
    region: &mut CacheRegion,
) -> Result<(), JsError> {
    if value.js_is_string() || value.js_is_i32() || value.js_is_f64() || value.js_is_bool() {
        let jsvf = realm.to_js_value_facade(value)?;

        region.put(key, jsvf);

        Ok(())
    } else {
        Err(JsError::new_str("Only cache primitives"))
    }
}

fn init_region_proxy<R: JsRealmAdapter + 'static>(realm: &R) -> Result<(), JsError> {
    let proxy = JsProxy::new(&["greco", "util", "cache"], "Region")
        .add_method("get", |_rt, realm: &R, instance_id, args| {
            if args.len() != 2 || !args[0].js_is_string() || !args[1].js_is_function() {
                return Err(JsError::new_str(
                    "get requires two arguments, key:string and init:function",
                ));
            }

            let key = args[0].js_to_string()?;

            with_cache_region(instance_id, move |cache_region| {
                let entry_opt = cache_region.get(key.as_str());
                if let Some(entry) = entry_opt {
                    let jsvf = &entry.val;
                    match jsvf {
                        JsValueFacade::I32 { val } => realm.js_i32_create(*val),
                        JsValueFacade::F64 { val } => realm.js_f64_create(*val),
                        JsValueFacade::String { val } => realm.js_string_create(val),
                        JsValueFacade::Boolean { val } => realm.js_boolean_create(*val),
                        _ => Err(JsError::new_str("unexpected cached jsvf type")),
                    }
                } else {
                    let init_func = args[1].clone();

                    let init_result = realm.js_function_invoke(None, &init_func, &[])?;

                    if init_result.js_is_promise() {
                        let then = realm.js_function_create(
                            "cache_add_func",
                            move |realm, _this, args| {
                                // cache args 0
                                with_cache_region(instance_id, |cache_region2| {
                                    cache_add(realm, &key, &args[0], cache_region2)
                                })?;

                                realm.js_null_create()
                            },
                            1,
                        )?;
                        realm.js_promise_add_reactions(&init_result, Some(then), None, None)?;
                    } else {
                        cache_add(realm, &key, &init_result, cache_region)?;
                    }
                    Ok(init_result)
                }
            })
        })
        .add_method("remove", |_rt, realm, instance_id, args| {
            if args.len() != 1 || !args[0].js_is_string() {
                return Err(JsError::new_str(
                    "remove requires one arguments, key:string",
                ));
            }

            let key = args[0].js_to_string()?;

            with_cache_region(instance_id, |region| {
                region.remove(key.as_str());
                realm.js_null_create()
            })
        })
        .set_finalizer(|_rt, _realm, instance_id| {
            //
            CACHES.with(|rc| {
                let caches = &mut *rc.borrow_mut();
                let _ = caches.remove(&instance_id);
            })
        });

    realm.js_proxy_install(proxy, false)?;
    Ok(())
}

fn init_exports<R: JsRealmAdapter + 'static>(
    realm: &R,
) -> Result<Vec<(&'static str, R::JsValueAdapterType)>, JsError> {
    let cache_region_function = realm.js_function_create(
        "getRegion",
        |realm, _this, args| {
            if args.is_empty()
                || !args[0].js_is_string()
                || (args.len() > 1 && !args[1].js_is_object())
            {
                return Err(JsError::new_str(
                    "getRegion requires one or two arguments, id:string and init: object",
                ));
            }

            let cache = &mut *CACHE.lock("getRegion").unwrap();

            let cache_id = args[0].js_to_string()?;
            let max_idle = Duration::from_secs(3600);
            let ttl = Duration::from_secs(86400);
            let max_items = 100000;

            let region = cache.get_or_create_region(
                realm.js_get_realm_id(),
                cache_id.as_str(),
                max_idle,
                ttl,
                max_items,
            );

            let instance_id = CACHES.with(|rc| {
                let caches = &mut *rc.borrow_mut();
                caches.insert(region)
            });

            realm.js_proxy_instantiate_with_id(&["greco", "util", "cache"], "Region", instance_id)
        },
        1,
    )?;

    Ok(vec![("getRegion", cache_region_function)])
}

pub(crate) fn init<B: JsRuntimeBuilder>(builder: B) -> B {
    // todo

    // add greco://cache module (machine local cache)
    // config per region, every region is a LRUCache
    builder.js_native_module_loader(CacheModuleLoader {})
}
