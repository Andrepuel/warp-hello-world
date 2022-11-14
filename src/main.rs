use bytes::Bytes;
use serde_json::Value;
use std::{
    net::{Ipv6Addr, SocketAddr},
    str::FromStr,
};
use warp::{
    hyper::{HeaderMap, Method},
    path::FullPath,
    Filter,
};

#[tokio::main]
async fn main() {
    env_logger::init();

    let all = warp::filters::method::method()
        .and(warp::path::full())
        .and(warp::header::headers_cloned())
        .and(warp::query::query())
        .and(
            warp::body::bytes()
                .map(|bytes: Bytes| {
                    let body = std::str::from_utf8(&bytes)
                        .map_err(|_| ())
                        .and_then(|str| Value::from_str(str).map_err(|_| ()))
                        .unwrap_or_else(|_| Value::from(base64::encode(bytes)));

                    Some(body)
                })
                .or(warp::any().map(|| None))
                .unify(),
        )
        .map(
            |method: Method,
             path: FullPath,
             headers: HeaderMap,
             query: Value,
             body: Option<Value>| {
                log::debug!("Method: {method:?}");
                log::debug!("Query:");
                log::debug!("{:#}", query);
                log::debug!("Path: {path:?}");
                log::debug!("Headers:");
                for header in headers.iter() {
                    log::debug!("    {header:?}");
                }
                if let Some(body) = body.as_ref() {
                    log::debug!("Body:");
                    log::debug!("{:#}", body);
                }
                let path = path.as_str();
                let headers = headers
                    .into_iter()
                    .map(|x| format!("{x:?}"))
                    .collect::<Vec<_>>();

                warp::reply::json(&(method.to_string(), path, headers, query, body))
            },
        );

    warp::serve(all)
        .run(SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 8080))
        .await;
}
