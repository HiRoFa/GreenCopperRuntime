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
        .getter("ok", |_rt, realm, instance_id| {
            with_response(instance_id, |response| {
                //
                realm.create_boolean(response.ok)
            })
            // todo with_response is impld sucky
            .unwrap()
        })
        .getter("status", |_rt, realm, instance_id| {
            with_response(instance_id, |response| {
                //
                realm.create_i32(response.status as i32)
            })
            // todo with_response is impld sucky
            .unwrap()
        })
        .method("text", |_rt, realm, instance_id, _args| {
            //
            let response = with_response(instance_id, |response| response.clone())
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
            let response = with_response(instance_id, |response| response.clone())
                .map_err(JsError::new_str)?;
            // todo promise may seem futile now but later we will just store bytes in body and encode to string async
            realm.create_resolving_promise_async(
                async move { response.text().await },
                // todo js_string_crea2 with String
                |realm, res| realm.json_parse(res.as_str()),
            )
        })
        // non std util method, need to impl readablestream and such later
        .method("bytes", |_rt, realm, instance_id, _args| {
            //
            let response = with_response(instance_id, |response| response.clone())
                .map_err(JsError::new_str)?;
            // todo promise may seem futile now but later we will just store bytes in body and encode to string async
            realm.create_resolving_promise_async(
                async move { response.bytes().await },
                // todo js_string_crea2 with String
                |realm, res| realm.create_typed_array_uint8(res),
            )
        })
        // non std header, todo create Headers proxy
        .method("getHeader", |_rt, realm, instance_id, args| {
            //
            let response = with_response(instance_id, |response| response.clone())
                .map_err(JsError::new_str)?;

            if args.is_empty() || !args[0].is_string() {
                return Err(JsError::new_str("getHeader expects a single String arg"));
            }

            let name = args[0].to_string()?;
            if let Some(headers) = response.headers.get(name.as_str()) {
                if !headers.is_empty() {
                    return realm.create_string(headers.first().unwrap());
                }
            }

            realm.create_null()
        });

    realm.install_proxy(response_proxy, false)?;

    Ok(())
}
