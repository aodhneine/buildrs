mod prebuild;

pub fn build(b: &mut prebuild::Builder) {
  let profile = b.debug_build_options();
  let mut build_exec = b.add_executable("build", "build.rs");
  build_exec.set_build_options(profile);
  build_exec.compile();
}

fn main() {
  let mut builder = prebuild::Builder::default();
  build(&mut builder);
}
