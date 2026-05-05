use aws_sdk_s3::config::{BehaviorVersion, Builder, Credentials};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::{config::Region, Client};
use std::env;
pub fn create_r2_client() -> Client {
    let credentials = Credentials::new(
        env::var("S3_ACCESS_KEY").unwrap(),
        env::var("S3_SECRET_ACCESS_KEY").unwrap(),
        None,
        None,
        "r2",
    );

    let config = Builder::new()
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new("auto"))
        .endpoint_url(env::var("S3_ENDPOINT").unwrap())
        .credentials_provider(credentials)
        .build();

    Client::from_conf(config)
}

pub async fn upload_to_r2(
    client: &Client,
    bucket: &str,
    key: &str,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let body = ByteStream::from_path(file_path).await?;

    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .content_type("image/gif")
        .send()
        .await?;

    Ok(())
}
