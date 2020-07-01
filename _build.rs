pub enum Profile {
  Debug,
  ReleaseFast,
  ReleaseSmall,
}

impl Profile {
  fn debug() -> rustc::BuildOptions {
    return rustc::BuildOptions {
      optimization_level: rustc::OptLevel::Numeric(0),
      debug_info: rustc::DebugInfo::Full,
    };
  }

  fn release_small() -> rustc::BuildOptions {
    return rustc::BuildOptions {
      optimization_level: rustc::OptLevel::S,
      debug_info: rustc::DebugInfo::None,
    };
  }

  fn release_fast() -> rustc::BuildOptions {
    return rustc::BuildOptions {
      optimization_level: rustc::OptLevel::Numeric(2),
      debug_info: rustc::DebugInfo::LineTablesOnly,
    };
  }

  fn deref(&self) -> rustc::BuildOptions {
    match self {
      Self::Debug => Self::debug(),
      Self::ReleaseFast => Self::release_fast(),
      Self::ReleaseSmall => Self::release_small(),
    }
  }
}

mod target {
  #[allow(dead_code)]
  pub enum Arch {
    Aarch64,
    Arm,
    ARMebv7r,
    ARM4t,
    ARM5te,
    ARM6,
    ARMV7,
    ARMV7a,
    ARMV7b,
    AsmJS,
    Hexagon,
    I586,
    I686,
    Mips,
    Mipsel,
    Mips64,
    Mips64el,
    PowerPC,
    PowerPC64,
    Sparc,
    Sparc64,
    SparcV9,
    Wasm32,
    X86_64,
  }

  #[allow(dead_code)]
  pub enum ABI {
    GNU,
    Darwin,
    Android,
    Kernel,
    GNUx32,
    Musl,
    NetBSD,
    OpenBSD,
    FreeBSD,
    DragonflyBSD,
    UClibc,
    UEFI,
    MSVC,
    None,
  }

  pub struct Target {
    pub arch: Arch,
    pub abi: ABI,
  }
}

pub struct Builder {
  profile: Profile,
}

impl Builder {
  pub fn default() -> Self {
    return Self {
      profile: Profile::Debug,
    };
  }

  pub fn debug_build_options(&self) -> rustc::BuildOptions {
    return self.profile.deref();
  }

  pub fn default_target_options(&self) -> target::Target {
    return target::Target {
      arch: target::Arch::X86_64,
      abi: target::ABI::GNU,
    };
  }

  pub fn set_build_profile(&mut self, profile: Profile) {
    self.profile = profile;
  }

  pub fn add_executable<'a>(&'a self, exec_name: &'a str, root_path: &'a str) -> Executable<'a> {
    return Executable {
      build_options: self.profile.deref(),
      target: self.default_target_options(),
      name: exec_name,
      path: root_path,
    };
  }
}

pub mod rustc {
  pub enum OptLevel {
    Numeric(u8),
    S,
    Z,
  }

  pub enum DebugInfo {
    None,
    LineTablesOnly,
    Full,
  }

  pub struct BuildOptions {
    pub optimization_level: OptLevel,
    pub debug_info: DebugInfo,
  }

  impl BuildOptions {
    pub fn into_slice(&self) -> Box<[String]> {
      let opt_level = format!("opt-level={}", match &self.optimization_level {
        OptLevel::Numeric(v) if *v <= 3 => *v,
        OptLevel::Numeric(_) => panic!("Invalid optimization level value!"),
        OptLevel::S => 's' as u8,
        OptLevel::Z => 'z' as u8,
      });

      let debuginfo = format!("debuginfo={}", match &self.debug_info {
        DebugInfo::None => 0,
        DebugInfo::LineTablesOnly => 1,
        DebugInfo::Full => 2,
      });

      return Box::new(["-C".into(), debuginfo, "-C".into(), opt_level]);
    }
  }
}

pub struct Executable<'a> {
  build_options: rustc::BuildOptions,
  target: target::Target,
  name: &'a str,
  path: &'a str,
}

impl Executable<'_> {
  pub fn set_build_options(&mut self, options: rustc::BuildOptions) {
    self.build_options = options;
  }

  pub fn set_target(&mut self, target: target::Target) {
    self.target = target;
  }

  pub fn compile(&self) {
    let mut cmd = std::process::Command::new("rustc");
    let cmd = cmd
      .args(&[self.path, "-o", self.name])
      .args(self.build_options.into_slice().iter());

    match cmd.spawn() {
      Ok(mut c) => {
        c.wait().unwrap();
      },
      Err(e) => eprintln!("Failed with {:?}!", e),
    };
  }
}
