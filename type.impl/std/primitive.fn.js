(function() {
    var type_impls = Object.fromEntries([["flate2",[]],["libz_sys",[]],["openssl_sys",[]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[13,16,19]}