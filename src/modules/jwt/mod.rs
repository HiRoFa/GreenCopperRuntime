//! # JWT
//! this module adds support for JWT
//!
//! # Example
//! ```javascript
//! async function test() {
//!     const alg = "EdDSA"; // or RS512
//!        
//!     const jwtMod = await import("greco://jwt");
//!     const key = await jwtMod.generateKey(alg);
//!                 
//!     const payload = {'user': 'somebody', 'obj': 'abcdef', 'privs': ['write', 'read']};
//!     const headers = { alg,  typ: "JWT" };
//!                 
//!     const jwtToken = await jwtMod.create(headers, payload, key);
//!                 
//!     const validatedPayload = await jwtMod.verify(jwtToken, key, alg);
//!     // validatedPayload will be like {"iat":1646137320,"exp":1646223720,"nbf":1646137320,"jti":"3ad1275f-e577-452e-a48f-413b6463b869", "user": "somebody", "obj": "abcdef", "privs": ["write", "read"]}
//!     return(jwtToken + " -> " + JSON.stringify(validatedPayload));
//!                 
//! };
//! ```
//!

use crate::modules::jwt::JwtAlgo::{EdDSA, RS512};
use jwt_simple::prelude::*;
use quickjs_runtime::builder::QuickJsRuntimeBuilder;
use quickjs_runtime::jsutils::modules::NativeModuleLoader;
use quickjs_runtime::jsutils::JsError;
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use quickjs_runtime::quickjsvalueadapter::QuickJsValueAdapter;
use quickjs_runtime::values::JsValueFacade::TypedArray;
use quickjs_runtime::values::{JsValueFacade, TypedArrayType};
use serde_json::Value;
use std::str::FromStr;

struct JwtModuleLoader {}

impl NativeModuleLoader for JwtModuleLoader {
    fn has_module(&self, _realm: &QuickJsRealmAdapter, module_name: &str) -> bool {
        module_name.eq("greco://jwt")
    }

    fn get_module_export_names(
        &self,
        _realm: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<&str> {
        vec!["create", "verify", "generateKey"]
    }

    fn get_module_exports(
        &self,
        realm: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<(&str, QuickJsValueAdapter)> {
        init_exports(realm).expect("init jwt exports failed")
    }
}

pub(crate) fn init(builder: QuickJsRuntimeBuilder) -> QuickJsRuntimeBuilder {
    builder.js_native_module_loader(JwtModuleLoader {})
}

fn init_exports(
    realm: &QuickJsRealmAdapter,
) -> Result<Vec<(&'static str, QuickJsValueAdapter)>, JsError> {
    let create = realm.create_function("create", create, 3)?;
    let verify = realm.create_function_async("verify", verify, 3)?;
    let generate_key = realm.create_function_async("generateKey", generate_key, 1)?;

    Ok(vec![
        ("create", create),
        ("verify", verify),
        ("generateKey", generate_key),
    ])
}

pub enum JwtAlgo {
    EdDSA,
    RS512,
}

impl FromStr for JwtAlgo {
    type Err = JsError;

    fn from_str(s: &str) -> Result<Self, JsError> {
        if s.eq_ignore_ascii_case("rs512") {
            Ok(RS512)
        } else if s.eq_ignore_ascii_case("eddsa") {
            Ok(EdDSA)
        } else {
            Err(JsError::new_str("Unsupported algoritm"))
        }
    }
}

impl ToString for JwtAlgo {
    fn to_string(&self) -> String {
        match self {
            EdDSA => "EdDSA".to_string(),
            RS512 => "Rs512".to_string(),
        }
    }
}

/// create a new JWT token
/// 3 args
/// 0: Object headers
/// 1: Object payload
/// 2: TypedArray key
fn create(
    realm: &QuickJsRealmAdapter,
    _this: &QuickJsValueAdapter,
    args: &[QuickJsValueAdapter],
) -> Result<QuickJsValueAdapter, JsError> {
    if args.len() != 3
        || !args[0].is_object()
        || !args[1].is_object()
        || !args[2].js_is_typed_array()
    {
        Err(JsError::new_str("invalid arguments for create"))
    } else {
        let alg_header = realm.get_object_property(&args[0], "alg")?;
        let alg = if alg_header.is_string() {
            JwtAlgo::from_str(alg_header.js_to_str()?)?
        } else {
            JwtAlgo::EdDSA
        };

        let payload_json = realm.json_stringify(&args[1], None)?;

        // todo create utils so we can borrow the buffer (with_buffer?)
        let key_bytes = realm.copy_typed_array_buffer(&args[2])?;

        realm.create_resolving_promise(
            move || {
                let custom: Value = serde_json::from_str(payload_json.as_str()).map_err(|er| {
                    JsError::new_string(format!("could not parse json payload {er}"))
                })?;

                // todo parse  duration from headers?
                let claims = Claims::with_custom_claims(custom, Duration::from_days(1))
                    .with_jwt_id(uuid::Uuid::new_v4());

                let token = match alg {
                    EdDSA => {
                        let key =
                            Ed25519KeyPair::from_bytes(key_bytes.as_slice()).map_err(|err| {
                                JsError::new_string(format!(
                                    "could not create key from bytes {err}"
                                ))
                            })?;
                        key.sign(claims)
                            .map_err(|err| JsError::new_string(format!("{err}")))?
                    }
                    RS512 => {
                        let key = RS512KeyPair::from_der(key_bytes.as_slice()).map_err(|err| {
                            JsError::new_string(format!("could not create key from bytes {err}"))
                        })?;
                        key.sign(claims)
                            .map_err(|err| JsError::new_string(format!("{err}")))?
                    }
                };

                Ok(token)
            },
            |realm, res| realm.create_string(res.as_str()),
        )
    }
}

/// verify a token and return payload
/// 3 args
/// 0: String token
/// 1: TypedArray key
/// 2: String algorithm
async fn verify(_this: JsValueFacade, args: Vec<JsValueFacade>) -> Result<JsValueFacade, JsError> {
    if !args.len() == 3 || !args[0].is_string() || !args[2].is_string() {
        Err(JsError::new_str("invalid args for verify"))
    } else if let TypedArray {
        buffer: key_bytes,
        array_type: _,
    } = &args[1]
    {
        let token = args[0].get_str();
        let alg = JwtAlgo::from_str(args[2].get_str())?;

        let parsed_claims = match alg {
            EdDSA => {
                let key = Ed25519KeyPair::from_bytes(key_bytes.as_slice()).map_err(|err| {
                    JsError::new_string(format!("could not create key from bytes {err}"))
                })?;
                key.public_key()
                    .verify_token::<Value>(token, None)
                    .map_err(|err| JsError::new_string(format!("{err}")))?
            }
            RS512 => {
                let key = RS512KeyPair::from_der(key_bytes.as_slice()).map_err(|err| {
                    JsError::new_string(format!("could not create key from bytes {err}"))
                })?;
                key.public_key()
                    .verify_token::<Value>(token, None)
                    .map_err(|err| JsError::new_string(format!("could not verify token{err}")))?
            }
        };

        let payload_json = serde_json::to_string(&parsed_claims)
            .map_err(|err| JsError::new_string(format!("could not serialize claims {err}")))?;

        Ok(JsValueFacade::JsonStr { json: payload_json })
    } else {
        Err(JsError::new_str("invalid args for verify"))
    }
}

/// generate a new key and return as typedarray
/// 1 arg, for key type RS512 or EdDSA
async fn generate_key(
    _this: JsValueFacade,
    args: Vec<JsValueFacade>,
) -> Result<JsValueFacade, JsError> {
    let key_bytes = if !args.is_empty() && args[0].is_string() {
        let alg = JwtAlgo::from_str(args[0].get_str())?;
        match alg {
            RS512 => Ok::<Vec<u8>, JsError>(
                RS512KeyPair::generate(4096)
                    .map_err(|err| {
                        JsError::new_string(format!("could not create RS512 keypair {err}"))
                    })?
                    .to_der()
                    .map_err(|err| {
                        JsError::new_string(format!("could not create RS512 keypair2 {err}"))
                    })?,
            ),
            EdDSA => Ok(Ed25519KeyPair::generate().to_bytes()),
        }?
    } else {
        Ed25519KeyPair::generate().to_bytes()
    };

    let res = JsValueFacade::TypedArray {
        buffer: key_bytes,
        array_type: TypedArrayType::Uint8,
    };
    Ok(res)
}

#[cfg(test)]
pub mod tests {
    use crate::init_greco_rt;
    use futures::executor::block_on;
    use quickjs_runtime::builder::QuickJsRuntimeBuilder;
    use quickjs_runtime::jsutils::Script;
    use quickjs_runtime::values::JsValueFacade;

    #[test]
    fn test_uuid() {
        let rt = init_greco_rt(QuickJsRuntimeBuilder::new()).build();
        let script = Script::new(
            "uuid.js",
            r#"
            async function test() {
            
                const alg = "EdDSA";
            
                const jwtMod = await import("greco://jwt");
                const key = await jwtMod.generateKey(alg);
                
                const payload = {'user': 'somebody', 'obj': 'abcdef', 'privs': ['write', 'read']};
                const headers = { alg,  typ: "JWT" };
                
                const jwtToken = await jwtMod.create(headers, payload, key);
                
                //
                
                const validatedPayload = await jwtMod.verify(jwtToken, key, alg);
                
                return(jwtToken + " -> " + JSON.stringify(validatedPayload));
                
            };
            test();
        "#,
        );
        let res = block_on(rt.eval(None, script)).ok().expect("script failed");

        if let JsValueFacade::JsPromise { cached_promise } = res {
            let prom_res = block_on(cached_promise.js_get_promise_result())
                .ok()
                .expect("promise timed out");

            match prom_res {
                Ok(res) => {
                    let s = res.get_str();
                    println!("jwt test res was {s}");
                }
                Err(err) => {
                    panic!("prmise was rejected {}", err.stringify());
                }
            }
        } else {
            panic!("not a promise");
        }
    }
}
