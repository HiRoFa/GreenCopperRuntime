use crate::features::js_fetch::spec::Response;
use hirofa_utils::js_utils::adapters::proxies::JsProxy;
use hirofa_utils::js_utils::adapters::JsRealmAdapter;
use hirofa_utils::js_utils::JsError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) fn impl_for<C>(realm: &C) -> Result<(), JsError>
where
    C: JsRealmAdapter + 'static,
{
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

fn impl_response<C>(realm: &C) -> Result<(), JsError>
where
    C: JsRealmAdapter + 'static,
{
    let mut response_proxy = JsProxy::new(&[], "Response")
        .set_finalizer(|_rt, _realm, id| {
            // todo.. need to use realm id as part of key?
            RESPONSE_INSTANCES.with(|rc| {
                let map = &mut *rc.borrow_mut();
                map.remove(&id);
            });
        })
        .add_method("text", |_rt, realm: &C, instance_id, _args| {
            //
            let response = with_response(&instance_id, |response| response.clone())
                .map_err(|s| JsError::new_str(s))?;
            // todo promise may seem futile now but later we will just store bytes in body and encode to string async
            realm.js_promise_create_resolving_async(
                async move { response.text().await },
                // todo js_string_crea2 with String
                |realm, res| realm.js_string_create(res.as_str()),
            )
        })
        .add_method("json", |_rt, realm: &C, instance_id, _args| {
            //
            let response = with_response(&instance_id, |response| response.clone())
                .map_err(|s| JsError::new_str(s))?;
            // todo promise may seem futile now but later we will just store bytes in body and encode to string async
            realm.js_promise_create_resolving_async(
                async move { response.text().await },
                // todo js_string_crea2 with String
                |realm, res| realm.js_json_parse(res.as_str()),
            )
        });

    realm.js_proxy_install(response_proxy, false)?;

    Ok(())
}
