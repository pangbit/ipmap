//https://github.com/tiborschneider/prefix-trie

#![allow(unused)]

use prefix_trie::{Prefix, PrefixMap};

use super::{IPNet, Map};

pub struct V2<K, V> {
    map: PrefixMap<K, V>,
}

impl<K, V> Map<K, V> for V2<K, V>
where
    K: Prefix + IPNet,
{
    fn insert(&mut self, ipnet: K, value: V) -> Option<V> {
        self.map.insert(ipnet, value)
    }

    fn get_lpm(&self, ipnet: K) -> Option<&V> {
        if let Some((_, value)) = self.map.get_lpm(&ipnet) {
            return Some(value);
        }

        None
    }
}

impl<K, V> V2<K, V>
where
    K: Prefix + IPNet,
{
    pub fn new() -> Self {
        Self {
            map: PrefixMap::<K, V>::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use ipnet::{IpNet, Ipv4Net, Ipv6Net};

    use super::*;

    #[test]
    fn test_v2() {
        let mut tblv4 = V2::<Ipv4Net, i32>::new();
        let mut tblv6 = V2::<Ipv6Net, i32>::new();

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
