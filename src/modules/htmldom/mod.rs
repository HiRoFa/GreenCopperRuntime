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
use kuchiki::iter::{Elements, NodeIterator, Siblings};
use kuchiki::traits::TendrilSink;
use kuchiki::{parse_html, NodeRef};
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
        realm: &R,
        _module_name: &str,
    ) -> Vec<(&str, R::JsValueAdapterType)> {
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

fn register_node<R: JsRealmAdapter>(
    realm: &R,
    node: NodeRef,
) -> Result<R::JsValueAdapterType, JsError> {
    // todo need native quickjs stuff here..
    // keep separate map with NodeRef as key
    // point at JsValueRef (dont increment refcount for those)
    // remove on finalize (dont decrement refcount :))
    // reuse here to create a new JsValueAdapter (and then increment refcount)

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

fn register_select_element_list<R: JsRealmAdapter>(
    realm: &R,
    select_element_list: SelectElementList,
) -> Result<R::JsValueAdapterType, JsError> {
    let id = SELECTELEMENTLISTS.with(|rc| {
        let map = &mut *rc.borrow_mut();
        map.insert(select_element_list)
    });
    realm.js_proxy_instantiate_with_id(&["greco", "htmldom"], "SelectElementList", id)
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

fn init_dom_parser_proxy<R: JsRealmAdapter>(realm: &R) -> Result<R::JsValueAdapterType, JsError> {
    let proxy = JsProxy::new(&["greco", "htmldom"], "DOMParser")
        .set_constructor(|_rt, _realm, _id, _args| Ok(()))
        .add_method("parseFromString", |_rt, realm: &R, _instance_id, args| {
            if !args.len() == 1 || !(args[0].js_is_string() || args[0].js_is_typed_array()) {
                Err(JsError::new_str(
                    "parseFromString expects a single string arg",
                ))
            } else {
                let doc = if args[0].js_is_string() {
                    let html = args[0].js_to_str()?;
                    parse_from_string(html)
                } else {
                    let bytes = realm.js_typed_array_detach_buffer(&args[0])?;
                    let html = String::from_utf8_lossy(bytes.as_slice());
                    parse_from_string(html.to_string().as_str())
                };

                register_node(realm, doc)
            }
        })
        .add_method("parseFromStringAsync", |_rt, realm, _instance_id, _args| {
            realm.js_null_create()
        });
    realm.js_proxy_install(proxy, true)
}

fn init_node_proxy<R: JsRealmAdapter>(realm: &R) -> Result<R::JsValueAdapterType, JsError> {
    let proxy = JsProxy::new(&["greco", "htmldom"], "Node")
        .set_finalizer(|_rt, _realm, id| {
            NODES.with(|rc| {
                let map = &mut rc.borrow_mut();
                map.remove(&id);
            })
        })
        .add_getter("childNodes", |_rt, realm: &R, id| {
            with_node(&id, |node| register_node_list(realm, node.children()))
        })
        .add_getter("children", |_rt, realm: &R, id| {
            with_node(&id, |node| {
                register_element_list(realm, node.children().elements())
            })
        })
        .add_getter("nodeValue", |_rt, realm: &R, id| {
            with_node(&id, |node| match node.as_text() {
                None => realm.js_null_create(),
                Some(rc) => realm.js_string_create(rc.borrow().as_str()),
            })
        })
        .add_getter("nodeName", |_rt, realm: &R, id| {
            with_node(&id, |node| match node.as_element() {
                None => realm.js_null_create(),
                Some(element) => realm.js_string_create(&element.name.local.to_uppercase()),
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
                Some(element) => realm.js_string_create(&element.name.local),
            })
        })
        .add_getter("localName", |_rt, realm: &R, id| {
            with_node(&id, |node| match node.as_element() {
                None => realm.js_null_create(),
                Some(element) => realm.js_string_create(&element.name.local),
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
        .add_getter("nextElementSibling", |_rt, realm: &R, id| {
            let ret_node = with_node(&id, |node| {
                let mut next = node.next_sibling();
                while next.is_some() && next.as_ref().unwrap().as_element().is_none() {
                    next = next.unwrap().next_sibling();
                }
                next
            });
            match ret_node {
                None => realm.js_null_create(),
                Some(node) => register_node(realm, node),
            }
        })
        .add_getter("previousElementSibling", |_rt, realm: &R, id| {
            let ret_node = with_node(&id, |node| {
                let mut prev = node.previous_sibling();
                while prev.is_some() && prev.as_ref().unwrap().as_element().is_none() {
                    prev = prev.unwrap().previous_sibling();
                }
                prev
            });
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
        .add_getter("firstElementChild", |_rt, realm: &R, id| {
            let ret_node = with_node(&id, |node| {
                let mut fc = node.first_child();
                while fc.is_some() && fc.as_ref().unwrap().as_element().is_none() {
                    fc = fc.unwrap().next_sibling();
                }
                fc
            });
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
                    .map_err(|err| JsError::new_string(format!("serialize failed: {err}")))?;
                let s = String::from_utf8_lossy(&buf);
                realm.js_string_create(s.to_string().as_str())
            })
        })
        .add_method("encodeHTML", |_rt, realm: &R, id, _args| {
            with_node(&id, |node| {
                let mut buf = vec![];
                node.serialize(&mut buf)
                    .map_err(|err| JsError::new_string(format!("serialize failed: {err}")))?;
                realm.js_typed_array_uint8_create(buf)
            })
        })
        .add_method("getBoundingClientRect", |_rt, realm: &R, id, _args| {
            with_node(&id, |node| {
                let width = get_num_attr(node, "width", 800)?;
                let height = get_num_attr(node, "height", 600)?;

                let ret_obj = realm.js_object_create()?;
                realm.js_object_set_property(&ret_obj, "width", &realm.js_i32_create(width)?)?;
                realm.js_object_set_property(&ret_obj, "height", &realm.js_i32_create(height)?)?;

                Ok(ret_obj)
            })
        })
        .add_getter_setter(
            "innerHTML",
            |_rt, realm: &R, id| {
                with_node(&id, |node| {
                    let mut buf = vec![];
                    for child in node.children() {
                        child.serialize(&mut buf).map_err(|err| {
                            JsError::new_string(format!("serialize failed: {err}"))
                        })?;
                    }

                    let s = String::from_utf8_lossy(&buf);
                    realm.js_string_create(s.to_string().as_str())
                })
            },
            |_rt, _realm: &R, id, val| {
                if !val.js_is_string() {
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
        .add_method("setAttribute", |_rt, realm, id, args| {
            if !args.len() == 2
                || !args[0].js_is_string()
                || !(args[1].js_is_string() || args[1].js_is_null_or_undefined())
            {
                return Err(JsError::new_str("setAttribute expects two string args"));
            }

            let local_name = args[0].js_to_str()?;
            let value = if args[1].js_is_string() {
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
                        realm.js_null_create()
                    }
                }
            })
        })
        .add_method("setAttributeNS", |_rt, realm, id, args| {
            if !args.len() == 3
                || !args[0].js_is_string()
                || !(args[1].js_is_string())
                || !(args[2].js_is_string() || args[2].js_is_null_or_undefined())
            {
                return Err(JsError::new_str("setAttributeNS expects three string args"));
            }

            let _namespace = args[0].js_to_str()?;
            let local_name = args[1].js_to_str()?;
            let value = if args[2].js_is_string() {
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
                        realm.js_null_create()
                    }
                }
            })
        })
        .add_method("equals", |_rt, realm, id, args| {
            if args.len() != 1 || !args[0].js_is_proxy_instance() {
                return Err(JsError::new_str("equals expects a single Node arg"));
            }

            let p_data = realm.js_proxy_instance_get_info(&args[0])?;
            if !p_data.0.eq("greco.htmldom.Node") {
                return Err(JsError::new_str("equals expects a single Node argument"));
            }

            let compare_node = with_node(&p_data.1, |child| child.clone());

            with_node(&id, |node| {
                //
                realm.js_boolean_create(node.eq(&compare_node))
            })
        })
        .add_getter_setter(
            "className",
            |_rt, realm, id| {
                //
                with_node(&id, |node| {
                    //
                    if let Some(element) = node.as_element() {
                        let attrs = &mut *element.attributes.borrow_mut();
                        match attrs.get("class") {
                            None => realm.js_string_create(""),
                            Some(attr) => realm.js_string_create(attr),
                        }
                    } else {
                        realm.js_undefined_create()
                    }
                })
            },
            |_rt, _realm, id, value| {
                //
                with_node(&id, |node| {
                    //
                    if let Some(element) = node.as_element() {
                        let attrs = &mut *element.attributes.borrow_mut();
                        if value.js_is_string() {
                            let cn = value.js_to_string()?;
                            attrs.insert("class", cn);
                        }
                    }
                    Ok(())
                })
            },
        )
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
        .add_method("querySelector", |_rt, realm, id, args| {
            if !args.len() == 1 || !args[0].js_is_string() {
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
                None => realm.js_null_create(),
                Some(node) => register_node(realm, node),
            }
        })
        .add_method("querySelectorAll", |_rt, realm, id, args| {
            if !args.len() == 1 || !args[0].js_is_string() {
                return Err(JsError::new_str("querySelectorAll expects one string arg"));
            }
            let selectors = args[0].js_to_string()?;
            let elements = with_node(&id, |node| SelectBase {
                selectors,
                node: node.clone(),
            });
            register_select_element_list(realm, elements)
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
        .add_method("removeChild", |_rt, realm, id, args| {
            // todo, calling this twice by mistake leads to other children being removed

            if args.len() != 1 || !args[0].js_is_proxy_instance() {
                return Err(JsError::new_str(
                    "removeChild expects a single Node argument",
                ));
            }
            let p_data = realm.js_proxy_instance_get_info(&args[0])?;
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
        .add_method("replaceChild", |_rt, realm, id, args| {
            //
            if args.len() != 2 || !args[0].js_is_proxy_instance() || !args[1].js_is_proxy_instance()
            {
                return Err(JsError::new_str(
                    "replaceChild expects two Node arguments (newChild, oldChild)",
                ));
            }

            let p_data_new_child = realm.js_proxy_instance_get_info(&args[0])?;
            if !p_data_new_child.0.eq("greco.htmldom.Node") {
                return Err(JsError::new_str(
                    "replaceChild expects two Node arguments (newChild, oldChild)",
                ));
            }

            let new_child = with_node(&p_data_new_child.1, |child| child.clone());

            let p_data_old_child = realm.js_proxy_instance_get_info(&args[1])?;
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
        .add_method("insertBefore", |_rt, realm, id, args| {
            //
            if args.len() != 2 || !args[0].js_is_proxy_instance() || !args[1].js_is_proxy_instance()
            {
                return Err(JsError::new_str(
                    "insertBefore expects two Node arguments (newNode, referenceNode)",
                ));
            }

            let p_data_new_node = realm.js_proxy_instance_get_info(&args[0])?;
            if !p_data_new_node.0.eq("greco.htmldom.Node") {
                return Err(JsError::new_str(
                    "insertBefore expects two Node arguments (newNode, referenceNode)",
                ));
            }

            let new_node = with_node(&p_data_new_node.1, |child| child.clone());

            let p_data_reference_node = realm.js_proxy_instance_get_info(&args[1])?;
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
        .add_method("insertAfter", |_rt, realm, id, args| {
            //
            if args.len() != 2 || !args[0].js_is_proxy_instance() || !args[1].js_is_proxy_instance()
            {
                return Err(JsError::new_str(
                    "insertAfter expects two Node arguments (newNode, referenceNode)",
                ));
            }

            let p_data_new_node = realm.js_proxy_instance_get_info(&args[0])?;
            if !p_data_new_node.0.eq("greco.htmldom.Node") {
                return Err(JsError::new_str(
                    "insertAfter expects two Node arguments (newNode, referenceNode)",
                ));
            }

            let new_node = with_node(&p_data_new_node.1, |child| child.clone());

            let p_data_reference_node = realm.js_proxy_instance_get_info(&args[1])?;
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
        .add_method("createElement", |_rt, realm, id, args| {
            //

            if args.is_empty() || !args[0].js_is_string() {
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
        .add_method("createElementNS", |_rt, realm, id, args| {
            //

            if args.is_empty() || !args[0].js_is_string() || !args[1].js_is_string() {
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
        .add_method("getElementById", |_rt, realm, id, args| {
            //

            if args.is_empty() || !args[0].js_is_string() {
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
                    Err(_) => realm.js_null_create(),
                },
                Err(e) => Err(e),
            }
        })
        .add_method("createTextNode", |_rt, realm, id, args| {
            //

            if args.is_empty() || !args[0].js_is_string() {
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
    realm.js_proxy_install(proxy, true)
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

fn init_nodelist_proxy<R: JsRealmAdapter>(realm: &R) -> Result<R::JsValueAdapterType, JsError> {
    let proxy = JsProxy::new(&["greco", "htmldom"], "NodeList")
        .set_finalizer(|_rt, _realm, id| {
            NODELISTS.with(|rc| {
                let map = &mut rc.borrow_mut();
                map.remove(&id);
            })
        })
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
    realm.js_proxy_install(proxy, true)
}

fn init_elementlist_proxy<R: JsRealmAdapter>(realm: &R) -> Result<R::JsValueAdapterType, JsError> {
    let proxy = JsProxy::new(&["greco", "htmldom"], "ElementList")
        .set_finalizer(|_rt, _realm, id| {
            ELEMENTLISTS.with(|rc| {
                let map = &mut rc.borrow_mut();
                map.remove(&id);
            })
        })
        .add_getter("length", |_rt, realm: &R, id| {
            with_element_list(&id, |node_list| {
                realm.js_i32_create(node_list.clone().count() as i32)
            })
        })
        .add_method("Symbol.iterator", |_rt, realm, id, _args| {
            //
            // this should be considered a hack, it only works in quicksj, we need Iterable support in utils::JsProxy

            // return an object with a next func, (clone NodeList and move to clusure)
            // next func should return an object with {done: false|true, value: null | nextVal}

            let obj = realm.js_object_create()?;

            let element_list_ref =
                RefCell::new(with_element_list(&id, |element_list| element_list.clone()));

            let next_func = realm.js_function_create(
                "next",
                move |realm: &R, _this, _args| {
                    //
                    let ret_obj = realm.js_object_create()?;
                    let element_list = &mut *element_list_ref.borrow_mut();

                    let next = element_list.next();

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
                                &register_node(realm, node.as_node().clone())?,
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
    realm.js_proxy_install(proxy, true)
}

fn init_select_elementlist_proxy<R: JsRealmAdapter>(
    realm: &R,
) -> Result<R::JsValueAdapterType, JsError> {
    let proxy = JsProxy::new(&["greco", "htmldom"], "SelectElementList")
        .set_finalizer(|_rt, _realm, id| {
            SELECTELEMENTLISTS.with(|rc| {
                let map = &mut rc.borrow_mut();
                map.remove(&id);
            })
        })
        .add_getter("length", |_rt, realm: &R, id| {
            with_select_element_list(&id, |select_base| {
                let select_res = select_base.node.select(select_base.selectors.as_str());
                match select_res {
                    Ok(select) => realm.js_i32_create(select.count() as i32),
                    Err(_) => realm.js_i32_create(0),
                }
            })
        })
        .add_method("Symbol.iterator", |_rt, realm, id, _args| {
            //
            // this should be considered a hack, it only works in quicksj, we need Iterable support in utils::JsProxy

            // return an object with a next func, (clone NodeList and move to clusure)
            // next func should return an object with {done: false|true, value: null | nextVal}

            let obj = realm.js_object_create()?;

            let select_opt_ref = RefCell::new(with_select_element_list(&id, |select_base| {
                let select_res = select_base.node.select(select_base.selectors.as_str());
                match select_res {
                    Ok(select) => Some(select),
                    Err(_) => None,
                }
            }));

            let next_func = realm.js_function_create(
                "next",
                move |realm: &R, _this, _args| {
                    //
                    let ret_obj = realm.js_object_create()?;
                    let select_opt = &mut *select_opt_ref.borrow_mut();

                    let next = match select_opt {
                        None => None,
                        Some(select) => select.next(),
                    };

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
                                &register_node(realm, node.as_node().clone())?,
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
    realm.js_proxy_install(proxy, true)
}

pub(crate) fn init<B: JsRuntimeBuilder>(builder: B) -> B {
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

        let promise = block_on(rt.js_eval(None, Script::new("testhtml.js", code)))
            .ok()
            .expect("script failed");
        let rti = rt.js_get_runtime_facade_inner().upgrade().unwrap();
        if let JsValueFacade::JsPromise { cached_promise } = promise {
            let prom_res =
                block_on(cached_promise.js_get_promise_result(&*rti)).expect("promise timed out");

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
