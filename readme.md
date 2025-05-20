<center>

## tcp-request

[![](https://img.shields.io/crates/v/tcp-request.svg)](https://crates.io/crates/tcp-request)
[![](https://img.shields.io/crates/d/tcp-request.svg)](https://img.shields.io/crates/d/tcp-request.svg)
[![](https://docs.rs/tcp-request/badge.svg)](https://docs.rs/tcp-request)
[![](https://github.com/eastspire/tcp-request/workflows/Rust/badge.svg)](https://github.com/eastspire/tcp-request/actions?query=workflow:Rust)
[![](https://img.shields.io/crates/l/tcp-request.svg)](./LICENSE)

</center>

[Official Documentation](https://docs.ltpp.vip/tcp-request/)

[Api Docs](https://docs.rs/tcp-request/latest/tcp_request/)

> A Rust library for sending raw TCP requests, with features for handling responses, managing redirects, and working with compressed data over TCP connections.

## Installation

To use this crate, you can run cmd:

```shell
cargo add tcp-request
```

## Use

#### Receive Text

```rs
use tcp_request::*;

let mut request_builder = RequestBuilder::new()
    .host("127.0.0.1")
    .port(80)
    .build();
request_builder
    .send("tcp send".as_bytes())
    .and_then(|response| {
        println!("ResponseTrait => {:?}", response.text());
        Ok(())
    })
    .unwrap_or_else(|e| println!("Error => {:?}", e));
```

#### Receive Binary

```rs
use tcp_request::*;

let mut request_builder = RequestBuilder::new()
    .host("127.0.0.1")
    .port(80)
    .build();
request_builder
    .send("tcp send".as_bytes())
    .and_then(|response| {
        println!("ResponseTrait => {:?}", response.binary());
        Ok(())
    })
    .unwrap_or_else(|e| println!("Error => {:?}", e));
```

## Help

Ensure that CMake is installed on the system

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Contact

For any inquiries, please reach out to the author at [root@ltpp.vip](mailto:root@ltpp.vip).
