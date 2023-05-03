use crate::features::js_fetch::spec::Response;
use quickjs_runtime::jsutils::jsproxies::JsProxy;
use quickjs_runtime::jsutils::JsError;
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) fn impl_for(realm: &QuickJsRealmAdapter) -> Result<(), JsError> {
    impl_response(realm)?;
    Ok(())
}

thread_local! {
    pub(crate) static RESPONSE_INSTANCES: RefCell<HashMap<usize, Arc<Response>>> = RefCell::new(HashMap::new());
}

fn with_response<C: FnOnce(&Arc<Response>) -> R, R>(id: &usize, consumer: C) -> Result<R, &str> {
    RESPONSE_INSTANCES.with(|rc| {
        let map = &*rc.borrow();
        if let Some(response) = map.get(id) {
            Ok(consumer(response))
        } else {
            Err("instance not found")
        }
    })
}

fn impl_response(realm: &QuickJsRealmAdapter) -> Result<(), JsError> {
    let response_proxy = JsProxy::new()
        .namespace(&[])
        .name("Response")
        .finalizer(|_rt, _realm, id| {
            // todo.. need to use realm id as part of key?
            RESPONSE_INSTANCES.with(|rc| {
                let map = &mut *rc.borrow_mut();
                map.remove(&id);
            });
        })
        .method("text", |_rt, realm, instance_id, _args| {
            //
            let response = with_response(&instance_id, |response| response.clone())
                .map_err(JsError::new_str)?;
            // todo promise may seem futile now but later we will just store bytes in body and encode to string async
            realm.create_resolving_promise_async(
                async move { response.text().await },
                // todo js_string_crea2 with String
                |realm, res| realm.create_string(res.as_str()),
            )
        })
        .method("json", |_rt, realm, instance_id, _args| {
            //
            let response = with_response(&instance_id, |response| response.clone())
                .map_err(JsError::new_str)?;
            // todo promise may seem futile now but later we will just store bytes in body and encode to string async
            realm.create_resolving_promise_async(
                async move { response.text().await },
                // todo js_string_crea2 with String
                |realm, res| realm.json_parse(res.as_str()),
            )
        });

    realm.install_proxy(response_proxy, false)?;

    Ok(())
}
