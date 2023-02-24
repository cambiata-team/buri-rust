use e2e::build_tests;

fn main() -> Result<(), String> {
    match build_tests() {
        Ok(s) => {
            println!("{s}");
            Ok(())
        }
        Err(e) => Err(e),
    }
}
