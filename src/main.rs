use chain::Chain;
use pbaas::local_pbaas_chains;
use verus::VerusChain;

pub mod chain;
pub mod pbaas;
pub mod verus;

fn main() {
    let mut all_chains: Vec<Box<dyn Chain>> = vec![];

    let v_chain = Box::new(VerusChain::new(true));
    let local_chains = local_pbaas_chains(true);

    all_chains.push(v_chain);
    local_chains.into_iter().for_each(|mut c| {
        c.set_name();
        all_chains.push(Box::new(c));
    });

    dbg!(all_chains);
}
