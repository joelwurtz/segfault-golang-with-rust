extern crate libc;

#[macro_use]
extern crate html5ever;

use std::default::Default;
use html5ever::QualName;
use html5ever::driver::ParseOpts;
use html5ever::rcdom::RcDom;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::{TreeBuilderOpts};
use html5ever::{serialize, parse_fragment};

#[no_mangle]
pub extern "C" fn html5ever_parse_data(data_cstr: *const libc::c_char) {
    let data = cstr_to_str(data_cstr);

    parse_data(data.as_str());
}

fn cstr_to_str(data_cstr: *const libc::c_char) -> String {
    unsafe {
        let cstring = std::ffi::CStr::from_ptr(data_cstr);
        let result = cstring.to_str();

        if result.is_err() {
            println!(
                "Unable to create string for '{}': {}",
                String::from_utf8_lossy(cstring.to_bytes()),
                result.err().unwrap()
            );

            return "".to_string();
        }

        result.unwrap().to_string()
    }
}

fn parse_data(data: &str) {
    let mut b = data.as_bytes();
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let dom = parse_fragment(
        RcDom::default(),
        opts,
        QualName::new(None, ns!(html), local_name!("body")),
        Vec::new(),
    )
        .from_utf8()
        .read_from(&mut b)
        .unwrap();

    serialize(&mut std::io::stdout(), &dom.document, Default::default())
        .ok()
        .expect("serialization failed");

    println!("\n");

//    println!("{:?}", dom.finish());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_parse_data() {
        parse_data("<html><head><meta property=\"test\" content=\"yolo\" /></head></html>")
    }
}