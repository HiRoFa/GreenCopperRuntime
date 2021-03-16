(function() {var implementors = {};
implementors["gpio_cdev"] = [{"text":"impl AsRawFd for LineHandle","synthetic":false,"types":[]},{"text":"impl AsRawFd for MultiLineHandle","synthetic":false,"types":[]},{"text":"impl AsRawFd for LineEventHandle","synthetic":false,"types":[]}];
implementors["nix"] = [{"text":"impl AsRawFd for Dir","synthetic":false,"types":[]},{"text":"impl AsRawFd for PtyMaster","synthetic":false,"types":[]},{"text":"impl AsRawFd for SignalFd","synthetic":false,"types":[]},{"text":"impl AsRawFd for Inotify","synthetic":false,"types":[]}];
implementors["socket2"] = [{"text":"impl AsRawFd for Socket","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()