use crate::database::object_database::ObjectDatabaseTrait;
use crate::graphql::request_context::RequestContext;
use crate::graphql::schema;
use crate::graphql::schema::Schema;
use actix_web::body::MessageBody;
use actix_web::dev::{Server, ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::web::Data;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};
use log::info;
use std::sync::Arc;

pub struct Webserver {
    pub object_database: Arc<Box<dyn ObjectDatabaseTrait>>,
}

impl Webserver {
    pub async fn serve(self, port: i32) -> Result<Server, actix_web::Error> {
        info!("Starting GraphQL server on port {}...", &port);

        let server = HttpServer::new(move || { WebApp::build() });
        Ok(server.bind(format!("0.0.0.0:{}", port))?.run())
    }
}

struct WebApp {}

impl WebApp {
    fn build() -> App<impl ServiceFactory<ServiceRequest, Config=(), Response=ServiceResponse<impl MessageBody>, Error=actix_web::Error, InitError=()>> {
        App::new()
            .app_data(Data::new(schema::schema()))
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/graphql")
                    .route(web::post().to(WebApp::graphql_route))
                    .route(web::get().to(WebApp::graphql_route)),
            )
            .service(web::resource("/graphiql").route(web::get().to(WebApp::graphiql_route)))
            .service(web::resource("/playground").route(web::get().to(WebApp::playground_route)))
    }

    async fn graphql_route(req: actix_web::HttpRequest, payload: web::Payload, schema: Data<Schema>) -> Result<HttpResponse, actix_web::Error> {
        let context = RequestContext {
            auth_token: None,
        };
        graphql_handler(
            &schema,
            &context,
            req,
            payload,
        ).await
    }

    async fn playground_route() -> Result<HttpResponse, actix_web::Error> {
        playground_handler("/graphql", None).await
    }

    async fn graphiql_route() -> Result<HttpResponse, actix_web::Error> {
        graphiql_handler("/graphql", None).await
    }
}