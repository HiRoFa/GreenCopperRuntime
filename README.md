# GreenCopperRuntime

**Just to get thing clear straight away, this is a very much work in progress project, nothing is definitive, it might never become definitive**

GreenCopperRuntime is a library for which provides as JavaScript runtime (based on the [quickjs_runtime](https://github.com/HiRoFa/quickjs_es_runtime) project) with additional features.

[GreenCopperCmd](https://github.com/HiRoFa/GreenCopperCmd) is a commandline utility which you can use to test/play with GreenCopperRuntime

[quickjs_runtime](https://github.com/HiRoFa/quickjs_es_runtime) provides a full fledged javascript runtime which includes:
* [x] Promises, from script or from rust
* [x] async/await on eval or Promise resolution
* [x] ES6 Modules
* [x] fetch api
* [x] Proxy classes implemented in rust

## Default implementations

[quickjs_runtime](https://github.com/HiRoFa/quickjs_es_runtime) provides interfaces for the fetch api and module loading, GreenCopperRuntime provides implementations for them
* [x] [FileSystemModuleLoader](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/moduleloaders/struct.FileSystemModuleLoader.html)
* [x] [HTTPModuleLoader](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/moduleloaders/struct.HttpModuleLoader.html)
* [x] [HTTPFetch](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/fetch) (http capable implementation of fetch api)

### Preprocessing

using [quickjs_runtime](https://github.com/HiRoFa/quickjs_es_runtime) 's preprocessing capabilities GreenCopperRuntime provides implementations for:
* [x] cpp style preprocessing (e.g. use #ifdef DEBUG in code)
* [ ] macro's generate script before eval 

The following features are optionally added by specifying them in your Cargo.toml.
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
      * [x] [commonjs](https://hirofa.github.io/GreenCopperRuntime/green_copper_runtime/features/require) (Work in progress)
* [ ] utilities
    * [ ] caching
      * [ ] cache
      * [ ] memcached

 
