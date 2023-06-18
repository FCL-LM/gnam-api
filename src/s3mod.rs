use std::process::exit;

use aws_sdk_s3 as s3;
use log::{error, info};
use s3::{
    error::SdkError,
    operation::create_bucket::{CreateBucketError, CreateBucketOutput},
    types::{BucketLocationConstraint, CreateBucketConfiguration},
    Client,
};

use crate::utils::get_env;

pub async fn get_client() -> Client {
    let endpoint = get_env("S3_ENDPOINT");
    let temp_config = aws_config::load_from_env().await;

    let config = s3::config::Builder::from(&temp_config).endpoint_url(endpoint);
    let client = s3::Client::from_conf(config.build());
    return client;
}

pub async fn create_bucket(
    client: &Client,
    bucket_name: &str,
    region: &str,
) -> Result<CreateBucketOutput, SdkError<CreateBucketError>> {
    let constraint = BucketLocationConstraint::from(region);
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(constraint)
        .build();
    client
        .create_bucket()
        .create_bucket_configuration(cfg)
        .bucket(bucket_name)
        .send()
        .await
}

pub async fn bucket_exists(client: &Client, bucket_name: &str) -> bool {
    let resp = client.list_buckets().send().await;

    if resp.is_err() {
        error!(
            "Error while checking if the bucket {} exists: {}",
            bucket_name,
            resp.err().unwrap()
        );

        exit(1);
    }

    let resp = resp.unwrap();
    let buckets = resp.buckets().unwrap_or_default();

    for bucket in buckets {
        if bucket.name().unwrap().eq(bucket_name) {
            return true;
        }
    }

    return false;
}

pub async fn create_source_bucket() {
    info!("Creating sources bucket...");

    let client = get_client().await;
    let bucket_name = "sources";

    if bucket_exists(&client, bucket_name).await {
        info!("The sources bucket already exists");
        return;
    }

    let s = create_bucket(&client, bucket_name, "").await;

    if s.is_err() {
        error!("Creating the document_sources bucket: {}", s.err().unwrap());
        exit(1);
    }

    info!("Bucket sources created.");
}
