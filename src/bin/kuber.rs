

use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing::{ event, info , Level };


use kube::{ Client, Api, api::ListParams };
use k8s_openapi::api::core::v1::Pod;

fn setup_log() {
    let appender = tracing_appender::rolling::never(".","kuber.log");

    let sub = tracing_subscriber::registry()
        .with(fmt::Layer::default().with_writer(appender).with_ansi(false))
        .with(fmt::Layer::default().compact().with_filter(LevelFilter::INFO));

    tracing::subscriber::set_global_default(sub).unwrap();
}

#[tokio::main]
async fn main() {
    setup_log();

    let client = Client::try_default().await.unwrap();
    
    let pods : Api<Pod> = Api::all(client);

    let lp = ListParams::default();

    let lst = pods.list(&lp).await.unwrap();

    for p in lst {
        //println!("POD: {}", p.metadata.name.unwrap());
        info!( "POD: {:?}", p.metadata.name.unwrap());
    }

    event!(Level::INFO, "working...");
}