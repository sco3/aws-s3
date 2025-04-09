use aws_config::ConfigLoader;
use aws_sdk_s3::Client;

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
        if let Ok(out) = result {
            for prefix in out.common_prefixes() {
                if let Some(name) = prefix.prefix() {
                    println!("{}", name);
                }
            }
        }
    }
}
