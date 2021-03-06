/* 
 * Zalando Shop API
 *
 * The shop API empowers developers to build amazing new apps or websites using Zalando shop data and services.
 *
 * OpenAPI spec version: v1.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ArticleReviewsSummaryStarRatingDistribution : start rating distribution of the article

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticleReviewsSummaryStarRatingDistribution {
  #[serde(rename = "1")] var_1: i32,
  #[serde(rename = "2")] var_2: i32,
  #[serde(rename = "3")] var_3: i32,
  #[serde(rename = "4")] var_4: i32,
  #[serde(rename = "5")] var_5: i32
}

impl ArticleReviewsSummaryStarRatingDistribution {
  /// start rating distribution of the article
  pub fn new(var_1: i32, var_2: i32, var_3: i32, var_4: i32, var_5: i32) -> ArticleReviewsSummaryStarRatingDistribution {
    ArticleReviewsSummaryStarRatingDistribution {
      var_1: var_1,
      var_2: var_2,
      var_3: var_3,
      var_4: var_4,
      var_5: var_5
    }
  }

  pub fn set_var_1(&mut self, var_1: i32) {
    self.var_1 = var_1;
  }

  pub fn with_var_1(mut self, var_1: i32) -> ArticleReviewsSummaryStarRatingDistribution {
    self.var_1 = var_1;
    self
  }

  pub fn var_1(&self) -> &i32 {
    &self.var_1
  }

  pub fn set_var_2(&mut self, var_2: i32) {
    self.var_2 = var_2;
  }

  pub fn with_var_2(mut self, var_2: i32) -> ArticleReviewsSummaryStarRatingDistribution {
    self.var_2 = var_2;
    self
  }

  pub fn var_2(&self) -> &i32 {
    &self.var_2
  }

  pub fn set_var_3(&mut self, var_3: i32) {
    self.var_3 = var_3;
  }

  pub fn with_var_3(mut self, var_3: i32) -> ArticleReviewsSummaryStarRatingDistribution {
    self.var_3 = var_3;
    self
  }

  pub fn var_3(&self) -> &i32 {
    &self.var_3
  }

  pub fn set_var_4(&mut self, var_4: i32) {
    self.var_4 = var_4;
  }

  pub fn with_var_4(mut self, var_4: i32) -> ArticleReviewsSummaryStarRatingDistribution {
    self.var_4 = var_4;
    self
  }

  pub fn var_4(&self) -> &i32 {
    &self.var_4
  }

  pub fn set_var_5(&mut self, var_5: i32) {
    self.var_5 = var_5;
  }

  pub fn with_var_5(mut self, var_5: i32) -> ArticleReviewsSummaryStarRatingDistribution {
    self.var_5 = var_5;
    self
  }

  pub fn var_5(&self) -> &i32 {
    &self.var_5
  }

}



