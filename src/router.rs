use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
pub struct Router {
    routes: radix_trie::Trie<String, HashSet<Method>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: Default::default(),
        }
    }

    pub fn route(mut self, path: &str, method: Method) -> Self {
        if !path.starts_with("/") {
            panic!("Route path must start with '/'");
        }

        match self.routes.get_mut(path) {
            Some(methods) => {
                methods.insert(method);
            }
            None => {
                self.routes
                    .insert(path.to_string(), HashSet::from([method]));
            }
        }

        self
    }
}

//TODO: Fix comparisons results. Find another solution to have only one callback for method
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Method {
    GET(fn()),
    HEAD(fn()),
    OPTIONS(fn()),
    TRACE(fn()),
    PUT(fn()),
    DELETE(fn()),
    POST(fn()),
    PATCH(fn()),
    CONNECT(fn()),
}
