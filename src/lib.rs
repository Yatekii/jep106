//! This crate provides a means to retrieve the JEDEC manufacturer string
//! for a corresponding JEP106 ID Code.
//!
//! All the codes can be found on the page of the JEDEC organization
//! but are presented in the riddiculous form of a PDF.
//!
//! This crate parses the PDF and exposes an interface
//! to poll the JEDEC manufacturer string of a JEP106 ID code.
//!
//! # Example
//!
//! ```
//! fn main() {
//!     let nordic = jep106::JEP106Code::new(0x02, 0x44).get();
//!     assert_eq!(Some("Nordic VLSI ASA"), nordic);
//! }
//! ```

mod codes;

#[macro_use]
extern crate serde;

pub use codes::version;

/// A Struct which contains a fully qualified JEP106 manufacturer code.
#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JEP106Code {
    /// JEP106 identification code.
    /// Points to a manufacturer name in the bank table corresponding to `cc`.
    pub id: u8,
    /// JEP106 continuation code.
    /// This code represents the bank which the manufacturer for a corresponding `id` has to be looked up.
    pub cc: u8,
}

impl JEP106Code {
    /// Creates a new [JEP106Code](struct.JEP106Code.html) struct from
    /// a JEP106 continuation code and a JEP106 id code tuple.
    pub const fn new(cc: u8, id: u8) -> Self {
        Self { id, cc }
    }

    /// Returns the manufacturer corresponding to a complete JEP106 code.
    ///
    /// Returns an empty string if the JEP106 code is unknown.
    pub const fn get(&self) -> Option<&'static str> {
        get(self.cc, self.id)
    }

    /// Returns the manufacturer corresponding to
    /// a JEP106 continuation code and a JEP106 id code tuple.
    ///
    /// Returns an empty string if the JEP106 code is unknown.
    pub const fn get_raw(cc: u8, id: u8) -> Option<&'static str> {
        get(cc, id)
    }
}

impl std::fmt::Debug for JEP106Code {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "JEP106Code({{ cc: 0x{:02x?}, id: 0x{:02x?} }} => {:?})",
            self.cc,
            self.id,
            self.get()
        )
    }
}

impl std::fmt::Display for JEP106Code {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get().unwrap_or("Unknown Manufacturer Code"))
    }
}

/// Returns the manufacturer corresponding to a complete JEP106 code.
/// Returns an empty string if the JEP106 code is unknown.
const fn get(cc: u8, id: u8) -> Option<&'static str> {
    // The `get` function is unfortunately not const,
    // so we have to check the index manually.
    if cc as usize > codes::CODES.len() {
        return None;
    }

    let cc_values = codes::CODES[cc as usize];

    if id as usize > cc_values.len() {
        return None;
    }

    cc_values[id as usize]
}
