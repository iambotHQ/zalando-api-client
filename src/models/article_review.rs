/* 
 * Zalando Shop API
 *
 * The shop API empowers developers to build amazing new apps or websites using Zalando shop data and services.
 *
 * OpenAPI spec version: v1.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

/// ArticleReview : Zalando API Article Review Schema

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleReview {
  /// unique article review id
  #[serde(rename = "reviewId")] review_id: String,
  #[serde(rename = "articleId")] article_id: String,
  #[serde(rename = "articleModelId")] article_model_id: String,
  /// Customer short name in the article review
  #[serde(rename = "customerNickname")] customer_nickname: Option<String>,
  /// customer city in the article review
  #[serde(rename = "customerCity")] customer_city: Option<String>,
  /// customer country in the article review
  #[serde(rename = "customerCountry")] customer_country: Option<String>,
  /// language in the article review
  #[serde(rename = "language")] language: String,
  /// title in the article review
  #[serde(rename = "title")] title: String,
  /// description of the article review
  #[serde(rename = "description")] description: String,
  /// article review created date and time
  #[serde(rename = "created")] created: String,
  /// customer rating of the article
  #[serde(rename = "rating")] rating: i32,
  /// customer recommend to the article
  #[serde(rename = "recommend")] recommend: Option<bool>,
  /// customer review helpful count of the article
  #[serde(rename = "helpfulCount")] helpful_count: i32,
  /// customer review unhelpful count of the article
  #[serde(rename = "unhelpfulCount")] unhelpful_count: i32,
  #[serde(rename = "articleSizeRatings")] article_size_ratings: Option<::models::ArticleReviewArticleSizeRatings>
}

impl ArticleReview {
  /// Zalando API Article Review Schema
  pub fn new(review_id: String, article_id: String, article_model_id: String, language: String, title: String, description: String, created: String, rating: i32, helpful_count: i32, unhelpful_count: i32) -> ArticleReview {
    ArticleReview {
      review_id: review_id,
      article_id: article_id,
      article_model_id: article_model_id,
      customer_nickname: None,
      customer_city: None,
      customer_country: None,
      language: language,
      title: title,
      description: description,
      created: created,
      rating: rating,
      recommend: None,
      helpful_count: helpful_count,
      unhelpful_count: unhelpful_count,
      article_size_ratings: None
    }
  }

  pub fn set_review_id(&mut self, review_id: String) {
    self.review_id = review_id;
  }

  pub fn with_review_id(mut self, review_id: String) -> ArticleReview {
    self.review_id = review_id;
    self
  }

  pub fn review_id(&self) -> &String {
    &self.review_id
  }

  pub fn set_article_id(&mut self, article_id: String) {
    self.article_id = article_id;
  }

  pub fn with_article_id(mut self, article_id: String) -> ArticleReview {
    self.article_id = article_id;
    self
  }

  pub fn article_id(&self) -> &String {
    &self.article_id
  }

  pub fn set_article_model_id(&mut self, article_model_id: String) {
    self.article_model_id = article_model_id;
  }

  pub fn with_article_model_id(mut self, article_model_id: String) -> ArticleReview {
    self.article_model_id = article_model_id;
    self
  }

  pub fn article_model_id(&self) -> &String {
    &self.article_model_id
  }

  pub fn set_customer_nickname(&mut self, customer_nickname: String) {
    self.customer_nickname = Some(customer_nickname);
  }

  pub fn with_customer_nickname(mut self, customer_nickname: String) -> ArticleReview {
    self.customer_nickname = Some(customer_nickname);
    self
  }

  pub fn customer_nickname(&self) -> &String {
    &self.customer_nickname
  }

  pub fn set_customer_city(&mut self, customer_city: String) {
    self.customer_city = Some(customer_city);
  }

  pub fn with_customer_city(mut self, customer_city: String) -> ArticleReview {
    self.customer_city = Some(customer_city);
    self
  }

  pub fn customer_city(&self) -> &String {
    &self.customer_city
  }

  pub fn set_customer_country(&mut self, customer_country: String) {
    self.customer_country = Some(customer_country);
  }

  pub fn with_customer_country(mut self, customer_country: String) -> ArticleReview {
    self.customer_country = Some(customer_country);
    self
  }

  pub fn customer_country(&self) -> &String {
    &self.customer_country
  }

  pub fn set_language(&mut self, language: String) {
    self.language = language;
  }

  pub fn with_language(mut self, language: String) -> ArticleReview {
    self.language = language;
    self
  }

  pub fn language(&self) -> &String {
    &self.language
  }

  pub fn set_title(&mut self, title: String) {
    self.title = title;
  }

  pub fn with_title(mut self, title: String) -> ArticleReview {
    self.title = title;
    self
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn set_description(&mut self, description: String) {
    self.description = description;
  }

  pub fn with_description(mut self, description: String) -> ArticleReview {
    self.description = description;
    self
  }

  pub fn description(&self) -> &String {
    &self.description
  }

  pub fn set_created(&mut self, created: String) {
    self.created = created;
  }

  pub fn with_created(mut self, created: String) -> ArticleReview {
    self.created = created;
    self
  }

  pub fn created(&self) -> &String {
    &self.created
  }

  pub fn set_rating(&mut self, rating: i32) {
    self.rating = rating;
  }

  pub fn with_rating(mut self, rating: i32) -> ArticleReview {
    self.rating = rating;
    self
  }

  pub fn rating(&self) -> &i32 {
    &self.rating
  }

  pub fn set_recommend(&mut self, recommend: bool) {
    self.recommend = Some(recommend);
  }

  pub fn with_recommend(mut self, recommend: bool) -> ArticleReview {
    self.recommend = Some(recommend);
    self
  }

  pub fn recommend(&self) -> &bool {
    &self.recommend
  }

  pub fn set_helpful_count(&mut self, helpful_count: i32) {
    self.helpful_count = helpful_count;
  }

  pub fn with_helpful_count(mut self, helpful_count: i32) -> ArticleReview {
    self.helpful_count = helpful_count;
    self
  }

  pub fn helpful_count(&self) -> &i32 {
    &self.helpful_count
  }

  pub fn set_unhelpful_count(&mut self, unhelpful_count: i32) {
    self.unhelpful_count = unhelpful_count;
  }

  pub fn with_unhelpful_count(mut self, unhelpful_count: i32) -> ArticleReview {
    self.unhelpful_count = unhelpful_count;
    self
  }

  pub fn unhelpful_count(&self) -> &i32 {
    &self.unhelpful_count
  }

  pub fn set_article_size_ratings(&mut self, article_size_ratings: ::models::ArticleReviewArticleSizeRatings) {
    self.article_size_ratings = Some(article_size_ratings);
  }

  pub fn with_article_size_ratings(mut self, article_size_ratings: ::models::ArticleReviewArticleSizeRatings) -> ArticleReview {
    self.article_size_ratings = Some(article_size_ratings);
    self
  }

  pub fn article_size_ratings(&self) -> &::models::ArticleReviewArticleSizeRatings {
    &self.article_size_ratings
  }

}


