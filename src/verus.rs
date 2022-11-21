use std::path::{Path, PathBuf};

use crate::chain::Chain;

#[derive(Debug)]
pub struct VerusChain {
    testnet: bool,
    name: String,
    currencyidhex: String,
    config_dir: PathBuf,
}

impl Chain for VerusChain {
    fn get_config_dir(&self) -> &Path {
        &self.config_dir.as_path()
    }

    fn get_config_file(&self) -> &Path {
        &self.config_dir.as_path()
    }

    fn testnet(&self) -> bool {
        self.testnet
    }

    fn currencyidhex(&self) -> String {
        self.currencyidhex.clone()
    }

    fn set_name(&mut self) {}

    fn get_name(&self) -> String {
        self.name.clone()
    }

    // fn fetch_name(&self) -> Option<String> {}
}

impl VerusChain {
    pub fn new(testnet: bool) -> Self {
        let name = match testnet {
            true => "vrsctest".to_string(),
            false => "VRSC".to_string(),
        };

        let currencyidhex = match testnet {
            true => "2d4eb6919e9fdb2934ff2481325e6335a29eefa6".to_string(),
            false => unimplemented!(),
        };

        VerusChain {
            testnet,
            name,
            currencyidhex,
            config_dir: PathBuf::new(),
        }
    }
}
