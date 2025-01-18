Chip-8 emulator and assembler written in rust.

The input is a bit janky, because its not possible to detect key release events on the terminal,
and I didn't want to use a GUI library.

## Usage
```shell
chip8emu <rom>
```

## Assembler
```shell
chip8emu <output> --assemble=<input>
```
See [chip-8 assembly language](docs/assembly_lang.md) for information about instructions.

## Tests
The tests are written using macros, to make them somewhat declarative.
```rust
#[test]
fn test_jump() {
    cpu_test!(r#"
            jmp 0x206
            add v0 0x2
            add v0 0x5
        "# 
        [0x01, 0x01] => [0x01, 0x01]
    );
}
```
It consists of three parts, the assembly code, the initial state of the registers, and the expected
state of the registers after the code is executed.

Visual tests weren't automated, and instead it was tested with roms.
