## tcp-request

[![](https://img.shields.io/crates/v/tcp-request.svg)](https://crates.io/crates/tcp-request)
[![](https://docs.rs/tcp-request/badge.svg)](https://docs.rs/tcp-request)
[![](https://img.shields.io/crates/l/tcp-request.svg)](./LICENSE)
[![](https://github.com/ltpp-universe/tcp-request/workflows/Rust/badge.svg)](https://github.com/ltpp-universe/tcp-request/actions?query=workflow:Rust)

[Official Documentation](https://docs.ltpp.vip/tcp-request/)

[Api Docs](https://docs.rs/tcp-request/latest/tcp_request/)

> A Rust library for sending raw TCP requests, with features for handling responses, managing redirects, and working with compressed data over TCP connections.

## Installation

To use this crate, you can run cmd:

```shell
cargo add tcp-request
```

## Use

#### Send Text

```rs
use tcp_request::*;
 let mut request_builder = RequestBuilder::new()
    .host("127.0.0.1")
    .port(80)
    .data("tcp send")
    .builder();
request_builder
    .send()
    .and_then(|response| {
        println!("ResponseTrait => {:?}", response.text());
        Ok(())
    })
    .unwrap_or_else(|e| output("Error => ", &format!("{:?}", e), Color::Red));
```

#### Send Binary

```rs

```

## Help

Ensure that CMake is installed on the system

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [ltpp-universe <root@ltpp.vip>](mailto:root@ltpp.vip).
