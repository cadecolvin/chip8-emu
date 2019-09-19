mod chip8;
use chip8::Chip8;

fn main() {
    let chip8 = Chip8::new();
    let right_size = 0x05;
    let too_big = 0x0345;
    let bottom_8 = 0x45;

    println!("right_size: {}", right_size as u8);
    println!("too_big: {}", too_big);
    println!("too_big truncated: {}", too_big as u8);
    println!("bottom_8: {}", bottom_8 as u8);
}
