use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Router {
    routes: radix_trie::Trie<String, HashMap<Method, fn()>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Default::default(),
        }
    }

    pub fn route(mut self, path: &str, method: Method, handler: fn()) -> Self {
        if !path.starts_with("/") {
            panic!("Route path must start with '/'");
        }

        match self.routes.get_mut(path) {
            Some(methods) => {
                methods.insert(method, handler);
            }
            None => {
                self.routes
                    .insert(path.to_string(), HashMap::from([(method, handler)]));
            }
        }

        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Method {
    GET,
    HEAD,
    OPTIONS,
    TRACE,
    PUT,
    DELETE,
    POST,
    PATCH,
    CONNECT,
}
