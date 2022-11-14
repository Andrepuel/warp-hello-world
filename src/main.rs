use std::net::{Ipv6Addr, SocketAddr};

use serde_json::Value;
use warp::{hyper::HeaderMap, path::FullPath, Filter};

#[tokio::main]
async fn main() {
    env_logger::init();

    let all = warp::path::full()
        .and(warp::header::headers_cloned())
        .and(
            warp::body::json()
                .map(Some)
                .or(warp::any().map(|| None))
                .unify(),
        )
        .map(|path: FullPath, headers: HeaderMap, body: Option<Value>| {
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

            warp::reply::json(&(path, headers, body))
        });

    warp::serve(all)
        .run(SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 8080))
        .await;
}
