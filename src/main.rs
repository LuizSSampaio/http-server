use crate::router::Router;

pub mod router;

#[tokio::main]
async fn main() {
    let mut router = Router::new();
    router.route("/", router::Method::GET, || {});

    router.serve("127.0.0.1:4221").await;
}
