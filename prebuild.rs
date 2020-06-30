pub enum Profile {
  Debug,
  ReleaseFast,
  ReleaseSmall,
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
    return match self.profile {
      Profile::Debug => rustc::BuildOptions::debug(),
      _ => unimplemented!(),
    };
  }

  pub fn add_executable<'a>(&'a self, exec_name: &'a str, root_path: &'a str) -> Executable<'a> {
    return Executable {
      build_options: rustc::BuildOptions::debug(),
      name: exec_name,
      path: root_path,
    };
  }
}

macro_rules! char_to_str {
  ($c:expr) => {
    Box::leak($c.to_string().into_boxed_str());
  };
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
    optimization_level: OptLevel,
    debug_info: DebugInfo,
  }

  impl BuildOptions {
    pub fn debug() -> Self {
      return Self {
        optimization_level: OptLevel::Numeric(0),
        debug_info: DebugInfo::Full,
      };
    }

    pub fn into_slice(&self) -> [&str; 2] {
      let opt_level = match &self.optimization_level {
        OptLevel::Numeric(val) if *val >= 0 && *val <= 3 => char_to_str!(val),
        S => "s",
        Z => "z",
      };
      
      return ["-C", Box::leak(format!("opt-level={}", opt_level).into_boxed_str())];
    }
  }
}

pub struct Executable<'a> {
  build_options: rustc::BuildOptions,
  name: &'a str,
  path: &'a str,
}

impl Executable<'_> {
  pub fn set_build_options(&mut self, options: rustc::BuildOptions) {

  }

  pub fn compile(&self) {
    let mut cmd = std::process::Command::new("rustc");
    let cmd = cmd
      .args(&[self.path, "-o", self.name])
      .args(&self.build_options.into_slice());

    match cmd.spawn() {
      Ok(mut c) => {
        c.wait();
      },
      Err(e) => eprintln!("Failed with {:?}!", e),
    };
  }
}
