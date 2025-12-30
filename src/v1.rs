//https://github.com/JakubOnderka/treebitmap

#![allow(unused)]

use ip_network_table_deps_treebitmap::{address::Address, IpLookupTable};

use super::{IPNet, Map};

pub struct V1<A, V> {
    map: IpLookupTable<A, V>,
}

impl<K, V, A> Map<K, V> for V1<A, V>
where
    K: IPNet<R = A>,
    A: Address,
{
    fn insert(&mut self, ipnet: K, value: V) -> Option<V> {
        let (network, prefix) = ipnet.network_and_prefix();
        self.map.insert(network, prefix, value)
    }

    fn get_lpm(&self, ipnet: K) -> Option<&V> {
        let (network, _) = ipnet.network_and_prefix();
        if let Some((_, _, value)) = self.map.longest_match(network) {
            return Some(value);
        }

        None
    }
}

impl<A, V> V1<A, V>
where
    A: Address,
{
    pub fn new() -> Self {
        Self {
            map: IpLookupTable::<A, V>::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};

    use ipnet::{IpNet, Ipv4Net, Ipv6Net};

    use super::*;

    #[test]
    fn test_v1() {
        let mut tblv4 = V1::<Ipv4Addr, i32>::new();
        let mut tblv6 = V1::<Ipv6Addr, i32>::new();

        let addrs = vec![
            ("192.168.12.0/24", 24),
            ("192.168.12.230/32", 32),
            ("1.1.0.0/16", 16),
            ("fd00::/16", 16),
            ("642E:ABCE:5A25:B54D:49E3:9FD8::/64", 64),
        ];

        addrs.iter().for_each(|(addr, value)| {
            let addr = addr.parse().unwrap();

            if let IpNet::V4(v) = addr {
                tblv4.insert(v, value.to_owned());
            } else if let IpNet::V6(v) = addr {
                tblv6.insert(v, value.to_owned());
            }
        });

        assert_eq!(
            tblv4.get_lpm("192.168.12.230/32".parse::<Ipv4Net>().unwrap()),
            Some(&32)
        );
        assert_eq!(
            tblv4.get_lpm("192.168.12.100/32".parse::<Ipv4Net>().unwrap()),
            Some(&24)
        );
        assert_eq!(
            tblv4.get_lpm("1.1.1.1/32".parse::<Ipv4Net>().unwrap()),
            Some(&16)
        );
        assert_eq!(
            tblv4.get_lpm("1.2.1.1/32".parse::<Ipv4Net>().unwrap()),
            None
        );
        assert_eq!(
            tblv6.get_lpm("fd00:fd00::/128".parse::<Ipv6Net>().unwrap()),
            Some(&16)
        );
        assert_eq!(
            tblv6.get_lpm("fd00:fd01::/128".parse::<Ipv6Net>().unwrap()),
            Some(&16)
        );
        assert_eq!(
            tblv6.get_lpm("fd01:fd00::/128".parse::<Ipv6Net>().unwrap()),
            None
        );
        assert_eq!(
            tblv6.get_lpm(
                "642E:ABCE:5A25:B54D:49E3:9FD8:70A2:6A20/128"
                    .parse::<Ipv6Net>()
                    .unwrap()
            ),
            Some(&64)
        );
        assert_eq!(
            tblv6.get_lpm(
                "642E:ABCE:5A25:B54E:49E3:9FD8:70A2:6A20/128"
                    .parse::<Ipv6Net>()
                    .unwrap()
            ),
            None
        );
    }
}
