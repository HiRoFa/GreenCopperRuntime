# GreenCopperRuntime

**Just to get thing clear straight away, this is a very much work in progress project, nothing is definitive, it might never become definitive**

# Roadmap / The plan

GreenCopperRuntime is a library which adds additional features to a QuickJs JavaScript runtime.

GreenCopperRuntime is based on [quickjs_runtime](https://github.com/HiRoFa/quickjs_es_runtime)

## Other GreenCopper projects

[GreenCopperCmd](https://github.com/HiRoFa/GreenCopperCmd) is a commandline utility which you can use to run js/ts files with GreenCopper

# Default implementations

GreenCopperRuntime provides implementations for abstract features of the Runtimes like:
* [x] [FileSystemModuleLoader](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/moduleloaders/struct.FileSystemModuleLoader.html)
* [x] [HTTPModuleLoader](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/moduleloaders/struct.HttpModuleLoader.html)
* [x] [HTTPFetch](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/features/js_fetch/index.html) (http capable implementation of fetch api)

### Preprocessing

GreenCopperRuntime provides script pre-processing for:
* [x] cpp style preprocessing (e.g. use #ifdef $GRECO_DEBUG in code) ([DOCS](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/preprocessors/cpp))
* [ ] macros which generate script before eval 
* [x] Typescript support is implemented as a separate optional project [typescript_utils](https://github.com/HiRoFa/typescript_utils) 

The following features are optionally added by specifying them in your Cargo.toml

* [x] [HTML Dom](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/modules/htmldom/index.html) (Work in progress)
* [ ] crypto
  * [x] crypto.randomUUID()
  * [ ] crypto.subtle
* [x] JWT (Work in progress)
* [ ] db
  * [x] [mysql](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/modules/db/mysql) (Work in progress)
    * [x] single query (named and positional params)
    * [x] execute (batch)
    * [x] transactions
  * [ ] cassandra
  * [ ] redis
* [ ] com
  * [ ] [http](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/modules/com/http) (Work in progress, was deleted due to fetch being done first. will review this func later for advanced things like client certs)
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
    * [x] cache (WiP)
    
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
* [QuickJsRuntimeFacade](https://hirofa.github.io/GreenCopperRuntime/hirofa_utils/js_utils/facades/trait.JsRuntimeFacade.html) (represents a Runtime)
* [JsValueFacade](https://hirofa.github.io/GreenCopperRuntime/hirofa_utils/js_utils/facades/values/enum.JsValueFacade.html) (represents a Value)

All of these work (with some exeptions) by adding a job to an EventLoop (a member of the JsRuntimeFacade) and getting the result async (the API returns a Future).

These jobs run in a single thread per runtime and provide access to the Adapters which DO interact with the actual Runtime/Context/Value directly, these are:
* [QuickJsRuntimeAdapter](https://hirofa.github.io/GreenCopperRuntime/hirofa_utils/js_utils/adapters/trait.JsRuntimeAdapter.html) (represents a Runtime)
* [QuickJsRealmAdapter](https://hirofa.github.io/GreenCopperRuntime/hirofa_utils/js_utils/adapters/trait.JsRealmAdapter.html) (represents a Context or Realm)
* [QuickJsValueAdapter](https://hirofa.github.io/GreenCopperRuntime/hirofa_utils/js_utils/adapters/trait.JsValueAdapter.html) (represents a Value)

## Example

// todo

## Adding features

// wip

### Functions

// wip

### Proxy classes

// wip

### Modules

// wip