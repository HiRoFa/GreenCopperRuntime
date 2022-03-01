use hirofa_utils::js_utils::adapters::JsRealmAdapter;
use hirofa_utils::js_utils::facades::values::{JsValueFacade, TypedArrayType};
use hirofa_utils::js_utils::facades::JsRuntimeBuilder;
use hirofa_utils::js_utils::modules::NativeModuleLoader;
use hirofa_utils::js_utils::JsError;

struct JwtModuleLoader {}

impl<R: JsRealmAdapter + 'static> NativeModuleLoader<R> for JwtModuleLoader {
    fn has_module(&self, _realm: &R, module_name: &str) -> bool {
        module_name.eq("greco://jwt")
    }

    fn get_module_export_names(&self, _realm: &R, _module_name: &str) -> Vec<&str> {
        vec!["create", "verify", "decode", "generateKey"]
    }

    fn get_module_exports(
        &self,
        realm: &R,
        _module_name: &str,
    ) -> Vec<(&str, R::JsValueAdapterType)> {
        init_exports(realm).ok().expect("init jwt exports failed")
    }
}

pub(crate) fn init<B: JsRuntimeBuilder>(builder: B) -> B {
    builder.js_native_module_loader(JwtModuleLoader {})
}

fn init_exports<R: JsRealmAdapter + 'static>(
    realm: &R,
) -> Result<Vec<(&'static str, R::JsValueAdapterType)>, JsError> {
    let create = realm.js_function_create_async("create", create, 3)?;
    let verify = realm.js_function_create_async("verify", verify, 3)?;
    let decode = realm.js_function_create_async("decode", decode, 3)?;
    let generate_key = realm.js_function_create_async("generateKey", generate_key, 1)?;

    Ok(vec![
        ("create", create),
        ("verify", verify),
        ("decode", decode),
        ("generateKey", generate_key),
    ])
}

async fn create(_this: JsValueFacade, _args: Vec<JsValueFacade>) -> Result<JsValueFacade, JsError> {
    Ok(JsValueFacade::Null)
}

async fn verify(_this: JsValueFacade, _args: Vec<JsValueFacade>) -> Result<JsValueFacade, JsError> {
    Ok(JsValueFacade::Null)
}

async fn decode(_this: JsValueFacade, _args: Vec<JsValueFacade>) -> Result<JsValueFacade, JsError> {
    Ok(JsValueFacade::Null)
}

async fn generate_key(
    _this: JsValueFacade,
    _args: Vec<JsValueFacade>,
) -> Result<JsValueFacade, JsError> {
    let _res = JsValueFacade::TypedArray {
        buffer: vec![],
        array_type: TypedArrayType::Uint8,
    };
    Ok(JsValueFacade::Null)
}
