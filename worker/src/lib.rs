#![allow(clippy::new_without_default)]
#![allow(clippy::or_fun_call)]

#[doc(hidden)]
use std::result::Result as StdResult;
use std::{
    pin::Pin,
    task::{self, Poll},
};

#[doc(hidden)]
pub use async_trait;
use fragile::Fragile;
use futures_util::Future;
#[doc(hidden)]
pub use js_sys;
pub use url::Url;
#[doc(hidden)]
pub use wasm_bindgen;
#[doc(hidden)]
pub use wasm_bindgen_futures;
pub use worker_kv as kv;

pub use cf::Cf;
pub use worker_macros::{durable_object, event};
#[doc(hidden)]
pub use worker_sys;
pub use worker_sys::{console_debug, console_error, console_log, console_warn};

pub use crate::abort::*;
pub use crate::cache::{Cache, CacheDeletionOutcome};
pub use crate::context::Context;
pub use crate::cors::Cors;
pub use crate::date::{Date, DateInit};
pub use crate::delay::Delay;
pub use crate::durable::*;
pub use crate::env::Env;
pub use crate::error::Error;
pub use crate::formdata::*;
pub use crate::global::Fetch;
pub use crate::headers::Headers;
pub use crate::method::Method;
pub use crate::request::Request;
pub use crate::request_init::*;
pub use crate::response::{Response, ResponseBody};
pub use crate::router::{RouteContext, RouteParams, Router};
pub use crate::schedule::*;
pub use crate::streams::*;
pub use crate::websocket::*;

mod abort;
mod cache;
mod cf;
mod context;
mod cors;
mod date;
mod delay;
pub mod durable;
mod env;
mod error;
mod formdata;
mod global;
mod headers;
pub mod http_types;
mod method;
mod request;
mod request_init;
mod response;
mod router;
mod schedule;
mod streams;
mod websocket;

pub type Result<T> = StdResult<T, error::Error>;

pub use ::http;
pub use ::http_body;

pub type HttpRequest = ::http::Request<ByteStream>;
pub type HttpResponse<B> = ::http::Response<B>;

/// Turns a `!Send` future into a `Send` one. Based on the wgpu-rs crate.
pub(crate) struct MakeSendFuture<F>(Fragile<F>);

impl<F> MakeSendFuture<F> {
    pub fn new(future: F) -> Self {
        Self(Fragile::new(future))
    }
}

impl<F: Future> Future for MakeSendFuture<F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll<Self::Output> {
        unsafe { self.map_unchecked_mut(|s| s.0.get_mut()) }.poll(cx)
    }
}

unsafe impl<F> Send for MakeSendFuture<F> {}
