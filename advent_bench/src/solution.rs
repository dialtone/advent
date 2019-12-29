use crate::intcode;

pub fn run(input: &str, args: Vec<isize>) -> isize {
    let mut computer = intcode::Computer::from(&input);
    for arg in args {
        computer.write_input(arg);
    }

    while !computer.has_halted() {
        computer.run();
    }

    computer.pop_output().unwrap()
}

#[async_std::test]
async fn test() -> async_std::io::Result<()> {
    use super::*;

    let input = async_std::fs::read_to_string("sum-of-primes").await?;
    assert_eq!(run(&input, vec![100_000]), 454_396_537);
    Ok(())
}
