use utoipa::OpenApi;

use crate::schemas::auth::{LoginSchema, RegisterSchema};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::auth::register,
        crate::handlers::auth::login,
    ),
    components(
        schemas(
            LoginSchema,
            RegisterSchema,
        )
    )
)]
pub struct ApiDoc;
