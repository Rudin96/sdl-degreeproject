mod engine;
use engine::run;

fn main ()-> Result<(), String> {

    run().unwrap();

    Ok(())
}