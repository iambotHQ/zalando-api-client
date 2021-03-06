/* 
 * Zalando Shop API
 *
 * The shop API empowers developers to build amazing new apps or websites using Zalando shop data and services.
 *
 * OpenAPI spec version: v1.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use super::{Error, configuration};

pub struct ArticlesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> ArticlesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ArticlesApiClient<C> {
        ArticlesApiClient {
            configuration: configuration,
        }
    }
}

pub trait ArticlesApi {
    fn ArticleReviewsGet(&self, article_id: Option<Vec<String>>, article_model_id: Option<Vec<String>>, min_star_rating: Option<&str>, max_star_rating: Option<&str>, sort: Option<&str>, page: Option<&str>, page_size: Option<&str>, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::ArticleReviews, Error = Error>>;
    fn ArticleReviewsReviewIdGet(&self, review_id: &str, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::ArticleReview, Error = Error>>;
    fn ArticleReviewsSummariesArticleModelIdGet(&self, article_model_id: &str, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::ArticleReviewsSummary, Error = Error>>;
    fn ArticleReviewsSummariesGet(&self, article_model_id: Vec<String>, page: Option<&str>, page_size: Option<&str>, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::ArticleReviewsSummaries, Error = Error>>;
    fn ArticlesArticleIdGet(&self, article_id: &str, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::Article, Error = Error>>;
    fn ArticlesArticleIdMediaGet(&self, article_id: &str, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::ArticleMedia, Error = Error>>;
    fn ArticlesArticleIdReviewsGet(&self, article_id: &str, min_star_rating: Option<&str>, max_star_rating: Option<&str>, sort: Option<&str>, page: Option<&str>, page_size: Option<&str>, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::ArticleReviews, Error = Error>>;
    fn ArticlesArticleIdReviewsSummaryGet(&self, article_id: &str, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::ArticleReviewsSummary, Error = Error>>;
    fn ArticlesArticleIdUnitsGet(&self, article_id: &str, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::ArticleUnits, Error = Error>>;
    fn ArticlesArticleIdUnitsUnitIdGet(&self, article_id: &str, unit_id: &str, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::ArticleUnit, Error = Error>>;
    fn ArticlesGet(&self, article_id: Option<Vec<String>>, article_model_id: Option<Vec<String>>, article_unit_id: Option<Vec<String>>, activation_date: Option<Vec<String>>, age_group: Option<Vec<String>>, assortment_area: Option<Vec<String>>, brand: Option<Vec<String>>, brandfamily: Option<Vec<String>>, category: Option<Vec<String>>, color: Option<Vec<String>>, den: Option<Vec<String>>, filling: Option<Vec<String>>, full_text: Option<&str>, gender: Option<Vec<String>>, heel_form: Option<Vec<String>>, heel_height: Option<Vec<String>>, length: Option<&str>, occasion: Option<Vec<String>>, pattern: Option<Vec<String>>, price: Option<&str>, sale: Option<Vec<String>>, season: Option<Vec<String>>, shaft_height: Option<Vec<String>>, shaft_width: Option<Vec<String>>, shirt_collar: Option<Vec<String>>, shoe_fastener: Option<Vec<String>>, shoe_toecap: Option<Vec<String>>, shop_area: Option<Vec<String>>, size: Option<&str>, sports: Option<Vec<String>>, technology: Option<Vec<String>>, trouser_rise: Option<Vec<String>>, upper_material: Option<Vec<String>>, volume: Option<Vec<String>>, page: Option<&str>, page_size: Option<&str>, sort: Option<&str>, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::Articles, Error = Error>>;
}


impl<C: hyper::client::Connect>ArticlesApi for ArticlesApiClient<C> {
   fn ArticleReviewsGet(&self, article_id: Option<Vec<String>>, article_model_id: Option<Vec<String>>, min_star_rating: Option<&str>, max_star_rating: Option<&str>, sort: Option<&str>, page: Option<&str>, page_size: Option<&str>, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::ArticleReviews, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let mut query = ::url::form_urlencoded::Serializer::new(String::new());
        match article_id{
           Some(value)=>{query.append_pair("articleId", &value.join(",").to_string());},
           None=>{},
        }
        match article_model_id{
           Some(value)=>{query.append_pair("articleModelId", &value.join(",").to_string());},
           None=>{},
        }
        match min_star_rating{
           Some(value)=>{query.append_pair("minStarRating", &value.to_string());},
           None=>{},
        }
        match max_star_rating{
           Some(value)=>{query.append_pair("maxStarRating", &value.to_string());},
           None=>{},
        }
        match sort{
           Some(value)=>{query.append_pair("sort", &value.to_string());},
           None=>{},
        }
        match page{
           Some(value)=>{query.append_pair("page", &value.to_string());},
           None=>{},
        }
        match page_size{
           Some(value)=>{query.append_pair("pageSize", &value.to_string());},
           None=>{},
        }
        match fields{
           Some(value)=>{query.append_pair("fields", &value.join(",").to_string());},
           None=>{},
        }
        let finished_query=query.finish();
        let uri_str = format!("{}/article-reviews?{}", configuration.base_path, finished_query);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());

        {
            let mut headers = req.headers_mut();
            match accept_language{
               Some(value)=>{headers.set_raw("Accept-Language", value);},
               None=>{},
            }
        }


        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::ArticleReviews, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

   fn ArticleReviewsReviewIdGet(&self, review_id: &str, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::ArticleReview, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let mut query = ::url::form_urlencoded::Serializer::new(String::new());
        match fields{
           Some(value)=>{query.append_pair("fields", &value.join(",").to_string());},
           None=>{},
        }
        let finished_query=query.finish();
        let uri_str = format!("{}/article-reviews/{reviewId}?{}", configuration.base_path, finished_query, reviewId=review_id);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());

        {
            let mut headers = req.headers_mut();
            match accept_language{
               Some(value)=>{headers.set_raw("Accept-Language", value);},
               None=>{},
            }
        }


        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::ArticleReview, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

   fn ArticleReviewsSummariesArticleModelIdGet(&self, article_model_id: &str, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::ArticleReviewsSummary, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let mut query = ::url::form_urlencoded::Serializer::new(String::new());
        match fields{
           Some(value)=>{query.append_pair("fields", &value.join(",").to_string());},
           None=>{},
        }
        let finished_query=query.finish();
        let uri_str = format!("{}/article-reviews-summaries/{articleModelId}?{}", configuration.base_path, finished_query, articleModelId=article_model_id);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());

        {
            let mut headers = req.headers_mut();
            match accept_language{
               Some(value)=>{headers.set_raw("Accept-Language", value);},
               None=>{},
            }
        }


        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::ArticleReviewsSummary, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

   fn ArticleReviewsSummariesGet(&self, article_model_id: Vec<String>, page: Option<&str>, page_size: Option<&str>, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::ArticleReviewsSummaries, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let mut query = ::url::form_urlencoded::Serializer::new(String::new());
        query.append_pair("articleModelId", &article_model_id.join(",").to_string());
        match page{
           Some(value)=>{query.append_pair("page", &value.to_string());},
           None=>{},
        }
        match page_size{
           Some(value)=>{query.append_pair("pageSize", &value.to_string());},
           None=>{},
        }
        match fields{
           Some(value)=>{query.append_pair("fields", &value.join(",").to_string());},
           None=>{},
        }
        let finished_query=query.finish();
        let uri_str = format!("{}/article-reviews-summaries?{}", configuration.base_path, finished_query);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());

        {
            let mut headers = req.headers_mut();
            match accept_language{
               Some(value)=>{headers.set_raw("Accept-Language", value);},
               None=>{},
            }
        }


        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::ArticleReviewsSummaries, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

   fn ArticlesArticleIdGet(&self, article_id: &str, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::Article, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let mut query = ::url::form_urlencoded::Serializer::new(String::new());
        match fields{
           Some(value)=>{query.append_pair("fields", &value.join(",").to_string());},
           None=>{},
        }
        let finished_query=query.finish();
        let uri_str = format!("{}/articles/{articleId}?{}", configuration.base_path, finished_query, articleId=article_id);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());

        {
            let mut headers = req.headers_mut();
            match accept_language{
               Some(value)=>{headers.set_raw("Accept-Language", value);},
               None=>{},
            }
        }


        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::Article, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

   fn ArticlesArticleIdMediaGet(&self, article_id: &str, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::ArticleMedia, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let mut query = ::url::form_urlencoded::Serializer::new(String::new());
        match fields{
           Some(value)=>{query.append_pair("fields", &value.join(",").to_string());},
           None=>{},
        }
        let finished_query=query.finish();
        let uri_str = format!("{}/articles/{articleId}/media?{}", configuration.base_path, finished_query, articleId=article_id);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());

        {
            let mut headers = req.headers_mut();
            match accept_language{
               Some(value)=>{headers.set_raw("Accept-Language", value);},
               None=>{},
            }
        }


        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::ArticleMedia, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

   fn ArticlesArticleIdReviewsGet(&self, article_id: &str, min_star_rating: Option<&str>, max_star_rating: Option<&str>, sort: Option<&str>, page: Option<&str>, page_size: Option<&str>, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::ArticleReviews, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let mut query = ::url::form_urlencoded::Serializer::new(String::new());
        match min_star_rating{
           Some(value)=>{query.append_pair("minStarRating", &value.to_string());},
           None=>{},
        }
        match max_star_rating{
           Some(value)=>{query.append_pair("maxStarRating", &value.to_string());},
           None=>{},
        }
        match sort{
           Some(value)=>{query.append_pair("sort", &value.to_string());},
           None=>{},
        }
        match page{
           Some(value)=>{query.append_pair("page", &value.to_string());},
           None=>{},
        }
        match page_size{
           Some(value)=>{query.append_pair("pageSize", &value.to_string());},
           None=>{},
        }
        match fields{
           Some(value)=>{query.append_pair("fields", &value.join(",").to_string());},
           None=>{},
        }
        let finished_query=query.finish();
        let uri_str = format!("{}/articles/{articleId}/reviews?{}", configuration.base_path, finished_query, articleId=article_id);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());

        {
            let mut headers = req.headers_mut();
            match accept_language{
               Some(value)=>{headers.set_raw("Accept-Language", value);},
               None=>{},
            }
        }


        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::ArticleReviews, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

   fn ArticlesArticleIdReviewsSummaryGet(&self, article_id: &str, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::ArticleReviewsSummary, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let mut query = ::url::form_urlencoded::Serializer::new(String::new());
        match fields{
           Some(value)=>{query.append_pair("fields", &value.join(",").to_string());},
           None=>{},
        }
        let finished_query=query.finish();
        let uri_str = format!("{}/articles/{articleId}/reviews-summary?{}", configuration.base_path, finished_query, articleId=article_id);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());

        {
            let mut headers = req.headers_mut();
            match accept_language{
               Some(value)=>{headers.set_raw("Accept-Language", value);},
               None=>{},
            }
        }


        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::ArticleReviewsSummary, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

   fn ArticlesArticleIdUnitsGet(&self, article_id: &str, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::ArticleUnits, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let mut query = ::url::form_urlencoded::Serializer::new(String::new());
        match fields{
           Some(value)=>{query.append_pair("fields", &value.join(",").to_string());},
           None=>{},
        }
        let finished_query=query.finish();
        let uri_str = format!("{}/articles/{articleId}/units?{}", configuration.base_path, finished_query, articleId=article_id);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());

        {
            let mut headers = req.headers_mut();
            match accept_language{
               Some(value)=>{headers.set_raw("Accept-Language", value);},
               None=>{},
            }
        }


        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::ArticleUnits, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

   fn ArticlesArticleIdUnitsUnitIdGet(&self, article_id: &str, unit_id: &str, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::ArticleUnit, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let mut query = ::url::form_urlencoded::Serializer::new(String::new());
        match fields{
           Some(value)=>{query.append_pair("fields", &value.join(",").to_string());},
           None=>{},
        }
        let finished_query=query.finish();
        let uri_str = format!("{}/articles/{articleId}/units/{unitId}?{}", configuration.base_path, finished_query, articleId=article_id, unitId=unit_id);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());

        {
            let mut headers = req.headers_mut();
            match accept_language{
               Some(value)=>{headers.set_raw("Accept-Language", value);},
               None=>{},
            }
        }


        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::ArticleUnit, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

   fn ArticlesGet(&self, article_id: Option<Vec<String>>, article_model_id: Option<Vec<String>>, article_unit_id: Option<Vec<String>>, activation_date: Option<Vec<String>>, age_group: Option<Vec<String>>, assortment_area: Option<Vec<String>>, brand: Option<Vec<String>>, brandfamily: Option<Vec<String>>, category: Option<Vec<String>>, color: Option<Vec<String>>, den: Option<Vec<String>>, filling: Option<Vec<String>>, full_text: Option<&str>, gender: Option<Vec<String>>, heel_form: Option<Vec<String>>, heel_height: Option<Vec<String>>, length: Option<&str>, occasion: Option<Vec<String>>, pattern: Option<Vec<String>>, price: Option<&str>, sale: Option<Vec<String>>, season: Option<Vec<String>>, shaft_height: Option<Vec<String>>, shaft_width: Option<Vec<String>>, shirt_collar: Option<Vec<String>>, shoe_fastener: Option<Vec<String>>, shoe_toecap: Option<Vec<String>>, shop_area: Option<Vec<String>>, size: Option<&str>, sports: Option<Vec<String>>, technology: Option<Vec<String>>, trouser_rise: Option<Vec<String>>, upper_material: Option<Vec<String>>, volume: Option<Vec<String>>, page: Option<&str>, page_size: Option<&str>, sort: Option<&str>, accept_language: Option<&str>, fields: Option<Vec<String>>) -> Box<Future<Item = ::models::Articles, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let mut query = ::url::form_urlencoded::Serializer::new(String::new());
        match article_id{
           Some(value)=>{query.append_pair("articleId", &value.join(",").to_string());},
           None=>{},
        }
        match article_model_id{
           Some(value)=>{query.append_pair("articleModelId", &value.join(",").to_string());},
           None=>{},
        }
        match article_unit_id{
           Some(value)=>{query.append_pair("articleUnitId", &value.join(",").to_string());},
           None=>{},
        }
        match activation_date{
           Some(value)=>{query.append_pair("activationDate", &value.join(",").to_string());},
           None=>{},
        }
        match age_group{
           Some(value)=>{query.append_pair("ageGroup", &value.join(",").to_string());},
           None=>{},
        }
        match assortment_area{
           Some(value)=>{query.append_pair("assortmentArea", &value.join(",").to_string());},
           None=>{},
        }
        match brand{
           Some(value)=>{query.append_pair("brand", &value.join(",").to_string());},
           None=>{},
        }
        match brandfamily{
           Some(value)=>{query.append_pair("brandfamily", &value.join(",").to_string());},
           None=>{},
        }
        match category{
           Some(value)=>{query.append_pair("category", &value.join(",").to_string());},
           None=>{},
        }
        match color{
           Some(value)=>{query.append_pair("color", &value.join(",").to_string());},
           None=>{},
        }
        match den{
           Some(value)=>{query.append_pair("den", &value.join(",").to_string());},
           None=>{},
        }
        match filling{
           Some(value)=>{query.append_pair("filling", &value.join(",").to_string());},
           None=>{},
        }
        match full_text{
           Some(value)=>{query.append_pair("fullText", &value.to_string());},
           None=>{},
        }
        match gender{
           Some(value)=>{query.append_pair("gender", &value.join(",").to_string());},
           None=>{},
        }
        match heel_form{
           Some(value)=>{query.append_pair("heelForm", &value.join(",").to_string());},
           None=>{},
        }
        match heel_height{
           Some(value)=>{query.append_pair("heelHeight", &value.join(",").to_string());},
           None=>{},
        }
        match length{
           Some(value)=>{query.append_pair("length", &value.to_string());},
           None=>{},
        }
        match occasion{
           Some(value)=>{query.append_pair("occasion", &value.join(",").to_string());},
           None=>{},
        }
        match pattern{
           Some(value)=>{query.append_pair("pattern", &value.join(",").to_string());},
           None=>{},
        }
        match price{
           Some(value)=>{query.append_pair("price", &value.to_string());},
           None=>{},
        }
        match sale{
           Some(value)=>{query.append_pair("sale", &value.join(",").to_string());},
           None=>{},
        }
        match season{
           Some(value)=>{query.append_pair("season", &value.join(",").to_string());},
           None=>{},
        }
        match shaft_height{
           Some(value)=>{query.append_pair("shaftHeight", &value.join(",").to_string());},
           None=>{},
        }
        match shaft_width{
           Some(value)=>{query.append_pair("shaftWidth", &value.join(",").to_string());},
           None=>{},
        }
        match shirt_collar{
           Some(value)=>{query.append_pair("shirtCollar", &value.join(",").to_string());},
           None=>{},
        }
        match shoe_fastener{
           Some(value)=>{query.append_pair("shoeFastener", &value.join(",").to_string());},
           None=>{},
        }
        match shoe_toecap{
           Some(value)=>{query.append_pair("shoeToecap", &value.join(",").to_string());},
           None=>{},
        }
        match shop_area{
           Some(value)=>{query.append_pair("shopArea", &value.join(",").to_string());},
           None=>{},
        }
        match size{
           Some(value)=>{query.append_pair("size", &value.to_string());},
           None=>{},
        }
        match sports{
           Some(value)=>{query.append_pair("sports", &value.join(",").to_string());},
           None=>{},
        }
        match technology{
           Some(value)=>{query.append_pair("technology", &value.join(",").to_string());},
           None=>{},
        }
        match trouser_rise{
           Some(value)=>{query.append_pair("trouserRise", &value.join(",").to_string());},
           None=>{},
        }
        match upper_material{
           Some(value)=>{query.append_pair("upperMaterial", &value.join(",").to_string());},
           None=>{},
        }
        match volume{
           Some(value)=>{query.append_pair("volume", &value.join(",").to_string());},
           None=>{},
        }
        match page{
           Some(value)=>{query.append_pair("page", &value.to_string());},
           None=>{},
        }
        match page_size{
           Some(value)=>{query.append_pair("pageSize", &value.to_string());},
           None=>{},
        }
        match sort{
           Some(value)=>{query.append_pair("sort", &value.to_string());},
           None=>{},
        }
        match fields{
           Some(value)=>{query.append_pair("fields", &value.join(",").to_string());},
           None=>{},
        }
        let finished_query=query.finish();
        let uri_str = format!("{}/articles?{}", configuration.base_path, finished_query);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());

        {
            let mut headers = req.headers_mut();
            match accept_language{
               Some(value)=>{headers.set_raw("Accept-Language", value);},
               None=>{},
            }
        }


        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::Articles, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}
