pub mod prebuild;
mod build;

fn main() {
  let mut builder = prebuild::Builder::default();
  build::build(&mut builder);
}
