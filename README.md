# jep106

[![crates.io](http://meritbadge.herokuapp.com/jep106)](https://crates.io/crates/jep106)

[![documentation](https://docs.rs/jep106/badge.svg)](https://docs.rs/jep106)

This crate provides a means to retrieve the JEDEC manufacturer string for a corresponding JEP106 ID Code.

All the codes can be found on the page of the JEDEC organization but are presented in as a PDF. This crate contains the parsed data
from the PDF and exposes an interface to poll the JEDEC manufacturer string of a JEP106 ID code.

The PDF is parsed by the [jep106-parse](https://github.com/Tiwalun/jep106-parser) helper crate.

## Status

The crate provides the JEP106BL Revision of the codes list which was published in February 2025.

## Usage

See [docs](https://docs.rs/jep106/).

## Development

This crate only contains the library part of the jep106 crate. To change how the PDF itself is
processed, check out the [jep106-parse](https://github.com/Tiwalun/jep106-parser) helper crate.

### Step to use `jep106-parse`

```bash
# clone the repo
git clone https://github.com/Tiwalun/jep106-parser.git
cd jep106-parser
# Download the latest version of the PDF, say, JEP106BE.pdf
# Then run the following command to parse the PDF and generate the `codes.rs`
cargo run -- --pdf JEP106BE.pdf --jep_version BE
# Copy the `codes.rs` to the src/ folder and format it
mv codes.rs /path/to/jep106/src/codes.rs
cargo fmt
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
