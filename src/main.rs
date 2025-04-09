use aws_config;
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;

#[tokio::main]
async fn main() {
    let sh_cfg = aws_config::defaults(BehaviorVersion::latest()) //
        .load()
        .await;

    let s3 = Client::new(&sh_cfg);

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
                            eprintln!("No prefix? {:?}", out)
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Fail: {}", e)
            }
        }
    }

    println!("Finish")
}
