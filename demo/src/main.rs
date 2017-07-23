extern crate sdl2;

fn main() {
  sdl2::init().unwrap();
}

#[cfg(target_os = "ios")]
#[link(name = "AVFoundation", kind = "framework")]
#[link(name = "UIKit", kind = "framework")]
#[link(name = "OpenGLES", kind = "framework")]
#[link(name = "QuartzCore", kind = "framework")]
#[link(name = "CoreAudio", kind = "framework")]
#[link(name = "AudioToolbox", kind = "framework")]
#[link(name = "CoreGraphics", kind = "framework")]
#[link(name = "CoreMotion", kind = "framework")]
#[link(name = "GameController", kind = "framework")]
extern {}
