extern crate tiff;

use tiff::TiffResult;
use tiff::decoder::Decoder;

use std::fs::File;

fn test_directory<F: Fn(File) -> bool>(path: &str, f: F) {
    for entry in std::fs::read_dir(path).unwrap() {
        let file = File::open(entry.unwrap().path()).unwrap();
        assert!(f(file));
    }
}

fn decode_tiff(file: File) -> TiffResult<()> {
    let mut decoder = Decoder::new(file)?;
    decoder.read_image()?;
    Ok(())
}

#[test]
fn oor_panic() {
    test_directory("./tests/fuzz_images/oor_panic", |file| {
        decode_tiff(file).ok();
        true
    });
}

#[test]
fn oom_crash() {
    test_directory("./tests/fuzz_images/oom_crash", |file| {
        decode_tiff(file).is_err()
    });
}

#[test]
fn inf_loop() {
    test_directory("./tests/fuzz_images/inf_loop", |file| {
        decode_tiff(file).ok();
        true
    });
}
