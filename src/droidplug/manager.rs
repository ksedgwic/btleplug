use super::adapter::Adapter;
use crate::{api, Result};
use async_trait::async_trait;
use log::debug;

#[derive(Clone, Debug)]
pub struct Manager;

impl Manager {
    pub async fn new() -> Result<Manager> {
        debug!("droidplug::Manager::new");
        Ok(Manager)
    }
}

#[async_trait]
impl api::Manager for Manager {
    type Adapter = Adapter;

    async fn adapters(&self) -> Result<Vec<Adapter>> {
        debug!("droidplug::Manager::adapters starting");
        let retval = vec![super::global_adapter().clone()];
        debug!("droidplug::Manager::adapters finished");
        Ok(retval)
    }
}
