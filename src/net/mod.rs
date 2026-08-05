use std::net::{IpAddr, Ipv4Addr};

use winnow::{
    ModalResult as WResult, Parser,
    ascii::{Caseless, multispace0},
    combinator::{fail, peek, repeat},
    error::ContextError,
    token::any,
};

use crate::symbol::ctx_desc;
use crate::utils::peek_one;

#[derive(PartialEq)]
enum AddrKind {
    Ipv4,
    Ipv6,
}

fn head_ip<'a>(last: &mut Option<AddrKind>) -> impl Parser<&'a str, char, ContextError> + '_ {
    move |input: &mut &'a str| {
        let initial = (peek(any)).parse_next(input)?;
        match initial {
            '0'..='9' => any.parse_next(input),
            'A'..='F' | 'a'..='f' => {
                *last = Some(AddrKind::Ipv6);
                any.parse_next(input)
            }
            '.' => {
                if *last == Some(AddrKind::Ipv6) {
                    // IPv4-mapped IPv6（如 ::ffff:192.168.1.10）：允许 '.'，保持 Ipv6，
                    // 最终交给 IpAddr::from_str 校验（std 支持 mapped 形式）。
                    any.parse_next(input)
                } else {
                    *last = Some(AddrKind::Ipv4);
                    any.parse_next(input)
                }
            }
            ':' => {
                if *last == Some(AddrKind::Ipv4) {
                    fail.parse_next(input)
                } else {
                    *last = Some(AddrKind::Ipv6);
                    any.parse_next(input)
                }
            }
            _ => fail.parse_next(input),
        }
    }
}

pub fn ip_v4(input: &mut &str) -> WResult<IpAddr> {
    let mut last_kind = None;
    // Build the candidate ip string, then parse using std::net::IpAddr::from_str.
    // Avoids relying on `try_map` error conversion semantics across winnow versions.
    let ip_str = match repeat(1.., head_ip(&mut last_kind))
        .fold(String::new, |mut acc, c| {
            acc.push(c);
            acc
        })
        .parse_next(input)
    {
        Ok(s) => s,
        Err(_e) => return fail.context(ctx_desc("<ipv4>")).parse_next(input),
    };
    match ip_str.parse::<IpAddr>() {
        Ok(ip) => Ok(ip),
        Err(_e) => fail.context(ctx_desc("<ipv4>")).parse_next(input),
    }
}
pub fn ip(input: &mut &str) -> WResult<IpAddr> {
    multispace0.parse_next(input)?;

    let str = peek_one.parse_next(input);
    if let Ok(s) = str {
        let addr = if s == "l" {
            Caseless("localhost")
                .map(|_| IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
                .context(ctx_desc("<localhost>"))
                .parse_next(input)?
        } else {
            ip_v4.context(ctx_desc("<ipv4>")).parse_next(input)?
        };
        Ok(addr)
    } else {
        fail.context(ctx_desc("ip error")).parse_next(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn parse_ip(input: &str) -> IpAddr {
        let mut s = input;
        ip(&mut s).expect("ip parse")
    }

    #[test]
    fn ip_parses_ipv4() {
        assert_eq!(parse_ip("192.168.1.10"), IpAddr::from_str("192.168.1.10").unwrap());
    }

    #[test]
    fn ip_parses_ipv4_boundaries() {
        assert_eq!(parse_ip("0.0.0.0"), IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
        assert_eq!(
            parse_ip("255.255.255.255"),
            IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255))
        );
        assert_eq!(parse_ip("10.0.0.1"), IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)));
    }

    #[test]
    fn ip_parses_ipv6() {
        assert_eq!(
            parse_ip("2001:db8:85a3::8a2e:370:7334"),
            IpAddr::from_str("2001:db8:85a3::8a2e:370:7334").unwrap()
        );
        assert_eq!(parse_ip("fd00:1::2"), IpAddr::from_str("fd00:1::2").unwrap());
    }

    #[test]
    fn ip_parses_ipv6_variants() {
        // loopback
        assert_eq!(parse_ip("::1"), IpAddr::from_str("::1").unwrap());
        // 压缩 IPv6
        assert_eq!(parse_ip("2001:db8::1"), IpAddr::from_str("2001:db8::1").unwrap());
        // ULA
        assert_eq!(parse_ip("fd00:1::2"), IpAddr::from_str("fd00:1::2").unwrap());
        // link-local
        assert_eq!(parse_ip("fe80::1"), IpAddr::from_str("fe80::1").unwrap());
        // 完整（非压缩）IPv6
        assert_eq!(
            parse_ip("2001:db8:0:0:0:0:0:1"),
            IpAddr::from_str("2001:db8:0:0:0:0:0:1").unwrap()
        );
    }

    #[test]
    fn ip_parses_ipv4_mapped_ipv6() {
        // IPv4-mapped IPv6：修复前 head_ip 对 IPv6 中的 '.' 直接 fail
        assert_eq!(
            parse_ip("::ffff:192.168.1.10"),
            IpAddr::from_str("::ffff:192.168.1.10").unwrap()
        );
        assert_eq!(parse_ip("::ffff:8.8.8.8"), IpAddr::from_str("::ffff:8.8.8.8").unwrap());
    }

    #[test]
    fn ip_parses_ipv4_mapped_ipv6_variants() {
        // 边界 mapped
        assert_eq!(parse_ip("::ffff:0.0.0.0"), IpAddr::from_str("::ffff:0.0.0.0").unwrap());
        assert_eq!(
            parse_ip("::ffff:255.255.255.255"),
            IpAddr::from_str("::ffff:255.255.255.255").unwrap()
        );
        assert_eq!(parse_ip("::ffff:8.8.8.8"), IpAddr::from_str("::ffff:8.8.8.8").unwrap());
    }

    #[test]
    fn ip_parses_localhost() {
        assert_eq!(parse_ip("localhost"), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    }

    #[test]
    fn ip_rejects_invalid_input() {
        // 非 IP 字符串
        let mut input = "not-an-ip";
        assert!(ip(&mut input).is_err());
        // IPv4 越界
        let mut input = "256.1.1.1";
        assert!(ip(&mut input).is_err());
        // 空串
        let mut input = "";
        assert!(ip(&mut input).is_err());
        // IPv6 首字符非法（'g' 不在 hex 范围）
        let mut input = "gggg::1";
        assert!(ip(&mut input).is_err());
    }
}
