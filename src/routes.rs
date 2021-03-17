use actix_web::{get, guard, post, web, HttpRequest, HttpResponse, Result, Scope};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Schema,
};
use async_graphql_actix_web::{Request, Response, WSSubscription};

use crate::RGSSchema;

/// The path prefix to use for the routes in this module
const SCOPE: &str = "/graphql";

#[post("")]
// The index route that handles the GraphQL requests
async fn index(schema: web::Data<RGSSchema>, gql_req: Request) -> Response {
    schema.execute(gql_req.into_inner()).await.into()
}

#[get("")]
// The index route that shows the GraphQL playground
async fn playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new(SCOPE).subscription_endpoint(SCOPE),
        )))
}

// The subscription handler
async fn index_ws(
    schema: web::Data<RGSSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    WSSubscription::start(Schema::clone(&*schema), &req, payload)
}

// The service builder
pub fn index_routes() -> Scope {
    web::scope(SCOPE)
        .service(index)
        .service(
            web::resource("")
                .guard(guard::Get())
                .guard(guard::Header("upgrade", "websocket"))
                .to(index_ws),
        )
        .service(playground)
}
