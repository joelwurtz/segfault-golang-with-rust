extern crate libc;

use html5ever::LocalName;

#[no_mangle]
pub extern "C" fn api_do_segfault(data_cstr: *const libc::c_char) {
    let data = cstr_to_str(data_cstr);

    do_segfault(data.as_str());
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

fn do_segfault(data: &str) {
    LocalName::from(&*data);

    println!("It works with {}", data);
}