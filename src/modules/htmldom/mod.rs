//! HTML Dom module
//!
//! make html parsing available compatible with web based dom mutation
//!
//! e.g.
//! ```javascript
//!  async function test(){
//!     let htmlMod = await import("greco://htmldom");
//!     let parser = new htmlMod.DOMParser();
//!     let html = '<html data-foo="abc"><head></head><body><p>hello world</p></body></html>';
//!     let doc = parser.parseFromString(html);
//!     let res = "";
//!     console.log("attr = %s", doc.documentElement.getAttribute("data-foo"));
//!     console.log("outerHTML = %s", doc.documentElement.outerHTML);
//! };
//! test()
//! ```
//!

use hirofa_utils::auto_id_map::AutoIdMap;
use hirofa_utils::js_utils::adapters::proxies::JsProxy;
use hirofa_utils::js_utils::adapters::{JsRealmAdapter, JsValueAdapter};
use hirofa_utils::js_utils::facades::JsRuntimeBuilder;
use hirofa_utils::js_utils::modules::NativeModuleLoader;
use hirofa_utils::js_utils::JsError;
use kuchiki::traits::TendrilSink;
use kuchiki::NodeRef;
use std::cell::RefCell;

// https://developer.mozilla.org/en-US/docs/Web/API/DOMParser
// https://developer.mozilla.org/en-US/docs/Web/API/Element

// create ts file with api (including Document vs HTMLElement vs Node etc)
// native mod has the following proxies,
// * DOMParser
// * DomNode
// * NodeList
// later
// * DataSet
// * ClassList

struct HtmlDomModuleLoader {}

impl<R: JsRealmAdapter> NativeModuleLoader<R> for HtmlDomModuleLoader {
    fn has_module(&self, _realm: &R, module_name: &str) -> bool {
        module_name.eq("greco://htmldom")
    }

    fn get_module_export_names(&self, _realm: &R, _module_name: &str) -> Vec<&str> {
        vec!["DOMParser", "Node", "NodeList"]
    }

    fn get_module_exports(
        &self,
        realm: &R,
        _module_name: &str,
    ) -> Vec<(&str, R::JsValueAdapterType)> {
        vec![
            (
                "DOMParser",
                init_dom_parser_proxy(realm)
                    .ok()
                    .expect("failed to init DOMParser proxy"),
            ),
            (
                "Node",
                init_node_proxy(realm)
                    .ok()
                    .expect("failed to init Node proxy"),
            ),
            (
                "NodeList",
                init_nodelist_proxy(realm)
                    .ok()
                    .expect("failed to init NodeList proxy"),
            ),
        ]
    }
}

thread_local! {
    static NODES: RefCell<AutoIdMap<NodeRef>> = RefCell::new(AutoIdMap::new());
    static NODELISTS: RefCell<AutoIdMap<Vec<NodeRef>>> = RefCell::new(AutoIdMap::new());
}

fn with_node<R, C: FnOnce(&NodeRef) -> R>(proxy_instance_id: &usize, consumer: C) -> R {
    NODES.with(|rc| {
        let map = &*rc.borrow();
        let node: &NodeRef = map.get(proxy_instance_id).expect("no such Node");
        consumer(node)
    })
}

fn register_node<R: JsRealmAdapter>(
    realm: &R,
    node: NodeRef,
) -> Result<R::JsValueAdapterType, JsError> {
    let id = NODES.with(|rc| {
        let map = &mut *rc.borrow_mut();
        map.insert(node)
    });
    realm.js_proxy_instantiate_with_id(&["greco", "htmldom"], "Node", id)
}

fn _with_node_list<R, C: FnOnce(&Vec<NodeRef>) -> R>(proxy_instance_id: &usize, consumer: C) -> R {
    NODELISTS.with(|rc| {
        let map = &*rc.borrow();
        let nodes: &Vec<NodeRef> = map.get(proxy_instance_id).expect("no such NodeList");
        consumer(nodes)
    })
}

fn parse_from_string(html: &str) -> NodeRef {
    kuchiki::parse_html().one(html)
}

fn init_dom_parser_proxy<R: JsRealmAdapter>(realm: &R) -> Result<R::JsValueAdapterType, JsError> {
    let proxy = JsProxy::new(&["greco", "htmldom"], "DOMParser")
        .set_constructor(|_rt, _realm, _id, _args| Ok(()))
        .add_method("parseFromString", |_rt, realm: &R, _instance_id, args| {
            if !args.len() == 1 || !args[0].js_is_string() {
                Err(JsError::new_str(
                    "parseFromString expects a single string arg",
                ))
            } else {
                let html = args[0].js_to_str()?;
                let doc = parse_from_string(html);
                register_node(realm, doc)
            }
        })
        .add_method("parseFromStringAsync", |_rt, realm, _instance_id, _args| {
            realm.js_null_create()
        });
    realm.js_proxy_install(proxy, false)
}

fn init_node_proxy<R: JsRealmAdapter>(realm: &R) -> Result<R::JsValueAdapterType, JsError> {
    let proxy = JsProxy::new(&["greco", "htmldom"], "Node")
        .add_getter("nodeValue", |_rt, realm: &R, id| {
            with_node(&id, |node| match node.as_text() {
                None => realm.js_null_create(),
                Some(rc) => realm.js_string_create(rc.borrow().as_str()),
            })
        })
        .add_getter("documentElement", |_rt, realm: &R, id| {
            let ret_node = with_node(&id, |node| node.first_child());
            match ret_node {
                None => realm.js_null_create(),
                Some(node) => register_node(realm, node),
            }
        })
        .add_getter("tagName", |_rt, realm: &R, id| {
            with_node(&id, |node| match node.as_element() {
                None => realm.js_null_create(),
                Some(element) => realm.js_string_create(&*element.name.local),
            })
        })
        .add_getter("localName", |_rt, realm: &R, id| {
            with_node(&id, |node| match node.as_element() {
                None => realm.js_null_create(),
                Some(element) => realm.js_string_create(&*element.name.local),
            })
        })
        .add_getter("parentElement", |_rt, realm: &R, id| {
            let ret_node = with_node(&id, |node| node.parent());
            match ret_node {
                None => realm.js_null_create(),
                Some(node) => register_node(realm, node),
            }
        })
        .add_getter("ownerDocument", |_rt, realm: &R, id| {
            let ret_node = with_node(&id, |node| {
                let mut upper = node.clone();
                while let Some(parent) = upper.parent() {
                    upper = parent;
                }
                if upper.as_document().is_some() {
                    Some(upper)
                } else {
                    None
                }
            });
            match ret_node {
                None => realm.js_null_create(),
                Some(node) => register_node(realm, node),
            }
        })
        .add_getter("previousSibling", |_rt, realm: &R, id| {
            let ret_node = with_node(&id, |node| node.previous_sibling());
            match ret_node {
                None => realm.js_null_create(),
                Some(node) => register_node(realm, node),
            }
        })
        .add_getter("nextSibling", |_rt, realm: &R, id| {
            let ret_node = with_node(&id, |node| node.next_sibling());
            match ret_node {
                None => realm.js_null_create(),
                Some(node) => register_node(realm, node),
            }
        })
        .add_getter("firstChild", |_rt, realm: &R, id| {
            let ret_node = with_node(&id, |node| node.first_child());
            match ret_node {
                None => realm.js_null_create(),
                Some(node) => register_node(realm, node),
            }
        })
        .add_getter("lastChild", |_rt, realm: &R, id| {
            let ret_node = with_node(&id, |node| node.last_child());
            match ret_node {
                None => realm.js_null_create(),
                Some(node) => register_node(realm, node),
            }
        })
        .add_getter("outerHTML", |_rt, realm: &R, id| {
            with_node(&id, |node| {
                let mut buf = vec![];
                node.serialize(&mut buf)
                    .map_err(|err| JsError::new_string(format!("serialize failed: {}", err)))?;
                let s = String::from_utf8_lossy(&buf);
                realm.js_string_create(s.to_string().as_str())
            })
        })
        .add_method("setAttribute", |_rt, realm, id, args| {
            if !args.len() == 2 || !args[0].js_is_string() || !args[1].js_is_string() {
                return Err(JsError::new_str("setAttribute expects two string args"));
            }

            let local_name = args[0].js_to_str()?;
            let value = args[1].js_to_string()?;

            with_node(&id, |node| {
                //
                match node.as_element() {
                    None => Err(JsError::new_str("not an Element")),
                    Some(element) => {
                        let attrs = &mut *element.attributes.borrow_mut();

                        attrs.insert(local_name, value);

                        realm.js_null_create()
                    }
                }
            })
        })
        .add_method("getAttribute", |_rt, realm, id, args| {
            if !args.len() == 1 || !args[0].js_is_string() {
                return Err(JsError::new_str("getAttribute expects one string arg"));
            }

            let local_name = args[0].js_to_str()?;

            with_node(&id, |node| {
                //
                match node.as_element() {
                    None => Err(JsError::new_str("not an Element")),
                    Some(element) => {
                        let attrs = &mut *element.attributes.borrow_mut();
                        match attrs.get(local_name) {
                            None => realm.js_null_create(),
                            Some(attr) => realm.js_string_create(attr),
                        }
                    }
                }
            })
        });
    realm.js_proxy_install(proxy, false)
}
fn init_nodelist_proxy<R: JsRealmAdapter>(realm: &R) -> Result<R::JsValueAdapterType, JsError> {
    let proxy = JsProxy::new(&["greco", "htmldom"], "NodeList");
    realm.js_proxy_install(proxy, false)
}

pub(crate) fn init<B: JsRuntimeBuilder>(builder: B) -> B {
    builder.js_native_module_loader(HtmlDomModuleLoader {})
}

#[cfg(test)]
pub mod tests {
    use crate::init_greco_rt;
    use futures::executor::block_on;
    use hirofa_utils::js_utils::facades::values::JsValueFacade;
    use hirofa_utils::js_utils::facades::JsRuntimeFacade;
    use hirofa_utils::js_utils::Script;
    use log::LevelFilter;
    use quickjs_runtime::builder::QuickJsRuntimeBuilder;

    use backtrace::Backtrace;
    use std::panic;

    #[test]
    fn test() {
        panic::set_hook(Box::new(|panic_info| {
            let backtrace = Backtrace::new();
            log::error!(
                "thread panic occurred: {}\nbacktrace: {:?}",
                panic_info,
                backtrace
            );
        }));

        simple_logging::log_to_file("grecort.log", LevelFilter::max())
            .ok()
            .expect("could not init logger");

        let rtb = QuickJsRuntimeBuilder::new();
        let rt = init_greco_rt(rtb).build();

        let code = r#"
        async function test(){
            let htmlMod = await import("greco://htmldom");
            let parser = new htmlMod.DOMParser();
            let html = '<html data-foo="abc"><head></head><body><p>hello world</p></body></html>';
            let doc = parser.parseFromString(html);
            let res = "";
            res += "attr=" + doc.documentElement.getAttribute("data-foo") + "\n";
            res += "html:\n" + doc.documentElement.outerHTML;
            
            return res;
        };
        test()
        "#;

        let promise = block_on(rt.js_eval(None, Script::new("testhtml.js", code)))
            .ok()
            .expect("script failed");
        let rti = rt.js_get_runtime_facade_inner().upgrade().unwrap();
        if let JsValueFacade::JsPromise { cached_promise } = promise {
            let prom_res = block_on(cached_promise.js_get_promise_result(&*rti))
                .ok()
                .expect("promise timed out");

            match prom_res {
                Ok(prom_str_res) => {
                    println!("res: {}", prom_str_res.get_str());
                }
                Err(e) => {
                    println!("err: {}", e.stringify());
                }
            }
        }
    }
}
