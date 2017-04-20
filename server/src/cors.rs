#![allow(dead_code)]

// https://github.com/SergioBenitez/Rocket/issues/25#issuecomment-271065434

use std::collections::HashSet;
use rocket::response::{self, Response, Responder};
use rocket::http::{Status, Method};

pub struct CORS<R> {
    responder: R,
    allow_origin: &'static str,
    allow_credentials: bool,
    expose_headers: HashSet<&'static str>,
    max_age: Option<usize>,
    allow_methods: HashSet<Method>,
    allow_headers: HashSet<&'static str>,
    status: Option<Status>,
}

impl<'r, R: Responder<'r>> CORS<R> {
    /// Consumes responder and returns CORS with any origin
    pub fn any(responder: R) -> CORS<R> {
        CORS::origin(responder, "*")
    }

    /// Consumes the responder and origin and returns basic CORS
    pub fn origin(responder: R, origin: &'static str) -> CORS<R> {
        CORS {
            responder: responder,
            allow_origin: origin,
            allow_credentials: false,
            expose_headers: HashSet::new(),
            max_age: None,
            allow_methods: HashSet::new(),
            allow_headers: HashSet::new(),
            status: None,
        }
    }

    /// Consumes the CORS, set allow_credentials to
    /// new value and returns changed CORS
    pub fn credentials(mut self, value: bool) -> CORS<R> {
        self.allow_credentials = value;
        self
    }

    /// Consumes the CORS, set expose_headers to
    /// passed headers and returns changed CORS
    pub fn exposed_headers(mut self, headers: &[&'static str]) -> CORS<R> {
        self.expose_headers = headers.into_iter().cloned().collect();
        self
    }

    /// Consumes the CORS, set max_age to
    /// passed value and returns changed CORS
    pub fn max_age(mut self, value: Option<usize>) -> CORS<R> {
        self.max_age = value;
        self
    }

    /// Consumes the CORS, set allow_methods to
    /// passed methods and returns changed CORS
    pub fn methods(mut self, methods: &[Method]) -> CORS<R> {
        self.allow_methods = methods.into_iter().cloned().collect();
        self
    }

    /// Consumes the CORS, set allow_headers to
    /// passed headers and returns changed CORS
    pub fn headers(mut self, headers: &[&'static str]) -> CORS<R> {
        self.allow_headers = headers.into_iter().cloned().collect();
        self
    }

    /// Sets a status on the Response
    pub fn status(mut self, status: Status) -> CORS<R> {
        self.status = Some(status);
        self
    }
}

impl<'r, R: Responder<'r>> Responder<'r> for CORS<R> {
    fn respond(self) -> response::Result<'r> {
        let mut response = Response::build_from(self.responder.respond()?)
            .raw_header("Access-Control-Allow-Origin", self.allow_origin)
            .finalize();

        if self.allow_credentials {
            response.set_raw_header("Access-Control-Allow-Credentials", "true");
        } else {
            response.set_raw_header("Access-Control-Allow-Credentials", "false");
        }

        if !self.expose_headers.is_empty() {
            let headers: Vec<_> = self.expose_headers.into_iter().collect();
            let headers = headers.join(", ");

            response.set_raw_header("Access-Control-Expose-Headers", headers);
        }

        if !self.allow_methods.is_empty() {
            let methods: Vec<_> = self.allow_methods
                .into_iter()
                .map(|m| m.as_str())
                .collect();
            let methods = methods.join(", ");

            response.set_raw_header("Access-Control-Allow-Methods", methods);
        }

        if self.max_age.is_some() {
            let max_age = self.max_age.unwrap();
            response.set_raw_header("Access-Control-Max-Age", max_age.to_string());
        }

        if self.status.is_some() {
            response.set_status(self.status.unwrap());
        }

        Ok(response)
    }
}
