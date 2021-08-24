use crate::features::js_fetch::spec::Response;
use hirofa_utils::auto_id_map::AutoIdMap;
use hirofa_utils::js_utils::adapters::proxies::JsProxy;
use hirofa_utils::js_utils::adapters::JsRealmAdapter;
use hirofa_utils::js_utils::JsError;
use std::cell::RefCell;
use std::sync::Arc;

pub(crate) fn impl_for<C>(realm: &C) -> Result<(), JsError>
where
    C: JsRealmAdapter + 'static,
{
    impl_response(realm)?;
    Ok(())
}

thread_local! {
    static RESPONSE_INSTANCES: RefCell<AutoIdMap<Arc<Response>>> = RefCell::new(AutoIdMap::new());
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
    let mut response_proxy = JsProxy::new(&[], "Response");
    response_proxy.set_finalizer(|rt, realm, id| {
        RESPONSE_INSTANCES.with(|rc| {
            let map = &mut *rc.borrow_mut();
            map.remove(id);
        });
    });
    response_proxy.add_method("text", |_rt, realm: &C, instance_id, _args| {
        //
        let response = with_response(instance_id, |response| response.clone())
            .map_err(|s| JsError::new_str(s))?;
        // todo promise may seem futile now but later we will just store bytes in body and encode to string async
        realm.js_promise_create_resolving(
            async move {
                let txt = response.body.text.clone(); // todo impl take in body
                Ok(txt)
            },
            // todo js_string_crea2 with String
            |realm, res| realm.js_string_create(res.as_str()),
        )
    });

    realm.js_proxy_install(response_proxy, false)?;

    Ok(())
}
