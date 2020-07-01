#[path = "_build.rs"]
mod build;

fn main() {
  let mut builder = build::Builder::default();
  builder.set_build_profile(build::Profile::Debug);

  let build_options = builder.debug_build_options();
  let target = builder.default_target_options();
  let mut exe = builder.add_executable("build", "build.rs");

  exe.set_build_options(build_options);
  exe.set_target(target);
  exe.compile();
}
