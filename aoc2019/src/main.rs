use aoc2019::intcode::Intcode;

fn main() {
    let prg = std::env::args().nth(1).expect("Must provide a source prg");
    let asm = Intcode::new(prg.split(',').filter_map(|s| s.parse().ok()).collect()).disassemble();
    println!("{asm}");
}
