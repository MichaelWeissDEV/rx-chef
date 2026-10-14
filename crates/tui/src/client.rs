use crate::events::Client;
use rxchef_client::{RemoteEngineClient, SupervisedEngineClient};
use std::{io, sync::Arc, time::Duration};
pub struct Session {
    pub client: Client,
    local: Arc<SupervisedEngineClient>,
    runtime: tokio::runtime::Runtime,
}
impl Session {
    pub fn new(url: Option<&str>, token: Option<&str>) -> io::Result<Self> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        let local = Arc::new(
            SupervisedEngineClient::new(
                rxchef_client::jobs::WorkerConfig::local(std::env::current_exe()?),
                runtime.handle().clone(),
            )
            .map_err(io::Error::other)?,
        );
        let client: Client = if let Some(url) = url {
            Arc::new(
                RemoteEngineClient::new(url, token, Duration::from_secs(10))
                    .map_err(io::Error::other)?,
            )
        } else {
            local.clone()
        };
        Ok(Self {
            client,
            local,
            runtime,
        })
    }
}
impl Drop for Session {
    fn drop(&mut self) {
        self.runtime.block_on(self.local.shutdown());
    }
}
