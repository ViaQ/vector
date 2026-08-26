#![allow(clippy::doc_overindented_list_items)]
include!(concat!(env!("OUT_DIR"), "/google.rpc.rs"));

impl warp::reject::Reject for Status {}
