use std::env;
use std::path::PathBuf;

use std::process::Command;
use std::process::Stdio;

fn main() {
  let target = env::var("TARGET").unwrap();

  if target == "asmjs-unknown-emscripten" {

    // $ emconfigure ../configure --host=asmjs-unknown-emscripten --disable-assembly --disable-threads --enable-cpuinfo=false CFLAGS="-O2"
    // $ emmake make

    let dst = env::var("OUT_DIR").unwrap();

    {
      let cfg = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("SDL2-2.0.5/configure");
      let mut cmd = Command::new("emconfigure");
      cmd.arg(cfg);
      cmd.args(&["--host=asmjs-unknown-emscripten", "--disable-assembly", "--disable-threads", "--enable-cpuinfo=false", "CFLAGS=-O2"]);
      cmd.current_dir(&dst);
      run(&mut cmd);
    }

    {
      let mut cmd = Command::new("emmake");
      cmd.arg("make");
      cmd.current_dir(&dst);
      run(&mut cmd);
    }

    // println!("cargo:root={}", dst.display());
    println!("cargo:rustc-link-search=native={}/build/.libs", dst);
    // println!("cargo:rustc-link-lib=static=png16");

  }
}

fn run(cmd: &mut Command) {
  println!("running: {:?}", cmd);
  assert!(cmd.stdout(Stdio::inherit())
          .stderr(Stdio::inherit())
          .status()
          .unwrap()
          .success());
}
