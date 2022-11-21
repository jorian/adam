use std::fmt::Debug;
use std::path::Path;

pub trait Chain {
    fn get_config_dir(&self) -> &Path;
    fn get_config_file(&self) -> &Path;
    fn get_name(&self) -> String;
    fn set_name(&mut self);
    fn testnet(&self) -> bool;
    fn currencyidhex(&self) -> String;
}

impl Debug for dyn Chain {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ({})", &self.currencyidhex(), self.get_name())
    }
}
