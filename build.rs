// Build the library.

extern crate gcc;

fn main() {
    let mut conf = gcc::Config::new();

    conf.file("bootutil/src/loader.c");
    conf.file("bootutil/src/bootutil_misc.c");
    conf.file("bootutil/src/run.c");
    conf.include("bootutil/include");
    conf.debug(true);
    conf.compile("libbootutil.a");
}
