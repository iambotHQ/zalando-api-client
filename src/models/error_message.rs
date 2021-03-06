/* 
 * Zalando Shop API
 *
 * The shop API empowers developers to build amazing new apps or websites using Zalando shop data and services.
 *
 * OpenAPI spec version: v1.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorMessage {
  #[serde(rename = "status")] status: i32,
  #[serde(rename = "message")] message: String,
  #[serde(rename = "errors")] errors: Option<Vec<::models::ErrorDetail>>
}

impl ErrorMessage {
  pub fn new(status: i32, message: String) -> ErrorMessage {
    ErrorMessage {
      status: status,
      message: message,
      errors: None
    }
  }

  pub fn set_status(&mut self, status: i32) {
    self.status = status;
  }

  pub fn with_status(mut self, status: i32) -> ErrorMessage {
    self.status = status;
    self
  }

  pub fn status(&self) -> &i32 {
    &self.status
  }

  pub fn set_message(&mut self, message: String) {
    self.message = message;
  }

  pub fn with_message(mut self, message: String) -> ErrorMessage {
    self.message = message;
    self
  }

  pub fn message(&self) -> &String {
    &self.message
  }

  pub fn set_errors(&mut self, errors: Vec<::models::ErrorDetail>) {
    self.errors = Some(errors);
  }

  pub fn with_errors(mut self, errors: Vec<::models::ErrorDetail>) -> ErrorMessage {
    self.errors = Some(errors);
    self
  }

  pub fn errors(&self) -> &Option<Vec<::models::ErrorDetail>> {
    &self.errors
  }

}



