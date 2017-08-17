use hyper;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod articles_api;
pub use self::articles_api::{ ArticlesApi, ArticlesApiClient };
mod brands_api;
pub use self::brands_api::{ BrandsApi, BrandsApiClient };
mod categories_api;
pub use self::categories_api::{ CategoriesApi, CategoriesApiClient };
mod domains_api;
pub use self::domains_api::{ DomainsApi, DomainsApiClient };
mod facets_api;
pub use self::facets_api::{ FacetsApi, FacetsApiClient };
mod filters_api;
pub use self::filters_api::{ FiltersApi, FiltersApiClient };
mod recommendations_api;
pub use self::recommendations_api::{ RecommendationsApi, RecommendationsApiClient };

pub mod configuration;
pub mod client;
