use juniper::*;
use warp::{http::Response, Filter};

#[graphql_interface(for=ConcreteNode)]
pub trait Node: Sized {
    fn id(&self) -> ID;
}

pub struct ConcreteNode;

#[graphql_interface]
impl Node for ConcreteNode {
    fn id(&self) -> ID {
        ID::new("A")
    }
}

#[graphql_object(impl=NodeValue)]
impl ConcreteNode {
    fn id(&self) -> ID {
        Node::id(self)
    }

    pub fn status(&self) -> String {
        "OK".to_string()
    }
}

#[derive(Clone, Copy, Debug)]
struct Query;

#[graphql_object]
impl Query {
    async fn nodeInterface() -> Option<NodeValue> {
        Some(NodeValue::ConcreteNode(ConcreteNode {}))
    }

    async fn nodeConcrete() -> Option<ConcreteNode> {
        Some(ConcreteNode {})
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<()>, EmptySubscription<()>>;

fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<()>::new(),
        EmptySubscription::<()>::new(),
    )
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let log = warp::log("warp_server");
    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body("<html><div>visit <a href=\"/graphiql\">/graphiql</a></html>".to_string())
    });
    let state = warp::any().map(|| ());
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());
    warp::serve(
        warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(homepage)
            .or(warp::path("graphql").and(graphql_filter))
            .with(log),
    )
    .run(([127, 0, 0, 1], 8080))
    .await
}
