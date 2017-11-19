use std::rc::Rc;

use futures::{future, Future, Stream};

use hyper;
use hyper::{Get, Post, Delete, Put, StatusCode, Body, Headers};
use hyper::header::ContentLength;
use hyper::server::{Request, Response, Service};

use serde_json::from_slice;

use database::Database;
use error::Error;
use types::*;

const GOODS: &'static str = "/api/goods";
const STAFF: &'static str = "/api/staff";
const RECEIPT: &'static str = "/api/receipt";
const CUSTOMER: &'static str = "/api/customer";
const PURCHASE: &'static str = "/api/purchase";

pub struct ApiService {
  db: Rc<Database>
}

impl ApiService {
  pub fn new(db: Rc<Database>) -> ApiService {
    ApiService { db }
  }

  fn get_body(body: Body, headers: Headers) -> Box<Future<Item=Vec<u8>, Error=Error>> {
    let vec;
    if let Some(len) = headers.get::<ContentLength>() {
      vec = Vec::with_capacity(**len as usize);
    } else {
      vec = vec![];
    }
    Box::new(
      body
        .fold(vec, |mut acc, chunk| {
          acc.extend_from_slice(&*chunk);
          Ok::<_, hyper::Error>(acc)
        })
        .map_err(|e| Error::Hyper(e))
    )
  }

  fn error_res(err: Error) -> Result<Response, hyper::Error> {
    error!("{:?}", err);
    Ok(Response::new()
      .with_status(StatusCode::InternalServerError)
      .with_body(err.json()))
  }
}

impl Service for ApiService {
  // hook up server's types
  type Request = Request;
  type Response = Response;
  type Error = hyper::Error;
  type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

  fn call(&self, req: Request) -> Self::Future {
    let db = Rc::clone(&self.db);
    let (method, uri, _, headers, body) = req.deconstruct();
    match method {
      Get => Box::new(
        future::done(match uri.path() {
          GOODS => self.db.join_supplied_goods(),
          CUSTOMER => self.db.get_all::<Customer>(),
          PURCHASE => self.db.join_purchase(),
          RECEIPT => self.db.join_receipt(),
          STAFF => self.db.get_all::<Staff>(),
          _ => Err(Error::NotFound)
        })
          .and_then(|s| Ok(Response::new().with_body(s)))
          .or_else(ApiService::error_res)
      ),
      Delete => Box::new(ApiService::get_body(body, headers)
        .and_then(move |chunks| {
          from_slice::<Id>(&chunks).map(|res| res.id).map_err(Error::from_serde)
        })
        .and_then(move |id| match uri.path() {
          GOODS => db.delete_by_id::<Goods>(id),
          CUSTOMER => db.delete_by_id::<Customer>(id),
          PURCHASE => db.delete_by_id::<Purchase>(id),
          RECEIPT => db.delete_by_id::<Receipt>(id),
          STAFF => db.delete_by_id::<Staff>(id),
          _ => Err(Error::NotFound)
        })
        .map(|_| Response::new().with_status(StatusCode::Ok))
        .or_else(ApiService::error_res)
      ),
      Post => Box::new(ApiService::get_body(body, headers)
        .and_then(move |chunks| match uri.path() {
          GOODS => from_slice::<Goods>(&chunks).map_err(Error::from_serde)
            .and_then(move |obj| db.create::<Goods>(obj)),
          CUSTOMER => from_slice::<Customer>(&chunks).map_err(Error::from_serde)
            .and_then(move |obj| db.create::<Customer>(obj)),
          PURCHASE => from_slice::<Purchase>(&chunks).map_err(Error::from_serde)
            .and_then(move |obj| db.create::<Purchase>(obj)),
          RECEIPT => from_slice::<Receipt>(&chunks).map_err(Error::from_serde)
            .and_then(move |obj| db.create::<Receipt>(obj)),
          STAFF => from_slice::<Staff>(&chunks).map_err(Error::from_serde)
            .and_then(move |obj| db.create::<Staff>(obj)),
          _ => Err(Error::NotFound)
        })
        .map(|_| Response::new().with_status(StatusCode::Ok))
        .or_else(ApiService::error_res)
      ),
      Put => Box::new(ApiService::get_body(body, headers)
        .and_then(move |chunks| match uri.path() {
          GOODS => from_slice::<Goods>(&chunks).map_err(Error::from_serde)
            .and_then(move |obj| db.update::<Goods>(obj)),
          CUSTOMER => from_slice::<Customer>(&chunks).map_err(Error::from_serde)
            .and_then(move |obj| db.update::<Customer>(obj)),
          PURCHASE => from_slice::<Purchase>(&chunks).map_err(Error::from_serde)
            .and_then(move |obj| db.update::<Purchase>(obj)),
          RECEIPT => from_slice::<Receipt>(&chunks).map_err(Error::from_serde)
            .and_then(move |obj| db.update::<Receipt>(obj)),
          STAFF => from_slice::<Staff>(&chunks).map_err(Error::from_serde)
            .and_then(move |obj| db.update::<Staff>(obj)),
          _ => Err(Error::NotFound)
        })
        .map(|_| Response::new().with_status(StatusCode::Ok))
        .or_else(ApiService::error_res)
      ),
      _ => Box::new(future::err(Error::NotFound).or_else(ApiService::error_res))
    }
  }
}
