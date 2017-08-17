/* 
 * Zalando Shop API
 *
 * The shop API empowers developers to build amazing new apps or websites using Zalando shop data and services.
 *
 * OpenAPI spec version: v1.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// Articles : A simple container object that carries paging information as well as a list of articles on the `content` property.

#[derive(Debug, Serialize, Deserialize)]
pub struct Articles {
  /// total elements in the response
  #[serde(rename = "totalElements")] total_elements: i32,
  /// total number of pages in the response
  #[serde(rename = "totalPages")] total_pages: i32,
  /// page number
  #[serde(rename = "page")] page: i32,
  /// total number of elements in a page
  #[serde(rename = "size")] size: i32,
  #[serde(rename = "content")] content: Vec<::models::Article>
}

impl Articles {
  /// A simple container object that carries paging information as well as a list of articles on the `content` property.
  pub fn new(total_elements: i32, total_pages: i32, page: i32, size: i32, content: Vec<::models::Article>) -> Articles {
    Articles {
      total_elements: total_elements,
      total_pages: total_pages,
      page: page,
      size: size,
      content: content
    }
  }

  pub fn set_total_elements(&mut self, total_elements: i32) {
    self.total_elements = total_elements;
  }

  pub fn with_total_elements(mut self, total_elements: i32) -> Articles {
    self.total_elements = total_elements;
    self
  }

  pub fn total_elements(&self) -> &i32 {
    &self.total_elements
  }

  pub fn set_total_pages(&mut self, total_pages: i32) {
    self.total_pages = total_pages;
  }

  pub fn with_total_pages(mut self, total_pages: i32) -> Articles {
    self.total_pages = total_pages;
    self
  }

  pub fn total_pages(&self) -> &i32 {
    &self.total_pages
  }

  pub fn set_page(&mut self, page: i32) {
    self.page = page;
  }

  pub fn with_page(mut self, page: i32) -> Articles {
    self.page = page;
    self
  }

  pub fn page(&self) -> &i32 {
    &self.page
  }

  pub fn set_size(&mut self, size: i32) {
    self.size = size;
  }

  pub fn with_size(mut self, size: i32) -> Articles {
    self.size = size;
    self
  }

  pub fn size(&self) -> &i32 {
    &self.size
  }

  pub fn set_content(&mut self, content: Vec<::models::Article>) {
    self.content = content;
  }

  pub fn with_content(mut self, content: Vec<::models::Article>) -> Articles {
    self.content = content;
    self
  }

  pub fn content(&self) -> &Vec<::models::Article> {
    &self.content
  }

}



