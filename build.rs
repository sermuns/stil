use base64::prelude::*;

fn main() {
    println!("cargo::rerun-if-changed=media/logo.svg");
    let logo_contents = std::fs::read_to_string("media/logo.svg").expect("unable to find logo");
    let logo_b64 = BASE64_STANDARD.encode(logo_contents);
    println!("cargo::rustc-env=LOGO_B64={logo_b64}");
}
