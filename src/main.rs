#![feature(decl_macro, proc_macro_hygiene)]

use dotenv::dotenv;
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use rocket::{response::content, State};
use rust_ethereum_study::{
    context::Context, establish_connection, generate_web3_transport, get_account, parse_address,
    Gheedorah, QueryRoot,
};
use std::error::Error;
use web3::api::Web3;

type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;

#[rocket::get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&schema, &context)
}

#[rocket::post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute_sync(&schema, &context)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let web3 = generate_web3_transport()?;
    let network_id = web3.net().version().await?;
    let instance = Gheedorah::deployed(&web3).await?;

    let context = Context::new(instance);
    let schema = Schema::new(
        QueryRoot,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    );

    rocket::ignite()
        .manage(context)
        .manage(schema)
        .mount(
            "/",
            rocket::routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();

    Ok(())
}
