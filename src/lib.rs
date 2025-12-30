use std::net::{Ipv4Addr, Ipv6Addr};

use ipnet::{Ipv4Net, Ipv6Net};

//https://github.com/JakubOnderka/treebitmap
mod v1;
pub use v1::V1;

//https://github.com/tiborschneider/prefix-trie
mod v2;
#[allow(unused_imports)]
pub use v2::V2;

pub trait IPNet {
    type R;

    fn network_and_prefix(&self) -> (Self::R, u32);
}

pub trait Map<K, V>
where
    K: IPNet,
{
    fn insert(&mut self, ipnet: K, value: V) -> Option<V>;
    fn get_lpm(&self, ipnet: K) -> Option<&V>;
}

impl IPNet for Ipv4Net {
    type R = Ipv4Addr;

    fn network_and_prefix(&self) -> (Self::R, u32) {
        (self.network(), self.prefix_len() as u32)
    }
}

impl IPNet for Ipv6Net {
    type R = Ipv6Addr;

    fn network_and_prefix(&self) -> (Self::R, u32) {
        (self.network(), self.prefix_len() as u32)
    }
}
