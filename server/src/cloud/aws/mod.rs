use crate::cloud::aws::ec2::EC2Impl;
use crate::cloud::*;
use aws_sdk_ec2::types::KeyPairInfo;
use aws_sdk_ec2::Error;
use std::sync::LazyLock;

mod ec2;

async fn get_client() -> anyhow::Result<aws_sdk_ec2::Client> {
    let config = aws_config::load_from_env().await;
    Ok(aws_sdk_ec2::Client::new(&config))
}

async fn get_ec2() -> anyhow::Result<EC2Impl> {
    let client = get_client().await?;
    Ok(EC2Impl::new(client))
}

// 
//
// pub async fn get_instances() -> anyhow::Result<Vec<Instance>> {
//     let client = get_client().await?;
//     let resp = client.describe_instances().send().await?;
//
//     println!("{:?}", resp);
//
//     let mut instances = Vec::new();
//
//     Ok(instances)
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[tokio::test]
    // async fn test_get_regions() {
    //     let regions = get_regions().await.unwrap();
    //     println!("{:?}", regions);
    //     assert!(!regions.is_empty());
    // }

    #[tokio::test]
    async fn test_get_instances() {
        //let instances = get_instances().await.unwrap();
        //println!("{:?}", instances);
        //assert!(!instances.is_empty());
    }
}
