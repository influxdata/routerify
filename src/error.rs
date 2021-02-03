/// The error type used by the `Routerify` library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Couldn't create router RegexSet")]
    CreateRouterRegexSet(#[source] regex::Error),

    #[error("Could not create an exact match regex for the route path: {1}")]
    GenerateExactMatchRegex(#[source] regex::Error, String),

    #[error("Could not create an exact match regex for the route path: {1}")]
    GeneratePrefixMatchRegex(#[source] regex::Error, String),
}

/// The error type used by `routerify::Router` for request error handling
#[derive(thiserror::Error, Debug)]
pub enum RouterError<E: HandlerError + 'static> {
    #[error("Couldn't decode the request path as UTF8")]
    DecodeRequestPath(#[source] std::str::Utf8Error),

    #[error("No handlers added to handle non-existent routes. Tips: Please add an '.any' route at the bottom to handle any routes.")]
    HandleNonExistentRoute,

    #[error("A route was unable to handle the pre middleware request")]
    HandlePreMiddlewareRequest(#[source] E),

    #[error("A route was unable to handle the request for target: {1}")]
    HandleRequest(#[source] E, String),

    #[error("One of the post middlewares (without info) couldn't process the response")]
    HandlePostMiddlewareWithoutInfoRequest(#[source] E),

    #[error("One of the post middlewares (with info) couldn't process the response")]
    HandlePostMiddlewareWithInfoRequest(#[source] E),
}

/// Errors which originate in user provided handlers must implement this trait
/// A blanket implementation is provided that should apply in most cases
pub trait HandlerError: std::error::Error + Unpin + Send + Sync {}

impl<T> HandlerError for T where T: std::error::Error + Unpin + Send + Sync {}
