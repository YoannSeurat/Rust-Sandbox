# Brainfuck

Brainfuck is a programming language created in 1993 by Swiss student Urban Müller. 

Designed to be extremely minimalistic, the language consists of only eight simple commands, a data pointer, and an instruction pointer.

While Brainfuck is fully Turing-complete, it is not intended for practical use but to challenge and amuse programmers.

## How it works

The eight language commands each consist of a single character:

| Character | Instruction Performed |
| --------- | --------------------- |
| `>`       | Increment the data pointer by one (to point to the next cell to the right). |
| `<`       | Decrement the data pointer by one (to point to the next cell to the left). Undefined if at 0. |
| `+`       | Increment the byte at the data pointer by one modulo 256. |
| `-` | Decrement the byte at the data pointer by one modulo 256. |
| `.`       | Output the byte at the data pointer. |
| `,`       | Accept one byte of input, storing its value in the byte at the data pointer.  |
| `[`       | If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching `]` command.  |
| `]`       | If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching `[` command. |

## Run

```bash
cargo run
```

Then enter a file number to test its program.

## Example

```text
Please enter the file number to test (1-11) : 
7

Hello World!

0 0 72 100 87 33 10 
```
