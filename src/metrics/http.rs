use hyper::{
    header::CONTENT_TYPE,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use prometheus::{TextEncoder, Encoder};

use crate::metrics::{INSTANCE_ID_GAUGER, TIKV_CLIENT_RETRIES, REQUEST_COUNTER, REQUEST_CMD_COUNTER, CURRENT_CONNECTION_COUNTER};

async fn serve_req(_r: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let response = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap();

    Ok(response)
}

pub async fn prometheus_server(instance_id: i64) -> Result<(), hyper::Error> {
    INSTANCE_ID_GAUGER.set(instance_id);
    TIKV_CLIENT_RETRIES.get();
    REQUEST_COUNTER.get();
    CURRENT_CONNECTION_COUNTER.get();

    let addr = ([127, 0, 0, 1], 9898).into();
    println!("Listening on http://{}", addr);
    let serve_future = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, hyper::Error>(service_fn(serve_req))
    }));

    serve_future.await?;
    Ok(())
}