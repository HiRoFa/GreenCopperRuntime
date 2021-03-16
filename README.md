# GreenCopperRuntime

**Just to get thing clear straight away, this is a very much work in progress project, nothing is definitive, it might never become definitive**

GreenCopperRuntime is a library for which provides as JavaScript runtime (based on the [quickjs_runtime](https://github.com/HiRoFa/quickjs_es_runtime) project) with additional features.

quickjs_runtime provides a full fledged javascript runtime which includes:
* [x] Promises, from script or from rust
* [x] async/await on eval or Promise resolution
* [x] ES6 Modules
* [x] fetch api
* [x] Proxy classes implemented in rust

The quickjs_runtime provides interfaces for the fetch api and module loading, GreenCopperRuntime provides implementations for them
* [x] FileSystemModuleLoader
* [x] HTTPModuleLoader
* [x] HTTPFetch

The following features are optionally added by specifying them in your Cargo.toml.
* [ ] DB
    * [ ] Mysql / TiDb [MYSQL](docs/db/MYSQL.md) (Work in progress)
    * [ ] Couchbase
    * [ ] Cassandra
    * [ ] Redis
* [ ] COM
    * [ ] [HTTP Client](docs/com/HTTP.md)
    * [ ] [Socket Client](docs/com/SOCKET.md)
* [ ] IO
    * [x] [GPIO](docs/io/GPIO.md) (Work in progress)
    * [ ] [FileIO](docs/io/FS.md)
    * [ ] USB access
    * [ ] camera control
* [ ] LibLoading
    * [ ] Rust/C
    * [ ] Java
    * [ ] JS/NPM
* [ ] Utilities
    * [ ] Caching
      * [ ] LocalCaching
      * [ ] Memcached

[GreenCopperCmd](https://github.com/HiRoFa/GreenCopperCmd) is a commandline utility which you can use to test/play with GreenCopperRuntime 