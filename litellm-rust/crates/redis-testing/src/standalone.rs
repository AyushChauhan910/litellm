use std::net::TcpListener;

use testcontainers_modules::{
    redis::{REDIS_PORT, Redis},
    testcontainers::core::IntoContainerPort,
    testcontainers::{ContainerAsync, ImageExt, runners::AsyncRunner},
};

use crate::Error;

const REDIS_TAG: &str =
    "7.4@sha256:44960b3100c48159a66d28c18b0588c193492e25f7551d60e51dd464fc3eca5b";

pub struct RedisServer {
    container: ContainerAsync<Redis>,
    url: String,
}

impl RedisServer {
    pub async fn start() -> Result<Self, Error> {
        let host_port = free_port()?;
        let container = Redis::default()
            .with_tag(REDIS_TAG)
            .with_mapped_port(host_port, REDIS_PORT.tcp())
            .start()
            .await?;
        let url = format!("redis://{}:{host_port}", container.get_host().await?);
        Ok(Self { container, url })
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub async fn restart(&self) -> Result<(), Error> {
        self.container.stop().await?;
        self.container.start().await?;
        Ok(())
    }

    pub async fn freeze(&self) -> Result<(), Error> {
        Ok(self.container.pause().await?)
    }

    pub async fn thaw(&self) -> Result<(), Error> {
        Ok(self.container.unpause().await?)
    }
}

fn free_port() -> Result<u16, Error> {
    TcpListener::bind("127.0.0.1:0")
        .and_then(|listener| listener.local_addr())
        .map(|address| address.port())
        .map_err(Error::Port)
}
