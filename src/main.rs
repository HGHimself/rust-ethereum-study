#![feature(decl_macro, proc_macro_hygiene)]

use dotenv::dotenv;
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use rust_ethereum_study::{
    context::Context, establish_connection, generate_web3_transport, get_account, parse_address,
    Gheedorah, MutationRoot, QueryRoot,
};
use std::error::Error;
use warp::{http::Response, Filter};
use web3::api::Web3;

type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let web3 = generate_web3_transport()?;
    let network_id = web3.net().version().await?;
    let instance = Gheedorah::deployed(&web3).await?;

    let schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription::<Context>::new());
    let context = Context::new(instance);

    let log = warp::log("warp_server");

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(format!(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
            ))
    });

    println!("Listening on 127.0.0.1:8080");

    let state = warp::any().map(move || context.clone());
    let graphql_filter = juniper_warp::make_graphql_filter(schema, state.boxed());

    warp::serve(
        warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(homepage)
            .or(warp::path("graphql").and(graphql_filter)),
    )
    .run(([127, 0, 0, 1], 8080))
    .await;

    Ok(())
}
