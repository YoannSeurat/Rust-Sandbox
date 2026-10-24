/*
https://codingdojo.org/kata/Brainfuck/

Brainfuck

Introduction
This exercise is based on the Brainfuck langage.

Brainfuck is an esoteric programming language created in 1993 by Urban Müller. It’s a minimalist langage composed of one character instructions while remaining Turing complete.

Problem Description
Create an interpreter, which, given a valid Brainfuck program execute it and return the memory state at the end of it.

The Brainfuck langage consists of :
a program;
a one-dimensional array of at least 30,000 byte cells initialized to zero, representing ASCII characters;
a movable data pointer (initialized to point to the leftmost byte of the array).
The program is composed of 8 different commands :

Character	Meaning
+	Increment the byte at the data pointer.
-	Decrement the byte at the data pointer.
>	Increment the data pointer.
<	Decrement the data pointer.
.	Output the byte at the data pointer (ASCII format)
,	Accept one byte of input (ASCII format), storing its value in the byte at the data pointer.
[	If the byte at the data pointer is zero jump it forward to the command after the matching ] command.
]	If the byte at the data pointer is nonzero jump it back to the command after the matching [ command.

Suggested steps
Implement + - > < commands to manage memory.
Implement . , to manage I/O.
Implement [ ] to manage jump/loops.
*/

fn main() {
    
}
