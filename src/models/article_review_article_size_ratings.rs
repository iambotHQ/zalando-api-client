/* 
 * Zalando Shop API
 *
 * The shop API empowers developers to build amazing new apps or websites using Zalando shop data and services.
 *
 * OpenAPI spec version: v1.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ArticleReviewArticleSizeRatings : 

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleReviewArticleSizeRatings {
  #[serde(rename = "OVERALL")] OVERALL: Option<i32>,
  #[serde(rename = "CHEST")] CHEST: Option<i32>,
  #[serde(rename = "SLEEVES")] SLEEVES: Option<i32>,
  #[serde(rename = "SHOULDERS")] SHOULDERS: Option<i32>,
  #[serde(rename = "LENGTH")] LENGTH: Option<i32>,
  #[serde(rename = "LEG_FIT")] LEG_FIT: Option<i32>,
  #[serde(rename = "SHOE_WIDTH")] SHOE_WIDTH: Option<i32>,
  #[serde(rename = "BOOTLEG_WIDTH")] BOOTLEG_WIDTH: Option<i32>,
  #[serde(rename = "HIPS_OR_REAR")] HIPS_OR_REAR: Option<i32>,
  #[serde(rename = "CUP_SIZE")] CUP_SIZE: Option<i32>,
  #[serde(rename = "CHEST_GIRTH")] CHEST_GIRTH: Option<i32>,
  #[serde(rename = "COLLAR_SIZE")] COLLAR_SIZE: Option<i32>
}

impl ArticleReviewArticleSizeRatings {
  /// 
  pub fn new() -> ArticleReviewArticleSizeRatings {
    ArticleReviewArticleSizeRatings {
      OVERALL: None,
      CHEST: None,
      SLEEVES: None,
      SHOULDERS: None,
      LENGTH: None,
      LEG_FIT: None,
      SHOE_WIDTH: None,
      BOOTLEG_WIDTH: None,
      HIPS_OR_REAR: None,
      CUP_SIZE: None,
      CHEST_GIRTH: None,
      COLLAR_SIZE: None
    }
  }

  pub fn set_OVERALL(&mut self, OVERALL: i32) {
    self.OVERALL = Some(OVERALL);
  }

  pub fn with_OVERALL(mut self, OVERALL: i32) -> ArticleReviewArticleSizeRatings {
    self.OVERALL = Some(OVERALL);
    self
  }

  pub fn OVERALL(&self) -> &Option<i32> {
    &self.OVERALL
  }

  pub fn set_CHEST(&mut self, CHEST: i32) {
    self.CHEST = Some(CHEST);
  }

  pub fn with_CHEST(mut self, CHEST: i32) -> ArticleReviewArticleSizeRatings {
    self.CHEST = Some(CHEST);
    self
  }

  pub fn CHEST(&self) -> &Option<i32> {
    &self.CHEST
  }

  pub fn set_SLEEVES(&mut self, SLEEVES: i32) {
    self.SLEEVES = Some(SLEEVES);
  }

  pub fn with_SLEEVES(mut self, SLEEVES: i32) -> ArticleReviewArticleSizeRatings {
    self.SLEEVES = Some(SLEEVES);
    self
  }

  pub fn SLEEVES(&self) -> &Option<i32> {
    &self.SLEEVES
  }

  pub fn set_SHOULDERS(&mut self, SHOULDERS: i32) {
    self.SHOULDERS = Some(SHOULDERS);
  }

  pub fn with_SHOULDERS(mut self, SHOULDERS: i32) -> ArticleReviewArticleSizeRatings {
    self.SHOULDERS = Some(SHOULDERS);
    self
  }

  pub fn SHOULDERS(&self) -> &Option<i32> {
    &self.SHOULDERS
  }

  pub fn set_LENGTH(&mut self, LENGTH: i32) {
    self.LENGTH = Some(LENGTH);
  }

  pub fn with_LENGTH(mut self, LENGTH: i32) -> ArticleReviewArticleSizeRatings {
    self.LENGTH = Some(LENGTH);
    self
  }

  pub fn LENGTH(&self) -> &Option<i32> {
    &self.LENGTH
  }

  pub fn set_LEG_FIT(&mut self, LEG_FIT: i32) {
    self.LEG_FIT = Some(LEG_FIT);
  }

  pub fn with_LEG_FIT(mut self, LEG_FIT: i32) -> ArticleReviewArticleSizeRatings {
    self.LEG_FIT = Some(LEG_FIT);
    self
  }

  pub fn LEG_FIT(&self) -> &Option<i32> {
    &self.LEG_FIT
  }

  pub fn set_SHOE_WIDTH(&mut self, SHOE_WIDTH: i32) {
    self.SHOE_WIDTH = Some(SHOE_WIDTH);
  }

  pub fn with_SHOE_WIDTH(mut self, SHOE_WIDTH: i32) -> ArticleReviewArticleSizeRatings {
    self.SHOE_WIDTH = Some(SHOE_WIDTH);
    self
  }

  pub fn SHOE_WIDTH(&self) -> &Option<i32> {
    &self.SHOE_WIDTH
  }

  pub fn set_BOOTLEG_WIDTH(&mut self, BOOTLEG_WIDTH: i32) {
    self.BOOTLEG_WIDTH = Some(BOOTLEG_WIDTH);
  }

  pub fn with_BOOTLEG_WIDTH(mut self, BOOTLEG_WIDTH: i32) -> ArticleReviewArticleSizeRatings {
    self.BOOTLEG_WIDTH = Some(BOOTLEG_WIDTH);
    self
  }

  pub fn BOOTLEG_WIDTH(&self) -> &Option<i32> {
    &self.BOOTLEG_WIDTH
  }

  pub fn set_HIPS_OR_REAR(&mut self, HIPS_OR_REAR: i32) {
    self.HIPS_OR_REAR = Some(HIPS_OR_REAR);
  }

  pub fn with_HIPS_OR_REAR(mut self, HIPS_OR_REAR: i32) -> ArticleReviewArticleSizeRatings {
    self.HIPS_OR_REAR = Some(HIPS_OR_REAR);
    self
  }

  pub fn HIPS_OR_REAR(&self) -> &Option<i32> {
    &self.HIPS_OR_REAR
  }

  pub fn set_CUP_SIZE(&mut self, CUP_SIZE: i32) {
    self.CUP_SIZE = Some(CUP_SIZE);
  }

  pub fn with_CUP_SIZE(mut self, CUP_SIZE: i32) -> ArticleReviewArticleSizeRatings {
    self.CUP_SIZE = Some(CUP_SIZE);
    self
  }

  pub fn CUP_SIZE(&self) -> &Option<i32> {
    &self.CUP_SIZE
  }

  pub fn set_CHEST_GIRTH(&mut self, CHEST_GIRTH: i32) {
    self.CHEST_GIRTH = Some(CHEST_GIRTH);
  }

  pub fn with_CHEST_GIRTH(mut self, CHEST_GIRTH: i32) -> ArticleReviewArticleSizeRatings {
    self.CHEST_GIRTH = Some(CHEST_GIRTH);
    self
  }

  pub fn CHEST_GIRTH(&self) -> &Option<i32> {
    &self.CHEST_GIRTH
  }

  pub fn set_COLLAR_SIZE(&mut self, COLLAR_SIZE: i32) {
    self.COLLAR_SIZE = Some(COLLAR_SIZE);
  }

  pub fn with_COLLAR_SIZE(mut self, COLLAR_SIZE: i32) -> ArticleReviewArticleSizeRatings {
    self.COLLAR_SIZE = Some(COLLAR_SIZE);
    self
  }

  pub fn COLLAR_SIZE(&self) -> &Option<i32> {
    &self.COLLAR_SIZE
  }

}



