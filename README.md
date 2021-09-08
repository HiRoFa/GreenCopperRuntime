# GreenCopperRuntime

**Just to get thing clear straight away, this is a very much work in progress project, nothing is definitive, it might never become definitive**

# Roadmap / The plan

GreenCopperRuntime is a library which abstracts several different JavaScript runtimes and adds additional features.

By using GreenCopper you can write generic code (including native features like functions and classes) for a JavaScript runtime and use it with the runtime which best suits your needs and/or platform and simply switch between runtimes when desired. This also makes it easier to switch to a more exeprimental runtime like Starlight or boa in the future.

Currently we're working on the following runtimes
* [quickjs_runtime](https://github.com/HiRoFa/quickjs_es_runtime)
* [starlight_runtime](https://github.com/HiRoFa/starlight_runtime)
* [spidermonkey_runtime](https://github.com/HiRoFa/spidermonkey_runtime)

and we're planning support for
* [boa](https://github.com/boa-dev/boa)
* [quickjs with msvc support](https://github.com/theduke/quickjs-rs/pull/114)

## Other GreenCopper projects

[GreenCopperCmd](https://github.com/HiRoFa/GreenCopperCmd) is a commandline utility which you can use to run js/ts files with GreenCopper

[GreenCopperServer](https://github.com/HiRoFa/GreenCopperServer) is a server runtime which add server features (like a http server) and serves as a FaaS-like platform.

# Default implementations

GreenCopperRuntime provides implementations for abstract features of the Runtimes like:
* [x] [FileSystemModuleLoader](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/moduleloaders/struct.FileSystemModuleLoader.html)
* [x] [HTTPModuleLoader](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/moduleloaders/struct.HttpModuleLoader.html)
* [x] [HTTPFetch](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/features/js_fetch/index.html) (http capable implementation of fetch api)

### Preprocessing

GreenCopperRuntime provides script pre-processing for:
* [x] cpp style preprocessing (e.g. use #ifdef $GRECO_DEBUG in code) ([DOCS](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/preprocessors/cpp))
* [ ] macro's which generate script before eval 
* [x] Typescript support is implemented as a separate optional project [typescript_utils](https://github.com/HiRoFa/typescript_utils) 
  * [ ] JavaScript transpiling specific to the runtime used

The following features are optionally added by specifying them in your Cargo.toml

* [ ] db
  * [x] [mysql](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/modules/db/mysql) (Work in progress)
  * [ ] couchbase
  * [ ] cassandra
  * [ ] redis
* [ ] com
  * [x] [http](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/modules/com/http) (Work in progress)
  * [ ] sockets
* [ ] io
  * [x] [gpio](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/modules/io/gpio) (Work in progress)
  * [x] [fs](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/modules/io/fs) (Work in progress)
* [ ] libloading
  * [ ] libc
  * [ ] java
  * [ ] npm
    * [x] [commonjs](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/features/require) 
* [ ] utilities
  * [ ] caching
    * [ ] cache
    * [ ] memcached
    * [ ] zookeeper
    
# Getting started

// wip

## Cargo.toml

In your cargo.toml you can add the green_copper dependency and specify the runtimes you want to use (as features)

```toml
green_copper_runtime =  { git = 'https://github.com/HiRoFa/GreenCopperRuntime', branch="main", features = ["engine_quickjs"]}
quickjs_runtime = {git = 'https://github.com/HiRoFa/quickjs_es_runtime', branch="main"}
```

## Main api concepts

// wip

GreenCopper based runtimes all split the API into two distinct halves, first of all there are your outer thread-safe API's which do not directly call the underlying runtime, These are the
* [JsRuntimeFacade](https://hirofa.github.io/GreenCopperRuntime/hirofa_utils/js_utils/facades/trait.JsRuntimeFacade.html) (represents a Runtime)
* [JsValueFacade](https://hirofa.github.io/GreenCopperRuntime/hirofa_utils/js_utils/facades/values/enum.JsValueFacade.html) (represents a Value)

All of these work (with some exeptions) by adding a job to an EventLoop (a member of the JsRuntimeFacade) and getting the result async (the API returns a Future).

These jobs run in a single thread per runtime and provide access to the Adapters which DO interact with the actual Runtime/Context/Value directly, these are:
* [JsRuntimeAdapter](https://hirofa.github.io/GreenCopperRuntime/hirofa_utils/js_utils/adapters/trait.JsRuntimeAdapter.html) (represents a Runtime)
* [JsRealmAdapter](https://hirofa.github.io/GreenCopperRuntime/hirofa_utils/js_utils/adapters/trait.JsRealmAdapter.html) (represents a Context or Realm)
* [JsValueAdapter](https://hirofa.github.io/GreenCopperRuntime/hirofa_utils/js_utils/adapters/trait.JsValueAdapter.html) (represents a Value)

### Example 
```rust
use quickjs_runtime::esruntime::EsRuntime;
use quickjs_runtime::esruntimebuilder::EsRuntimeBuilder;
use hirofa_utils::js_utils::Script;
use hirofa_utils::js_utils::facades::{JsRuntimeFacade, JsValueFacade};
use hirofa_utils::js_utils::adapters::{JsRealmAdapter, JsRuntimeAdapter};
use quickjs_runtime::quickjscontext::QuickJsContext;

async fn example<T: JsRuntimeFacade>(rt: &T) -> Box<dyn JsValueFacade> {
    // add a job for the main realm (None as realm_name)
    rt.js_loop_realm(None, |_rt_adapter, realm_adapter| {
        let script = Script::new("example.js", "7 + 13");
        let value_adapter= realm_adapter.js_eval(script).ok().expect("script failed");
        // convert value_adapter to value_facade because value_adapter is not Send
        realm_adapter.to_js_value_facade(&value_adapter)
    }).await
}

 fn main() {
    // start a new runtime
    let rt = EsRuntimeBuilder::new().build();
    let val = block_on(example(&rt));
    assert_eq!(val.js_as_i32(), 20);
}

```

## Adding features

// wip

### Functions

// wip

### Proxy classes

// wip

### Modules

// wip