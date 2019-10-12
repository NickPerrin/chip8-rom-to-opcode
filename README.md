# chip8-rom-to-opcode
Library to translate binary or hex chip8 roms into human readable opcodes.

## Motivation

This is the core component of a disassembler for Chip-8 roms, which can also be uses as a debugging tool in a Chip-8 emulator.

## Usage

Convert a vector of bytes(u8) into human readable memonics. The strings are returned as Vec<String> to allow the user to format, display, or use the results however they like. 
