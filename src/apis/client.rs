use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  articles_api: Box<::apis::ArticlesApi>,
  brands_api: Box<::apis::BrandsApi>,
  categories_api: Box<::apis::CategoriesApi>,
  domains_api: Box<::apis::DomainsApi>,
  facets_api: Box<::apis::FacetsApi>,
  filters_api: Box<::apis::FiltersApi>,
  recommendations_api: Box<::apis::RecommendationsApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      articles_api: Box::new(::apis::ArticlesApiClient::new(rc.clone())),
      brands_api: Box::new(::apis::BrandsApiClient::new(rc.clone())),
      categories_api: Box::new(::apis::CategoriesApiClient::new(rc.clone())),
      domains_api: Box::new(::apis::DomainsApiClient::new(rc.clone())),
      facets_api: Box::new(::apis::FacetsApiClient::new(rc.clone())),
      filters_api: Box::new(::apis::FiltersApiClient::new(rc.clone())),
      recommendations_api: Box::new(::apis::RecommendationsApiClient::new(rc.clone())),
    }
  }

  pub fn articles_api(&self) -> &::apis::ArticlesApi{
    self.articles_api.as_ref()
  }

  pub fn brands_api(&self) -> &::apis::BrandsApi{
    self.brands_api.as_ref()
  }

  pub fn categories_api(&self) -> &::apis::CategoriesApi{
    self.categories_api.as_ref()
  }

  pub fn domains_api(&self) -> &::apis::DomainsApi{
    self.domains_api.as_ref()
  }

  pub fn facets_api(&self) -> &::apis::FacetsApi{
    self.facets_api.as_ref()
  }

  pub fn filters_api(&self) -> &::apis::FiltersApi{
    self.filters_api.as_ref()
  }

  pub fn recommendations_api(&self) -> &::apis::RecommendationsApi{
    self.recommendations_api.as_ref()
  }


}
