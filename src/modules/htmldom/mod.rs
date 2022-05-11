//! HTML Dom module
//!
//! make html parsing available compatible with web based dom mutation
//!
//! e.g.
//! ```javascript
//!  async function test(){
//!     let htmlMod = await import("greco://htmldom"); // or if you use ts: "https://raw.githubusercontent.com/HiRoFa/GreenCopperRuntime/main/modules/dom/htmldom.ts"
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
use html5ever::LocalName;
use html5ever::Namespace;
use html5ever::QualName;
use kuchiki::iter::Siblings;
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
        vec!["DOMParser", "Node", "NodeList", "ElementList"]
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
            (
                "ElementList",
                init_elementlist_proxy(realm)
                    .ok()
                    .expect("failed to init ElementList proxy"),
            ),
        ]
    }
}

type NodeList = Siblings;
type ElementList = Siblings;

thread_local! {
    static NODES: RefCell<AutoIdMap<NodeRef>> = RefCell::new(AutoIdMap::new());
    static NODELISTS: RefCell<AutoIdMap<NodeList>> = RefCell::new(AutoIdMap::new());
    static ELEMENTLISTS: RefCell<AutoIdMap<ElementList>> = RefCell::new(AutoIdMap::new());
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

fn register_node_list<R: JsRealmAdapter>(
    realm: &R,
    node_list: NodeList,
) -> Result<R::JsValueAdapterType, JsError> {
    let id = NODELISTS.with(|rc| {
        let map = &mut *rc.borrow_mut();
        map.insert(node_list)
    });
    realm.js_proxy_instantiate_with_id(&["greco", "htmldom"], "NodeList", id)
}

fn register_element_list<R: JsRealmAdapter>(
    realm: &R,
    element_list: ElementList,
) -> Result<R::JsValueAdapterType, JsError> {
    let id = ELEMENTLISTS.with(|rc| {
        let map = &mut *rc.borrow_mut();
        map.insert(element_list)
    });
    realm.js_proxy_instantiate_with_id(&["greco", "htmldom"], "ElementList", id)
}

fn with_node_list<R, C: FnOnce(&NodeList) -> R>(proxy_instance_id: &usize, consumer: C) -> R {
    NODELISTS.with(|rc| {
        let map = &*rc.borrow();
        let nodes: &NodeList = map.get(proxy_instance_id).expect("no such NodeList");
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
        .add_getter("childNodes", |_rt, realm: &R, id| {
            with_node(&id, |node| register_node_list(realm, node.children()))
        })
        .add_getter("children", |_rt, realm: &R, id| {
            with_node(&id, |node| register_element_list(realm, node.children()))
        })
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
        })
        .add_method("appendChild", |_rt, realm, id, args| {
            //
            if args.len() != 1 || !args[0].js_is_proxy_instance() {
                return Err(JsError::new_str(
                    "appendChild expects a single Node argument",
                ));
            }
            let p_data = realm.js_proxy_instance_get_info(&args[0])?;
            if !p_data.0.eq("greco.htmldom.Node") {
                return Err(JsError::new_str(
                    "appendChild expects a single Node argument",
                ));
            }

            let child = with_node(&p_data.1, |child| child.clone());

            with_node(&id, |node| match node.as_element() {
                None => Err(JsError::new_str("Node was not an Element")),
                Some(_element) => {
                    node.append(child);
                    Ok(args[0].clone())
                }
            })
        })
        .add_method("createElement", |_rt, realm, id, args| {
            //

            if args.len() < 1 || !args[0].js_is_string() {
                return Err(JsError::new_str(
                    "createElement expects a single string argument",
                ));
            }

            let tag_name = args[0].js_to_str()?;

            let res = with_node(&id, |node| match node.as_document() {
                None => Err(JsError::new_str("not a Document")),
                Some(_document) => {
                    // todo create some static consts for ns/qualname
                    let q_name = QualName::new(
                        None,
                        Namespace::from("http://www.w3.org/1999/xhtml"),
                        LocalName::from(tag_name),
                    );
                    let new_node = NodeRef::new_element(q_name, vec![]);
                    Ok(new_node)
                }
            });
            match res {
                Ok(node) => register_node(realm, node),
                Err(e) => Err(e),
            }
        })
        .add_method("createTextNode", |_rt, realm, id, args| {
            //

            if args.len() < 1 || !args[0].js_is_string() {
                return Err(JsError::new_str(
                    "createTextNode expects a single string argument",
                ));
            }

            let content = args[0].js_to_string()?;

            let res = with_node(&id, |node| match node.as_document() {
                None => Err(JsError::new_str("not a Document")),
                Some(_document) => {
                    let new_node = NodeRef::new_text(content);
                    Ok(new_node)
                }
            });
            match res {
                Ok(node) => register_node(realm, node),
                Err(e) => Err(e),
            }
        });
    realm.js_proxy_install(proxy, false)
}

fn init_nodelist_proxy<R: JsRealmAdapter>(realm: &R) -> Result<R::JsValueAdapterType, JsError> {
    let proxy = JsProxy::new(&["greco", "htmldom"], "NodeList")
        .add_getter("length", |_rt, realm: &R, id| {
            with_node_list(&id, |node_list| {
                realm.js_i32_create(node_list.clone().count() as i32)
            })
        })
        .add_method("Symbol.iterator", |_rt, realm, id, _args| {
            //
            // this should be considered a hack, it only works in quicksj, we need Iterable support in utils::JsProxy

            // return an object with a next func, (clone NodeList and move to clusure)
            // next func should return an object with {done: false|true, value: null | nextVal}

            let obj = realm.js_object_create()?;

            let node_list_ref = RefCell::new(with_node_list(&id, |node_list| node_list.clone()));

            let next_func = realm.js_function_create(
                "next",
                move |realm: &R, _this, _args| {
                    //
                    let ret_obj = realm.js_object_create()?;
                    let node_list = &mut *node_list_ref.borrow_mut();
                    match node_list.next() {
                        None => {
                            realm.js_object_set_property(
                                &ret_obj,
                                "done",
                                &realm.js_boolean_create(true)?,
                            )?;
                        }
                        Some(node) => {
                            realm.js_object_set_property(
                                &ret_obj,
                                "done",
                                &realm.js_boolean_create(false)?,
                            )?;
                            realm.js_object_set_property(
                                &ret_obj,
                                "value",
                                &register_node(realm, node)?,
                            )?;
                        }
                    }
                    Ok(ret_obj)
                },
                0,
            )?;
            realm.js_object_set_property(&obj, "next", &next_func)?;

            Ok(obj)
        });
    realm.js_proxy_install(proxy, false)
}

fn init_elementlist_proxy<R: JsRealmAdapter>(realm: &R) -> Result<R::JsValueAdapterType, JsError> {
    let proxy = JsProxy::new(&["greco", "htmldom"], "ElementList")
        .add_getter("length", |_rt, realm: &R, id| {
            with_node_list(&id, |node_list| {
                realm.js_i32_create(node_list.clone().count() as i32)
            })
        })
        .add_method("Symbol.iterator", |_rt, realm, id, _args| {
            //
            // this should be considered a hack, it only works in quicksj, we need Iterable support in utils::JsProxy

            // return an object with a next func, (clone NodeList and move to clusure)
            // next func should return an object with {done: false|true, value: null | nextVal}

            let obj = realm.js_object_create()?;

            let node_list_ref = RefCell::new(with_node_list(&id, |node_list| node_list.clone()));

            let next_func = realm.js_function_create(
                "next",
                move |realm: &R, _this, _args| {
                    //
                    let ret_obj = realm.js_object_create()?;
                    let node_list = &mut *node_list_ref.borrow_mut();

                    let mut next: Option<NodeRef> = node_list.next();
                    while next.is_some() && next.as_ref().unwrap().as_element().is_none() {
                        next = node_list.next();
                    }

                    match next {
                        None => {
                            realm.js_object_set_property(
                                &ret_obj,
                                "done",
                                &realm.js_boolean_create(true)?,
                            )?;
                        }
                        Some(node) => {
                            realm.js_object_set_property(
                                &ret_obj,
                                "done",
                                &realm.js_boolean_create(false)?,
                            )?;
                            realm.js_object_set_property(
                                &ret_obj,
                                "value",
                                &register_node(realm, node)?,
                            )?;
                        }
                    }
                    Ok(ret_obj)
                },
                0,
            )?;
            realm.js_object_set_property(&obj, "next", &next_func)?;

            Ok(obj)
        });
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
            let html = '<html data-foo="abc"><head></head><body><p>hello</p><p>>world</p></body></html>';
            let doc = parser.parseFromString(html);
            let res = "";
            res += "attr=" + doc.documentElement.getAttribute("data-foo") + "\n";
            res += "html:\n" + doc.documentElement.outerHTML;
            
            let body = doc.documentElement.lastChild;
            let nodeList = body.childNodes;
            
            res += "\nnodeList.length = " + nodeList.length;
            
            let thirdP = doc.createElement("p");
            body.appendChild(thirdP);
            
            res += "\nnodeList.length after p added = " + nodeList.length;
            nodeList = body.childNodes;
            res += "\nnodeList.length after p added = " + nodeList.length;
                        
            res += "\nhtml:\n" + doc.documentElement.outerHTML;
            
            for (let node of nodeList) {
                res += "\nnode.tagName = " + node.tagName;
            }
            
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
