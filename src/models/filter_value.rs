/* 
 * Zalando Shop API
 *
 * The shop API empowers developers to build amazing new apps or websites using Zalando shop data and services.
 *
 * OpenAPI spec version: v1.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// FilterValue : Zalando API FilterValue Schema

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilterValue {
  /// The key for a filterValue. Only unique within one filter
  #[serde(rename = "key")] key: String,
  /// translated display name
  #[serde(rename = "displayName")] display_name: String
}

impl FilterValue {
  /// Zalando API FilterValue Schema
  pub fn new(key: String, display_name: String) -> FilterValue {
    FilterValue {
      key: key,
      display_name: display_name
    }
  }

  pub fn set_key(&mut self, key: String) {
    self.key = key;
  }

  pub fn with_key(mut self, key: String) -> FilterValue {
    self.key = key;
    self
  }

  pub fn key(&self) -> &String {
    &self.key
  }

  pub fn set_display_name(&mut self, display_name: String) {
    self.display_name = display_name;
  }

  pub fn with_display_name(mut self, display_name: String) -> FilterValue {
    self.display_name = display_name;
    self
  }

  pub fn display_name(&self) -> &String {
    &self.display_name
  }

}



