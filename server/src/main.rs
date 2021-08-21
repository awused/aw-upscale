#![windows_subsystem = "windows"]

use std::net::ToSocketAddrs;
use std::sync::Arc;
use std::time::Duration;

use api::aw_upscale_server::{AwUpscale, AwUpscaleServer};
use api::upscale_request::TargetSize;
use api::{Resolution, UpscaleRequest, UpscaleResponse};
use aw_upscale::Upscaler;
use futures::future::try_join_all;
use once_cell::sync::Lazy;
use structopt::StructOpt;
use tokio::sync::Mutex;
use tokio::time::{self, Interval};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod api {
    tonic::include_proto!("upscale");
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "aw-upscale-server",
    about = "Server to use aw-upscale remotely."
)]
pub struct Opt {
    #[structopt(short, long)]
    /// Address to listen on. Example: locahost:9091
    addr: String,

    #[structopt(short, long)]
    /// Upscaler to use. Leave unset to use the default waifu2x.
    upscaler: Option<String>,
}

pub static OPTIONS: Lazy<Opt> = Lazy::new(Opt::from_args);


pub struct UpscaleServer {
    interval: Arc<Mutex<Interval>>,
}


#[tonic::async_trait]
impl AwUpscale for UpscaleServer {
    async fn upscale(
        &self,
        request: Request<UpscaleRequest>,
    ) -> Result<Response<UpscaleResponse>, Status> {
        let mut u = Upscaler::new(OPTIONS.upscaler.clone());

        let req = request.get_ref();
        if let Some(TargetSize::Scale(scale)) = &req.target_size {
            u.set_scale(*scale as u8);
        }

        if let Some(TargetSize::Resolutions(resolutions)) = &req.target_size {
            if let Some(target) = &resolutions.target {
                u.set_target_width(target.width);
                u.set_target_height(target.height);
            }
            if let Some(min) = &resolutions.minimum {
                u.set_min_width(min.width);
                u.set_min_height(min.height);
            }
        }

        u.set_denoise(req.denoise);

        let input = tempfile::Builder::new()
            .prefix("aw-upscale")
            .suffix(&(".".to_string() + &req.original_ext))
            .tempfile()?;

        let output = tempfile::Builder::new()
            .prefix("aw-upscale")
            .suffix(".png")
            .tempfile()?;

        let output = output.into_temp_path();

        tokio::fs::write(input.path(), &req.original_file).await?;
        let input = input.into_temp_path();

        let outpath = (*output).to_path_buf();
        self.interval.lock().await.tick().await;

        let (width, height) = tokio::task::spawn_blocking(move || {
            u.run(input, &outpath)
                .map_err(|e| Status::internal(e.to_string()))
        })
        .await
        .map_err(|e| Status::internal(e.to_string()))??;

        let upscaled = tokio::fs::read(output).await?;

        println!("Upscaled {}x{}", width, height);

        Ok(Response::new(UpscaleResponse {
            res: Some(Resolution { width, height }),
            upscaled,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addrs = OPTIONS.addr.to_socket_addrs().expect("Invalid address");
    // Use an interval to let tasks trickle in, improving latency (at least for the first image in
    // the batch) and throughput. Without the client making guarantees about order, though, this
    // can adversely affect user-visible latency to a minor degree.
    let mut interval = time::interval(Duration::from_millis(500));
    interval.set_missed_tick_behavior(time::MissedTickBehavior::Delay);
    let interval = Arc::new(Mutex::new(interval));


    let servers: Vec<_> = addrs
        .map(|addr| {
            let server = UpscaleServer {
                interval: interval.clone(),
            };
            println!("Listening on {}", addr);

            let server = AwUpscaleServer::new(server);

            Server::builder().add_service(server).serve(addr)
        })
        .collect();

    try_join_all(servers).await?;
    Ok(())
}
