# Changelog

## 0.2.1 Unreleased
- **Fix `net::ip` IPv4-mapped IPv6 parsing**: the IP parser previously rejected any IPv6 address containing `.` (its `head_ip` classified by the first character and bailed on `.` once marked as IPv6). `::ffff:192.168.1.10`-style IPv4-mapped IPv6 addresses therefore failed to parse. The parser now collects `.` within IPv6 and defers validation to `std::net::IpAddr::from_str`, which natively supports the mapped form. Added parser tests covering IPv4 boundaries, IPv6 variants, IPv4-mapped IPv6, and invalid inputs.

## 0.1.0 - 2026-03-13
- Initial standalone release extracted from the `wp-motor` workspace.
- Includes atom, symbol, scope, comment, function-call, network, and utility parsers.
