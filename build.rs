use std::env;

//#[cfg(windows)]
#[cfg(target_os="windows")]
fn main() {    
    let out_dir = env::var("CARGO_MANIFEST_DIR").ok().expect("can't find out_dir");
    println!("cargo:rustc-link-search={}/src/windows.static",out_dir);
    println!("cargo:rustc-link-lib=rpcrt4");
    println!("cargo:rustc-link-lib=ws2_32");
}

//#[cfg(not(windows))]  
#[cfg(all(target_os="linux",not(any(feature="android_arm",feature="android_x86",feature="android_x86_64",feature="android_arm64"))))]
fn main() {  
    let out_dir = env::var("CARGO_MANIFEST_DIR").ok().expect("can't find out_dir");
    println!("cargo:rustc-link-search={}/src/linux.static",out_dir);
}

#[cfg(all(target_os="linux",feature="android_arm"))]
fn main() {    
    let out_dir = env::var("CARGO_MANIFEST_DIR").ok().expect("can't find out_dir");
    println!("cargo:rustc-link-search={}/src/android.static/armv7",out_dir);
}

#[cfg(all(target_os="linux",feature="android_arm64"))]
fn main() {    
    let out_dir = env::var("CARGO_MANIFEST_DIR").ok().expect("can't find out_dir");
    println!("cargo:rustc-link-search={}/src/android.static/arm64",out_dir);
}

#[cfg(all(target_os="linux",feature="android_x86"))]
fn main() {    
    let out_dir = env::var("CARGO_MANIFEST_DIR").ok().expect("can't find out_dir");
    println!("cargo:rustc-link-search={}/src/android.static/x86",out_dir);
}

#[cfg(all(target_os="linux",feature="android_x86_64"))]
fn main() {    
    let out_dir = env::var("CARGO_MANIFEST_DIR").ok().expect("can't find out_dir");
    println!("cargo:rustc-link-search={}/src/android.static/x86_64",out_dir);
}