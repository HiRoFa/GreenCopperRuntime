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
use html5ever::LocalName;
use html5ever::Namespace;
use html5ever::QualName;
use kuchiki::iter::{Elements, NodeIterator, Siblings};
use kuchiki::traits::TendrilSink;
use kuchiki::{parse_html, NodeRef};
use quickjs_runtime::builder::QuickJsRuntimeBuilder;
use quickjs_runtime::jsutils::jsproxies::JsProxy;
use quickjs_runtime::jsutils::modules::NativeModuleLoader;
use quickjs_runtime::jsutils::JsError;
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use quickjs_runtime::quickjsvalueadapter::QuickJsValueAdapter;
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

impl NativeModuleLoader for HtmlDomModuleLoader {
    fn has_module(&self, _realm: &QuickJsRealmAdapter, module_name: &str) -> bool {
        module_name.eq("greco://htmldom")
    }

    fn get_module_export_names(
        &self,
        _realm: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<&str> {
        vec![
            "DOMParser",
            "Node",
            "NodeList",
            "ElementList",
            "SelectElementList",
        ]
    }

    fn get_module_exports(
        &self,
        realm: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<(&str, QuickJsValueAdapter)> {
        vec![
            (
                "DOMParser",
                init_dom_parser_proxy(realm).expect("failed to init DOMParser proxy"),
            ),
            (
                "Node",
                init_node_proxy(realm).expect("failed to init Node proxy"),
            ),
            (
                "NodeList",
                init_nodelist_proxy(realm).expect("failed to init NodeList proxy"),
            ),
            (
                "ElementList",
                init_elementlist_proxy(realm).expect("failed to init ElementList proxy"),
            ),
            (
                "SelectElementList",
                init_select_elementlist_proxy(realm)
                    .expect("failed to init SelectElementList proxy"),
            ),
        ]
    }
}

struct SelectBase {
    node: NodeRef,
    selectors: String,
}

type NodeList = Siblings;
type ElementList = Elements<Siblings>;
type SelectElementList = SelectBase;

thread_local! {
    static NODES: RefCell<AutoIdMap<NodeRef>> = RefCell::new(AutoIdMap::new());
    static NODELISTS: RefCell<AutoIdMap<NodeList>> = RefCell::new(AutoIdMap::new());
    static ELEMENTLISTS: RefCell<AutoIdMap<ElementList>> = RefCell::new(AutoIdMap::new());
    static SELECTELEMENTLISTS: RefCell<AutoIdMap<SelectElementList >> = RefCell::new(AutoIdMap::new());
}

fn with_node<R, C: FnOnce(&NodeRef) -> R>(proxy_instance_id: &usize, consumer: C) -> R {
    NODES.with(|rc| {
        let map = &*rc.borrow();
        let node: &NodeRef = map.get(proxy_instance_id).expect("no such Node");
        consumer(node)
    })
}

fn register_node(
    realm: &QuickJsRealmAdapter,
    node: NodeRef,
) -> Result<QuickJsValueAdapter, JsError> {
    // todo need native quickjs stuff here..
    // keep separate map with NodeRef as key
    // point at JsValueRef (dont increment refcount for those)
    // remove on finalize (dont decrement refcount :))
    // reuse here to create a new JsValueAdapter (and then increment refcount)

    let id = NODES.with(|rc| {
        let map = &mut *rc.borrow_mut();
        map.insert(node)
    });
    realm.instantiate_proxy_with_id(&["greco", "htmldom"], "Node", id)
}

fn register_node_list(
    realm: &QuickJsRealmAdapter,
    node_list: NodeList,
) -> Result<QuickJsValueAdapter, JsError> {
    let id = NODELISTS.with(|rc| {
        let map = &mut *rc.borrow_mut();
        map.insert(node_list)
    });
    realm.instantiate_proxy_with_id(&["greco", "htmldom"], "NodeList", id)
}

fn register_element_list(
    realm: &QuickJsRealmAdapter,
    element_list: ElementList,
) -> Result<QuickJsValueAdapter, JsError> {
    let id = ELEMENTLISTS.with(|rc| {
        let map = &mut *rc.borrow_mut();
        map.insert(element_list)
    });
    realm.instantiate_proxy_with_id(&["greco", "htmldom"], "ElementList", id)
}

fn register_select_element_list(
    realm: &QuickJsRealmAdapter,
    select_element_list: SelectElementList,
) -> Result<QuickJsValueAdapter, JsError> {
    let id = SELECTELEMENTLISTS.with(|rc| {
        let map = &mut *rc.borrow_mut();
        map.insert(select_element_list)
    });
    realm.instantiate_proxy_with_id(&["greco", "htmldom"], "SelectElementList", id)
}

fn with_node_list<R, C: FnOnce(&NodeList) -> R>(proxy_instance_id: &usize, consumer: C) -> R {
    NODELISTS.with(|rc| {
        let map = &*rc.borrow();
        let nodes: &NodeList = map.get(proxy_instance_id).expect("no such NodeList");
        consumer(nodes)
    })
}

fn with_element_list<R, C: FnOnce(&ElementList) -> R>(proxy_instance_id: &usize, consumer: C) -> R {
    ELEMENTLISTS.with(|rc| {
        let map = &*rc.borrow();
        let nodes: &ElementList = map.get(proxy_instance_id).expect("no such ElementList");
        consumer(nodes)
    })
}

fn with_select_element_list<R, C: FnOnce(&SelectElementList) -> R>(
    proxy_instance_id: &usize,
    consumer: C,
) -> R {
    SELECTELEMENTLISTS.with(|rc| {
        let map = &*rc.borrow();
        let nodes: &SelectElementList = map
            .get(proxy_instance_id)
            .expect("no such SelectElementList");
        consumer(nodes)
    })
}

fn parse_from_string(html: &str) -> NodeRef {
    kuchiki::parse_html().one(html)
}

fn init_dom_parser_proxy(realm: &QuickJsRealmAdapter) -> Result<QuickJsValueAdapter, JsError> {
    let proxy = JsProxy::new()
        .namespace(&["greco", "htmldom"])
        .name("DOMParser")
        .constructor(|_rt, _realm, _id, _args| Ok(()))
        .method("parseFromString", |_rt, realm, _instance_id, args| {
            if !args.len() == 1 || !(args[0].is_string() || args[0].js_is_typed_array()) {
                Err(JsError::new_str(
                    "parseFromString expects a single string arg",
                ))
            } else {
                let doc = if args[0].is_string() {
                    let html = args[0].js_to_str()?;
                    parse_from_string(html)
                } else {
                    let bytes = realm.detach_typed_array_buffer(&args[0])?;
                    let html = String::from_utf8_lossy(bytes.as_slice());
                    parse_from_string(html.to_string().as_str())
                };

                register_node(realm, doc)
            }
        })
        .method("parseFromStringAsync", |_rt, realm, _instance_id, _args| {
            realm.create_null()
        });
    realm.install_proxy(proxy, true)
}

fn init_node_proxy(realm: &QuickJsRealmAdapter) -> Result<QuickJsValueAdapter, JsError> {
    let proxy = JsProxy::new()
        .namespace(&["greco", "htmldom"])
        .name("Node")
        .finalizer(|_rt, _realm, id| {
            NODES.with(|rc| {
                let map = &mut rc.borrow_mut();
                map.remove(&id);
            })
        })
        .getter("childNodes", |_rt, realm, id| {
            with_node(&id, |node| register_node_list(realm, node.children()))
        })
        .getter("children", |_rt, realm, id| {
            with_node(&id, |node| {
                register_element_list(realm, node.children().elements())
            })
        })
        .getter("nodeValue", |_rt, realm, id| {
            with_node(&id, |node| match node.as_text() {
                None => realm.create_null(),
                Some(rc) => realm.create_string(rc.borrow().as_str()),
            })
        })
        .getter("nodeName", |_rt, realm, id| {
            with_node(&id, |node| match node.as_element() {
                None => realm.create_null(),
                Some(element) => realm.create_string(&element.name.local.to_uppercase()),
            })
        })
        .getter("documentElement", |_rt, realm, id| {
            let ret_node = with_node(&id, |node| node.first_child());
            match ret_node {
                None => realm.create_null(),
                Some(node) => register_node(realm, node),
            }
        })
        .getter("tagName", |_rt, realm, id| {
            with_node(&id, |node| match node.as_element() {
                None => realm.create_null(),
                Some(element) => realm.create_string(&element.name.local),
            })
        })
        .getter("localName", |_rt, realm, id| {
            with_node(&id, |node| match node.as_element() {
                None => realm.create_null(),
                Some(element) => realm.create_string(&element.name.local),
            })
        })
        .getter("parentElement", |_rt, realm, id| {
            let ret_node = with_node(&id, |node| node.parent());
            match ret_node {
                None => realm.create_null(),
                Some(node) => register_node(realm, node),
            }
        })
        .getter("ownerDocument", |_rt, realm, id| {
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
                None => realm.create_null(),
                Some(node) => register_node(realm, node),
            }
        })
        .getter("previousSibling", |_rt, realm, id| {
            let ret_node = with_node(&id, |node| node.previous_sibling());
            match ret_node {
                None => realm.create_null(),
                Some(node) => register_node(realm, node),
            }
        })
        .getter("nextSibling", |_rt, realm, id| {
            let ret_node = with_node(&id, |node| node.next_sibling());
            match ret_node {
                None => realm.create_null(),
                Some(node) => register_node(realm, node),
            }
        })
        .getter("nextElementSibling", |_rt, realm, id| {
            let ret_node = with_node(&id, |node| {
                let mut next = node.next_sibling();
                while next.is_some() && next.as_ref().unwrap().as_element().is_none() {
                    next = next.unwrap().next_sibling();
                }
                next
            });
            match ret_node {
                None => realm.create_null(),
                Some(node) => register_node(realm, node),
            }
        })
        .getter("previousElementSibling", |_rt, realm, id| {
            let ret_node = with_node(&id, |node| {
                let mut prev = node.previous_sibling();
                while prev.is_some() && prev.as_ref().unwrap().as_element().is_none() {
                    prev = prev.unwrap().previous_sibling();
                }
                prev
            });
            match ret_node {
                None => realm.create_null(),
                Some(node) => register_node(realm, node),
            }
        })
        .getter("firstChild", |_rt, realm, id| {
            let ret_node = with_node(&id, |node| node.first_child());
            match ret_node {
                None => realm.create_null(),
                Some(node) => register_node(realm, node),
            }
        })
        .getter("firstElementChild", |_rt, realm, id| {
            let ret_node = with_node(&id, |node| {
                let mut fc = node.first_child();
                while fc.is_some() && fc.as_ref().unwrap().as_element().is_none() {
                    fc = fc.unwrap().next_sibling();
                }
                fc
            });
            match ret_node {
                None => realm.create_null(),
                Some(node) => register_node(realm, node),
            }
        })
        .getter("lastChild", |_rt, realm, id| {
            let ret_node = with_node(&id, |node| node.last_child());
            match ret_node {
                None => realm.create_null(),
                Some(node) => register_node(realm, node),
            }
        })
        .getter("outerHTML", |_rt, realm, id| {
            with_node(&id, |node| {
                let mut buf = vec![];
                node.serialize(&mut buf)
                    .map_err(|err| JsError::new_string(format!("serialize failed: {err}")))?;
                let s = String::from_utf8_lossy(&buf);
                realm.create_string(s.to_string().as_str())
            })
        })
        .method("encodeHTML", |_rt, realm, id, _args| {
            with_node(&id, |node| {
                let mut buf = vec![];
                node.serialize(&mut buf)
                    .map_err(|err| JsError::new_string(format!("serialize failed: {err}")))?;
                realm.create_typed_array_uint8(buf)
            })
        })
        .method("getBoundingClientRect", |_rt, realm, id, _args| {
            with_node(&id, |node| {
                let width = get_num_attr(node, "width", 800)?;
                let height = get_num_attr(node, "height", 600)?;

                let ret_obj = realm.create_object()?;
                realm.set_object_property(&ret_obj, "width", &realm.create_i32(width)?)?;
                realm.set_object_property(&ret_obj, "height", &realm.create_i32(height)?)?;

                Ok(ret_obj)
            })
        })
        .getter_setter(
            "innerHTML",
            |_rt, realm, id| {
                with_node(&id, |node| {
                    let mut buf = vec![];
                    for child in node.children() {
                        child.serialize(&mut buf).map_err(|err| {
                            JsError::new_string(format!("serialize failed: {err}"))
                        })?;
                    }

                    let s = String::from_utf8_lossy(&buf);
                    realm.create_string(s.to_string().as_str())
                })
            },
            |_rt, _realm, id, val| {
                if !val.is_string() {
                    return Err(JsError::new_str("innerHTML should be a string"));
                }

                let html = val.js_to_str()?;

                with_node(&id, |node| {
                    while let Some(child) = node.first_child() {
                        // todo do i need to do this recursively?
                        child.detach();
                        child.parent().take();
                    }

                    // todo actually use fragment and don't get a full html doc? or is there another faster way..?
                    let document_frag = parse_html().one(html);
                    let body = document_frag.first_child().unwrap().last_child().unwrap();
                    while let Some(new_child) = body.first_child() {
                        node.append(new_child);
                    }
                });
                Ok(())
            },
        )
        .method("setAttribute", |_rt, realm, id, args| {
            if !args.len() == 2
                || !args[0].is_string()
                || !(args[1].is_string() || args[1].is_null_or_undefined())
            {
                return Err(JsError::new_str("setAttribute expects two string args"));
            }

            let local_name = args[0].js_to_str()?;
            let value = if args[1].is_string() {
                Some(args[1].js_to_string()?)
            } else {
                None
            };

            with_node(&id, |node| {
                //
                match node.as_element() {
                    None => Err(JsError::new_str("not an Element")),
                    Some(element) => {
                        let attrs = &mut *element.attributes.borrow_mut();

                        if let Some(value) = value {
                            attrs.insert(local_name, value);
                        } else {
                            attrs.remove(local_name);
                        }
                        realm.create_null()
                    }
                }
            })
        })
        .method("setAttributeNS", |_rt, realm, id, args| {
            if !args.len() == 3
                || !args[0].is_string()
                || !(args[1].is_string())
                || !(args[2].is_string() || args[2].is_null_or_undefined())
            {
                return Err(JsError::new_str("setAttributeNS expects three string args"));
            }

            let _namespace = args[0].js_to_str()?;
            let local_name = args[1].js_to_str()?;
            let value = if args[2].is_string() {
                Some(args[2].js_to_string()?)
            } else {
                None
            };

            with_node(&id, |node| {
                //
                match node.as_element() {
                    None => Err(JsError::new_str("not an Element")),
                    Some(element) => {
                        let attrs = &mut *element.attributes.borrow_mut();

                        if let Some(value) = value {
                            attrs.insert(local_name, value);
                        } else {
                            attrs.remove(local_name);
                        }
                        realm.create_null()
                    }
                }
            })
        })
        .method("equals", |_rt, realm, id, args| {
            if args.len() != 1 || !args[0].js_is_proxy_instance() {
                return Err(JsError::new_str("equals expects a single Node arg"));
            }

            let p_data = realm.get_proxy_instance_info(&args[0])?;
            if !p_data.0.eq("greco.htmldom.Node") {
                return Err(JsError::new_str("equals expects a single Node argument"));
            }

            let compare_node = with_node(&p_data.1, |child| child.clone());

            with_node(&id, |node| {
                //
                realm.create_boolean(node.eq(&compare_node))
            })
        })
        .getter_setter(
            "className",
            |_rt, realm, id| {
                //
                with_node(&id, |node| {
                    //
                    if let Some(element) = node.as_element() {
                        let attrs = &mut *element.attributes.borrow_mut();
                        match attrs.get("class") {
                            None => realm.create_string(""),
                            Some(attr) => realm.create_string(attr),
                        }
                    } else {
                        realm.create_undefined()
                    }
                })
            },
            |_rt, _realm, id, value| {
                //
                with_node(&id, |node| {
                    //
                    if let Some(element) = node.as_element() {
                        let attrs = &mut *element.attributes.borrow_mut();
                        if value.is_string() {
                            let cn = value.js_to_string()?;
                            attrs.insert("class", cn);
                        }
                    }
                    Ok(())
                })
            },
        )
        .method("getAttribute", |_rt, realm, id, args| {
            if !args.len() == 1 || !args[0].is_string() {
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
                            None => realm.create_null(),
                            Some(attr) => realm.create_string(attr),
                        }
                    }
                }
            })
        })
        .method("querySelector", |_rt, realm, id, args| {
            if !args.len() == 1 || !args[0].is_string() {
                return Err(JsError::new_str("querySelector expects one string arg"));
            }

            let selectors = args[0].js_to_str()?;

            let res = with_node(&id, |node| {
                //
                let result = node.select_first(selectors);
                match result {
                    Ok(ndr) => Some(ndr.as_node().clone()),
                    Err(_) => None,
                }
            });
            match res {
                None => realm.create_null(),
                Some(node) => register_node(realm, node),
            }
        })
        .method("querySelectorAll", |_rt, realm, id, args| {
            if !args.len() == 1 || !args[0].is_string() {
                return Err(JsError::new_str("querySelectorAll expects one string arg"));
            }
            let selectors = args[0].js_to_string()?;
            let elements = with_node(&id, |node| SelectBase {
                selectors,
                node: node.clone(),
            });
            register_select_element_list(realm, elements)
        })
        .method("appendChild", |_rt, realm, id, args| {
            //
            if args.len() != 1 || !args[0].js_is_proxy_instance() {
                return Err(JsError::new_str(
                    "appendChild expects a single Node argument",
                ));
            }
            let p_data = realm.get_proxy_instance_info(&args[0])?;
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
        .method("removeChild", |_rt, realm, id, args| {
            // todo, calling this twice by mistake leads to other children being removed

            if args.len() != 1 || !args[0].js_is_proxy_instance() {
                return Err(JsError::new_str(
                    "removeChild expects a single Node argument",
                ));
            }
            let p_data = realm.get_proxy_instance_info(&args[0])?;
            if !p_data.0.eq("greco.htmldom.Node") {
                return Err(JsError::new_str(
                    "removeChild expects a single Node argument",
                ));
            }

            let child = with_node(&p_data.1, |child| child.clone());

            with_node(&id, |node| match node.as_element() {
                None => Err(JsError::new_str("Node was not an Element")),

                Some(_element) => {
                    child.detach();
                    let _ = child.parent().take();
                    Ok(args[0].clone())
                }
            })
        })
        .method("replaceChild", |_rt, realm, id, args| {
            //
            if args.len() != 2 || !args[0].js_is_proxy_instance() || !args[1].js_is_proxy_instance()
            {
                return Err(JsError::new_str(
                    "replaceChild expects two Node arguments (newChild, oldChild)",
                ));
            }

            let p_data_new_child = realm.get_proxy_instance_info(&args[0])?;
            if !p_data_new_child.0.eq("greco.htmldom.Node") {
                return Err(JsError::new_str(
                    "replaceChild expects two Node arguments (newChild, oldChild)",
                ));
            }

            let new_child = with_node(&p_data_new_child.1, |child| child.clone());

            let p_data_old_child = realm.get_proxy_instance_info(&args[1])?;
            if !p_data_old_child.0.eq("greco.htmldom.Node") {
                return Err(JsError::new_str(
                    "replaceChild expects two Node arguments (newChild, oldChild)",
                ));
            }

            let old_child = with_node(&p_data_old_child.1, |child| child.clone());

            with_node(&id, |node| match node.as_element() {
                None => Err(JsError::new_str("Node was not an Element")),
                Some(_element) => {
                    old_child.insert_before(new_child);
                    old_child.detach();
                    let _ = old_child.parent().take();
                    Ok(args[1].clone())
                }
            })
        })
        .method("insertBefore", |_rt, realm, id, args| {
            //
            if args.len() != 2 || !args[0].js_is_proxy_instance() || !args[1].js_is_proxy_instance()
            {
                return Err(JsError::new_str(
                    "insertBefore expects two Node arguments (newNode, referenceNode)",
                ));
            }

            let p_data_new_node = realm.get_proxy_instance_info(&args[0])?;
            if !p_data_new_node.0.eq("greco.htmldom.Node") {
                return Err(JsError::new_str(
                    "insertBefore expects two Node arguments (newNode, referenceNode)",
                ));
            }

            let new_node = with_node(&p_data_new_node.1, |child| child.clone());

            let p_data_reference_node = realm.get_proxy_instance_info(&args[1])?;
            if !p_data_reference_node.0.eq("greco.htmldom.Node") {
                return Err(JsError::new_str(
                    "insertBefore expects two Node arguments (newNode, referenceNode)",
                ));
            }

            let reference_node = with_node(&p_data_reference_node.1, |child| child.clone());

            with_node(&id, |node| match node.as_element() {
                None => Err(JsError::new_str("Node was not an Element")),
                Some(_element) => {
                    reference_node.insert_before(new_node);
                    Ok(args[0].clone())
                }
            })
        })
        .method("insertAfter", |_rt, realm, id, args| {
            //
            if args.len() != 2 || !args[0].js_is_proxy_instance() || !args[1].js_is_proxy_instance()
            {
                return Err(JsError::new_str(
                    "insertAfter expects two Node arguments (newNode, referenceNode)",
                ));
            }

            let p_data_new_node = realm.get_proxy_instance_info(&args[0])?;
            if !p_data_new_node.0.eq("greco.htmldom.Node") {
                return Err(JsError::new_str(
                    "insertAfter expects two Node arguments (newNode, referenceNode)",
                ));
            }

            let new_node = with_node(&p_data_new_node.1, |child| child.clone());

            let p_data_reference_node = realm.get_proxy_instance_info(&args[1])?;
            if !p_data_reference_node.0.eq("greco.htmldom.Node") {
                return Err(JsError::new_str(
                    "insertAfter expects two Node arguments (newNode, referenceNode)",
                ));
            }

            let reference_node = with_node(&p_data_reference_node.1, |child| child.clone());

            with_node(&id, |node| match node.as_element() {
                None => Err(JsError::new_str("Node was not an Element")),
                Some(_element) => {
                    reference_node.insert_after(new_node);
                    Ok(args[0].clone())
                }
            })
        })
        .method("createElement", |_rt, realm, id, args| {
            //

            if args.is_empty() || !args[0].is_string() {
                return Err(JsError::new_str(
                    "createElement expects a single string argument",
                ));
            }

            let tag_name = args[0].js_to_str()?;

            let res = with_node(&id, |node| match node.as_document() {
                None => Err(JsError::new_str("not a Document")),
                Some(_document) => {
                    let q_name =
                        QualName::new(None, Namespace::from(""), LocalName::from(tag_name));
                    let new_node = NodeRef::new_element(q_name, vec![]);
                    Ok(new_node)
                }
            });
            match res {
                Ok(node) => register_node(realm, node),
                Err(e) => Err(e),
            }
        })
        .method("createElementNS", |_rt, realm, id, args| {
            //

            if args.is_empty() || !args[0].is_string() || !args[1].is_string() {
                return Err(JsError::new_str(
                    "createElementNS expects a two string arguments",
                ));
            }

            let namespace_uri = args[0].js_to_str()?;
            let qualified_name = args[1].js_to_str()?;

            let res = with_node(&id, |node| match node.as_document() {
                None => Err(JsError::new_str("not a Document")),
                Some(_document) => {
                    let q_name = QualName::new(
                        None,
                        Namespace::from(namespace_uri),
                        LocalName::from(qualified_name),
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
        .method("getElementById", |_rt, realm, id, args| {
            //

            if args.is_empty() || !args[0].is_string() {
                return Err(JsError::new_str(
                    "getElementById expects a single string argument",
                ));
            }

            let id_attr = args[0].js_to_str()?;

            let res = with_node(&id, |node| match node.as_document() {
                None => Err(JsError::new_str("not a Document")),
                Some(_document) => {
                    let node_res = node.select_first(format!("#{id_attr}").as_str());
                    Ok(node_res)
                }
            });
            match res {
                Ok(node_res) => match node_res {
                    Ok(node) => register_node(realm, node.as_node().clone()),
                    Err(_) => realm.create_null(),
                },
                Err(e) => Err(e),
            }
        })
        .method("createTextNode", |_rt, realm, id, args| {
            //

            if args.is_empty() || !args[0].is_string() {
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
    realm.install_proxy(proxy, true)
}

fn get_num_attr(node: &NodeRef, attr_name: &str, default_value: i32) -> Result<i32, JsError> {
    let mut cur_node = node.clone();

    loop {
        if let Some(element_data) = cur_node.as_element() {
            let attrs = element_data.attributes.borrow();
            let attr = attrs.get(attr_name);

            if let Some(attr_str) = attr {
                if !attr_str.eq("100%") {
                    if attr_str.ends_with("px") {
                        if let Some(n) = &attr_str[0..attr_str.len() - 2].parse::<i32>().ok() {
                            return Ok(*n);
                        }
                    }

                    if let Some(n) = &attr_str.parse::<i32>().ok() {
                        return Ok(*n);
                    }
                }
            }
        } else {
            return Err(JsError::new_str("Not an Element"));
        }

        if let Some(p) = cur_node.parent() {
            cur_node = p;
        } else {
            break;
        }
    }
    Ok(default_value)
}

fn init_nodelist_proxy(realm: &QuickJsRealmAdapter) -> Result<QuickJsValueAdapter, JsError> {
    let proxy = JsProxy::new()
        .namespace(&["greco", "htmldom"])
        .name("NodeList")
        .finalizer(|_rt, _realm, id| {
            NODELISTS.with(|rc| {
                let map = &mut rc.borrow_mut();
                map.remove(&id);
            })
        })
        .getter("length", |_rt, realm, id| {
            with_node_list(&id, |node_list| {
                realm.create_i32(node_list.clone().count() as i32)
            })
        })
        .method("Symbol.iterator", |_rt, realm, id, _args| {
            //
            // this should be considered a hack, it only works in quicksj, we need Iterable support in utils::JsProxy

            // return an object with a next func, (clone NodeList and move to clusure)
            // next func should return an object with {done: false|true, value: null | nextVal}

            let obj = realm.create_object()?;

            let node_list_ref = RefCell::new(with_node_list(&id, |node_list| node_list.clone()));

            let next_func = realm.create_function(
                "next",
                move |realm, _this, _args| {
                    //
                    let ret_obj = realm.create_object()?;
                    let node_list = &mut *node_list_ref.borrow_mut();
                    match node_list.next() {
                        None => {
                            realm.set_object_property(
                                &ret_obj,
                                "done",
                                &realm.create_boolean(true)?,
                            )?;
                        }
                        Some(node) => {
                            realm.set_object_property(
                                &ret_obj,
                                "done",
                                &realm.create_boolean(false)?,
                            )?;
                            realm.set_object_property(
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
            realm.set_object_property(&obj, "next", &next_func)?;

            Ok(obj)
        });
    realm.install_proxy(proxy, true)
}

fn init_elementlist_proxy(realm: &QuickJsRealmAdapter) -> Result<QuickJsValueAdapter, JsError> {
    let proxy = JsProxy::new()
        .namespace(&["greco", "htmldom"])
        .name("ElementList")
        .finalizer(|_rt, _realm, id| {
            ELEMENTLISTS.with(|rc| {
                let map = &mut rc.borrow_mut();
                map.remove(&id);
            })
        })
        .getter("length", |_rt, realm, id| {
            with_element_list(&id, |node_list| {
                realm.create_i32(node_list.clone().count() as i32)
            })
        })
        .method("Symbol.iterator", |_rt, realm, id, _args| {
            //
            // this should be considered a hack, it only works in quicksj, we need Iterable support in utils::JsProxy

            // return an object with a next func, (clone NodeList and move to clusure)
            // next func should return an object with {done: false|true, value: null | nextVal}

            let obj = realm.create_object()?;

            let element_list_ref =
                RefCell::new(with_element_list(&id, |element_list| element_list.clone()));

            let next_func = realm.create_function(
                "next",
                move |realm, _this, _args| {
                    //
                    let ret_obj = realm.create_object()?;
                    let element_list = &mut *element_list_ref.borrow_mut();

                    let next = element_list.next();

                    match next {
                        None => {
                            realm.set_object_property(
                                &ret_obj,
                                "done",
                                &realm.create_boolean(true)?,
                            )?;
                        }
                        Some(node) => {
                            realm.set_object_property(
                                &ret_obj,
                                "done",
                                &realm.create_boolean(false)?,
                            )?;
                            realm.set_object_property(
                                &ret_obj,
                                "value",
                                &register_node(realm, node.as_node().clone())?,
                            )?;
                        }
                    }
                    Ok(ret_obj)
                },
                0,
            )?;
            realm.set_object_property(&obj, "next", &next_func)?;

            Ok(obj)
        });
    realm.install_proxy(proxy, true)
}

fn init_select_elementlist_proxy(
    realm: &QuickJsRealmAdapter,
) -> Result<QuickJsValueAdapter, JsError> {
    let proxy = JsProxy::new()
        .namespace(&["greco", "htmldom"])
        .name("SelectElementList")
        .finalizer(|_rt, _realm, id| {
            SELECTELEMENTLISTS.with(|rc| {
                let map = &mut rc.borrow_mut();
                map.remove(&id);
            })
        })
        .getter("length", |_rt, realm, id| {
            with_select_element_list(&id, |select_base| {
                let select_res = select_base.node.select(select_base.selectors.as_str());
                match select_res {
                    Ok(select) => realm.create_i32(select.count() as i32),
                    Err(_) => realm.create_i32(0),
                }
            })
        })
        .method("Symbol.iterator", |_rt, realm, id, _args| {
            //
            // this should be considered a hack, it only works in quicksj, we need Iterable support in utils::JsProxy

            // return an object with a next func, (clone NodeList and move to clusure)
            // next func should return an object with {done: false|true, value: null | nextVal}

            let obj = realm.create_object()?;

            let select_opt_ref = RefCell::new(with_select_element_list(&id, |select_base| {
                let select_res = select_base.node.select(select_base.selectors.as_str());
                match select_res {
                    Ok(select) => Some(select),
                    Err(_) => None,
                }
            }));

            let next_func = realm.create_function(
                "next",
                move |realm, _this, _args| {
                    //
                    let ret_obj = realm.create_object()?;
                    let select_opt = &mut *select_opt_ref.borrow_mut();

                    let next = match select_opt {
                        None => None,
                        Some(select) => select.next(),
                    };

                    match next {
                        None => {
                            realm.set_object_property(
                                &ret_obj,
                                "done",
                                &realm.create_boolean(true)?,
                            )?;
                        }
                        Some(node) => {
                            realm.set_object_property(
                                &ret_obj,
                                "done",
                                &realm.create_boolean(false)?,
                            )?;
                            realm.set_object_property(
                                &ret_obj,
                                "value",
                                &register_node(realm, node.as_node().clone())?,
                            )?;
                        }
                    }
                    Ok(ret_obj)
                },
                0,
            )?;
            realm.set_object_property(&obj, "next", &next_func)?;

            Ok(obj)
        });
    realm.install_proxy(proxy, true)
}

pub(crate) fn init(builder: QuickJsRuntimeBuilder) -> QuickJsRuntimeBuilder {
    builder
        .js_realm_adapter_init_hook(|_rt, _realm| {
            // init
            // document.createElement (moet Element of SVGElement of HTMLDivElement of HTMLTableElement etc etc teruggeven)
            // document.createElementNS
            // document.createTextNode
            // document.createComment
            // document.createCDATASection
            // Element
            // SVGElement

            Ok(())
        })
        .js_native_module_loader(HtmlDomModuleLoader {})
}

#[cfg(test)]
pub mod tests {
    use crate::init_greco_rt;
    use futures::executor::block_on;
    use log::LevelFilter;
    use quickjs_runtime::builder::QuickJsRuntimeBuilder;

    use backtrace::Backtrace;
    use quickjs_runtime::jsutils::Script;
    use quickjs_runtime::values::JsValueFacade;
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
            let html = '<html xmlns:svg="http://www.w3.org/2000/svg" data-foo="abc"><head></head><body class="bodyc1 bodyc2">text<p id="helloId">hello</p><p class="worldly">world</p></body></html>';
            let doc = parser.parseFromString(html);
            let res = "";
            
            let helloNode = doc.getElementById("helloId");
            
            const svg = doc.createElementNS("http://www.w3.org/2000/svg", "svg");
            helloNode.appendChild(svg);
            
            res += helloNode?"\nhelloNode.innerHTML = "+helloNode.innerHTML:"\nhello node not found";
            
            res += "\nattr=" + doc.documentElement.getAttribute("data-foo") + "\n";
            res += "html:\n" + doc.documentElement.outerHTML;
            
            let body = doc.documentElement.lastChild;
            let nodeList = body.childNodes;
            
            res += "\nnodeList.length = " + nodeList.length;
            
            let thirdP = doc.createElement("p");
            body.appendChild(thirdP);
            
            let worldP = doc.querySelector("p.worldly");
            res += "\nworldP = " + (worldP?worldP.outerHTML:"not found");

            let allPs = doc.querySelectorAll("p");
            res += "\nallPs = " + allPs.length;
            for (let p of allPs) {
                res += "\nallPs = " + p.outerHTML;
            }
            
            res += "\n\nnodeList.length after p added = " + nodeList.length;
            nodeList = body.childNodes;
            res += "\nnodeList.length after p added = " + nodeList.length;
                        
            res += "\nhtml:\n" + doc.documentElement.outerHTML;
            
            res += "\nchildNodes:"
            for (let node of nodeList) {
                res += "\nnode.tagName = " + node.tagName;
            }
            
            nodeList = body.children;
            res += "\nchildren:"
            for (let node of nodeList) {
                res += "\nnode.tagName = " + node.tagName + "["+node.innerHTML+"]";
            }
            
            res += "\nbody.innerHTML=" + body.innerHTML;
            
            body.innerHTML = "";
            
            res += "\nbody.outerHTML=" + body.outerHTML;
            
            body.innerHTML = "<span>two </span><span>spans</span>";
            
            res += "\nbody.outerHTML=" + body.outerHTML;
            
            res += "\nbody.className = " + body.className;
            
            
            
            return res;
        };
        test()
        "#;

        let promise = block_on(rt.eval(None, Script::new("testhtml.js", code)))
            .ok()
            .expect("script failed");
        if let JsValueFacade::JsPromise { cached_promise } = promise {
            let prom_res =
                block_on(cached_promise.js_get_promise_result()).expect("promise timed out");

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
