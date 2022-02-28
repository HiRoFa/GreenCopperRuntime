use hirofa_utils::js_utils::adapters::proxies::JsProxy;
use hirofa_utils::js_utils::adapters::JsRealmAdapter;
use hirofa_utils::js_utils::facades::JsRuntimeBuilder;
use hirofa_utils::js_utils::JsError;

pub(crate) fn init<B: JsRuntimeBuilder>(builder: B) -> B {
    builder.js_realm_adapter_init_hook(|_rt, realm| {
        // init crypto proxy
        init_crypto_proxy(realm)
    })
}

fn init_crypto_proxy<R: JsRealmAdapter>(realm: &R) -> Result<(), JsError> {
    let crypto_proxy =
        JsProxy::new(&[], "crypto").add_static_method("randomUUID", |_rt, realm: &R, _args| {
            let uuid = uuid::Uuid::new_v4().to_string();
            realm.js_string_create(uuid.as_str())
        });
    realm.js_proxy_install(crypto_proxy, true)?;
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use crate::init_greco_rt;
    use futures::executor::block_on;
    use hirofa_utils::js_utils::facades::JsRuntimeFacade;
    use hirofa_utils::js_utils::Script;
    use quickjs_runtime::builder::QuickJsRuntimeBuilder;

    #[test]
    fn test_uuid() {
        let rt = init_greco_rt(QuickJsRuntimeBuilder::new()).build();
        let script = Script::new("uuid.js", "crypto.randomUUID();");
        let res = block_on(rt.js_eval(None, script))
            .ok()
            .expect("script failed");
        assert!(res.is_string());
        //println!("uuid={}", res.get_str());
        assert_eq!(res.get_str().len(), 36);
    }
}
