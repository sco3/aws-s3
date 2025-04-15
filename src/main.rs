use aws_config::ConfigLoader;
use aws_sdk_s3::Client;
use log::error;

#[tokio::main]
async fn main() {
    let s3 = Client::new(&ConfigLoader::default().load().await);

    let mut response = s3
        .list_objects_v2()
        .bucket("dz-bucket-1234")
        .prefix("agent_")
        .delimiter("/")
        .into_paginator()
        .send();

    while let Some(result) = response.next().await {
        match result {
            Ok(out) => {
                let prefixes = out.common_prefixes();
                for prefix in prefixes {
                    match prefix.prefix() {
                        Some(name) => {
                            println!("{}", name);
                        }
                        None => {
                            error!("No prefix? {:?}", prefix)
                        }
                    }
                }
            }
            Err(e) => {
                error!("Fail: {:?}", e)
            }
        }
    }
}
