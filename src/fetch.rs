use quickjs_runtime::features::fetch::request::FetchRequest;
use quickjs_runtime::features::fetch::response::FetchResponse;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Arc;
use ureq::{Agent, Response};

lazy_static! {
    static ref CLIENT: Arc<Agent> = Arc::new(Agent::new());
}

// todo refactor to trait so we can create different impls with rules and such

pub fn fetch_response_provider(fetch_request: &FetchRequest) -> Box<dyn FetchResponse + Send> {
    // throw a ureq client in a lazy_static
    //NOTE: we are not in the worker thread here but a random helper thread

    let agent: &Agent = &*CLIENT;
    // todo this provider should return a Result
    // todo METHOD in FetchRequest
    // TODO querystr in FetchRequest

    log::debug!(
        "fetch_response_provider::fetch: {}",
        fetch_request.get_url()
    );

    let mut req = agent.request("GET", fetch_request.get_url());

    let response: Response = req.call();

    let headers: HashMap<String, String> = HashMap::new();
    let mut bytes = vec![];
    let status = response.status();
    let mut reader = response.into_reader();
    reader
        .read_to_end(&mut bytes)
        .ok()
        .expect("could not read reader"); // todo remove this expect when we can return a Result
    Box::new(FetchResponseStruct {
        status,
        headers,
        bytes: Some(bytes),
    })
}

struct FetchResponseStruct {
    status: u16,
    headers: HashMap<String, String>,
    bytes: Option<Vec<u8>>,
}

impl FetchResponse for FetchResponseStruct {
    fn get_http_status(&self) -> u16 {
        self.status
    }

    fn get_header(&self, name: &str) -> Option<&str> {
        match self.headers.get(name) {
            None => None,
            Some(header) => Some(header.as_str()),
        }
    }

    fn read(&mut self) -> Option<Vec<u8>> {
        std::mem::replace(&mut self.bytes, None)
    }
}

#[cfg(test)]
mod tests {
    use crate::new_greco_rt_builder;
    use quickjs_runtime::esscript::EsScript;
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn test_fetch() {
        let rt = new_greco_rt_builder().build();
        let res = rt.eval_sync(EsScript::new(
            "test_fetch.es",
            "(fetch('https://httpbin.org/get').then((res) => {return(res.json()).then((json) => {return ('got stuff ' + JSON.stringify(json));});}));",
        ));

        match res {
            Ok(esvf) => {
                assert!(esvf.is_promise());
                let p_res = esvf.get_promise_result_sync();
                match p_res {
                    Ok(o) => {
                        // promise resolved
                        assert!(o.is_string());
                        let st = o.get_str();
                        log::trace!("res = {}", st);
                        assert!(st.starts_with("got stuff "));
                    }
                    Err(e) => {
                        // promise rejected
                        panic!("promise rejected to {:?}", e);
                    }
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}
