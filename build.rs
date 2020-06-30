use prebuild;

pub fn build(b: &mut prebuild::Builder) {
  let profile = b.debug_build_options();
  let mut main = b.add_executable("main", "main.rs");
  main.set_build_options(profile);
  main.compile();
}
