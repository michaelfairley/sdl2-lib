use std::env;
use std::path::PathBuf;

use std::process::Command;
use std::process::Stdio;

fn main() {
  let target = env::var("TARGET").unwrap();
  let out_dir = env::var("OUT_DIR").unwrap();

  match target.as_str() {
    "asmjs-unknown-emscripten" => {
      {
        let cfg = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("SDL2-2.0.5/configure");
        let mut cmd = Command::new("emconfigure");
        cmd
          .arg(cfg)
          .args(&["--host=asmjs-unknown-emscripten", "--disable-assembly", "--disable-threads", "--enable-cpuinfo=false", "CFLAGS=-O2"])
          .current_dir(&out_dir);
        run(&mut cmd);
      }

      {
        let mut cmd = Command::new("emmake");
        cmd
          .arg("make")
          .current_dir(&out_dir);
        run(&mut cmd);
      }

      println!("cargo:rustc-link-search=native={}/build/.libs", out_dir);
    },
    "x86_64-apple-darwin" => {
      let flags = "-mmacosx-version-min=10.6 -DMAC_OS_X_VERSION_MIN_REQUIRED=1060 -arch x86_64";

      {
        let cfg = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("SDL2-2.0.5/configure");
        let mut cmd = Command::new(cfg);
        cmd
          .current_dir(&out_dir)
          .env("CFLAGS", flags)
          .env("CXXFLAGS", flags);
        run(&mut cmd);
      }

      {
        let mut cmd = Command::new("make");
          cmd
          .current_dir(&out_dir);
        run(&mut cmd);
      }

      println!("cargo:rustc-link-search=native={}/build/.libs", out_dir);
    },
    other => panic!("sdl2-lib is not implemented for {}", other),
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
