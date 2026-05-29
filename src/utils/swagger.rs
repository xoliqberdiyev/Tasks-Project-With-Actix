use utoipa::OpenApi;

use crate::{
    handlers::auth::{login, register},
    schemas::auth::{LoginSchema, RegisterSchema},
};

#[derive(OpenApi)]
#[openapi(
    paths(
        register,
        login,
    ),
    components(
        schemas(
            RegisterSchema,
            LoginSchema,
        )
    )
)]
pub struct ApiDoc;
