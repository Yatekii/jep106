use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("jep106.rs");
    let mut f = File::create(&dest_path).unwrap();

    let version = "JEP106AY";

    let contents = pdf_extract::extract_text(version.to_owned() + ".pdf")
        .expect("Something went wrong reading the file");

    let mut data: Vec<Vec<Option<String>>> = vec![];

    for line in contents.lines() {
        use regex::Regex;
        let re = Regex::new(r"^[0-9]+\s+(.*?)\s+([01]\s+){8}([0-9A-F]{2})\s+$").unwrap();
        if let Some(capture) = re.captures(line) {
            if &capture[3] == "01" {
                data.push(vec![None; 256]);
            }
            data.iter_mut().last().expect("This is a bug.")[usize::from_str_radix(&capture[3], 16).expect("This is a bug.") & 0x7F] = Some(capture[1].into());
        }
    }

    let _ = f.write(format!("const CODES: [[Option<&'static str>; 256]; {}] = [", data.len()).as_bytes());

    for bank in data.iter() {
        let _ = f.write(b"[");
        for company in bank {
            if let Some(company) = company {
                let _ = f.write(format!("Some(\"{}\"),", company).as_bytes());
            } else {
                let _ = f.write(b"None,");
            }
        }
        let _ = f.write(b"],");
    }

    let _ = f.write(b"];");

    let _ = f.write_all(b"
        /// Returns the manufacturer corresponding to a complete JEP106 code.
        /// Returns an empty string if the JEP106 code is unknown.
        const fn get(cc: u8, id: u8) -> Option<&'static str> {
            CODES[cc as usize][id as usize]
        }
    ").unwrap();

    let _ = f.write_all(format!("
        /// Returns the JEP106 specification version code.
        pub const fn version() -> &'static str {{
            \"{}\"
        }}
    ", version).as_bytes()).unwrap();
}