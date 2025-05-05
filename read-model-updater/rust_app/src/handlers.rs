use std::{future::Future, pin::Pin};

use lambda_runtime::Error;

use crate::Response;

type HandlerResult = Pin<Box<dyn Future<Output = Result<Response, Error>> + Send>>;

pub mod event_handler;
pub mod reservations;
pub mod table_managements;
