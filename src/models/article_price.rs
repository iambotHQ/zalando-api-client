/* 
 * Zalando Shop API
 *
 * The shop API empowers developers to build amazing new apps or websites using Zalando shop data and services.
 *
 * OpenAPI spec version: v1.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ArticlePrice : Zalando API Article Price Schema

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticlePrice {
  /// 
  #[serde(rename = "currency")] currency: String,
  /// 
  #[serde(rename = "value")] value: f32,
  /// 
  #[serde(rename = "formatted")] formatted: String
}

impl ArticlePrice {
  /// Zalando API Article Price Schema
  pub fn new(currency: String, value: f32, formatted: String) -> ArticlePrice {
    ArticlePrice {
      currency: currency,
      value: value,
      formatted: formatted
    }
  }

  pub fn set_currency(&mut self, currency: String) {
    self.currency = currency;
  }

  pub fn with_currency(mut self, currency: String) -> ArticlePrice {
    self.currency = currency;
    self
  }

  pub fn currency(&self) -> &String {
    &self.currency
  }

  pub fn set_value(&mut self, value: f32) {
    self.value = value;
  }

  pub fn with_value(mut self, value: f32) -> ArticlePrice {
    self.value = value;
    self
  }

  pub fn value(&self) -> &f32 {
    &self.value
  }

  pub fn set_formatted(&mut self, formatted: String) {
    self.formatted = formatted;
  }

  pub fn with_formatted(mut self, formatted: String) -> ArticlePrice {
    self.formatted = formatted;
    self
  }

  pub fn formatted(&self) -> &String {
    &self.formatted
  }

}



