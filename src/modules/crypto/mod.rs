use quickjs_runtime::builder::QuickJsRuntimeBuilder;
use quickjs_runtime::jsutils::jsproxies::JsProxy;
use quickjs_runtime::jsutils::JsError;
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;

pub(crate) fn init(builder: QuickJsRuntimeBuilder) -> QuickJsRuntimeBuilder {
    builder.realm_adapter_init_hook(|_rt, realm| {
        // init crypto proxy
        init_crypto_proxy(realm)
    })
}

// todo simple hashes
// https://www.geeksforgeeks.org/node-js-crypto-createhash-method/?ref=lbp

fn init_crypto_proxy(realm: &QuickJsRealmAdapter) -> Result<(), JsError> {
    let crypto_proxy = JsProxy::new().name("crypto").static_method(
        "randomUUID",
        |_rt, realm: &QuickJsRealmAdapter, _args| {
            let uuid = uuid::Uuid::new_v4().to_string();
            realm.create_string(uuid.as_str())
        },
    );
    realm.install_proxy(crypto_proxy, true)?;
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use crate::init_greco_rt;
    use futures::executor::block_on;
    use quickjs_runtime::builder::QuickJsRuntimeBuilder;
    use quickjs_runtime::jsutils::Script;

    #[test]
    fn test_uuid() {
        let rt = init_greco_rt(QuickJsRuntimeBuilder::new()).build();
        let script = Script::new("uuid.js", "crypto.randomUUID();");
        let res = block_on(rt.eval(None, script)).ok().expect("script failed");
        assert!(res.is_string());
        //println!("uuid={}", res.get_str());
        assert_eq!(res.get_str().len(), 36);
    }
}
