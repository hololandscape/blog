# Introduciton to 6811 programming

## Overview

This article is picked from the article [Introduction to 6811 programming](https://www.cs.uml.edu/~fredm/courses/91.305-fall04/files/6811intr.pdf).

## Bits and Bytes

In computers, information is represented with voltages, and it is most convenitent for he voltage level to represebt only two states:
* A binary one or binary zero

Thus computers process binary digits, or bits. For example, in an 8-bit numeral, 256 different states can be represented, since

$$
2^8 = 256
$$

Programmers use these 256 states to represent different things, like:
* a natural number from 0 to 255;
* an interger in the range of -128 to 127;
* a character of data(a letter, number, or printable symbol);

When programmers need to represent larger numberal, they group bytes together. A common grouping is two bytes, often called a 16-bit word, or a *short*(a "regular" word is 4 bytes of data). A word can have 65536 sattes, since 

$$
2^16 = 65536
$$

## Hexadecimal Notation

Decimal numbers are painful to use when talking about binary information. To make life easier, programmers started to use the base 16 *hexadecimal*(*hex* for short) numbering system whne taling about bits, bytes, and other binary data.

Hexadecimal numbers are written using the digits 0 through 9 and the letters A through F. The letters A through F represent the decimal numbers 10 through 15, respectively.

binary|decimal|hex
---|---|---
0b000|0|0x0
0b001|1|0x1
0b010|2|0x2
0b011|3|0x3
0b100|4|0x4
0b101|5|0x5
0b110|6|0x6
0b111|7|0x7
0b1000|8|0x8
0b1001|9|0x9
0b1010|10|0xA
0b1011|11|0xB
0b1100|12|0xC
0b1101|13|0xD
0b1110|14|0xE
0b1111|15|0xF
0b10000|16|0x10

According to table above, we can see that Binary numbers are represented by the prefix **0b**. Hexadecimal numbers are represented by the prefix **0x**. Decimal numbers are represented by no prefix.

## Memory Map

Memory is oraganized as a contiguous string of addresses, or locations. The entire amount of memory that a processor can access is called its address space.

## Registers

A microprocessor does its work by moving data from memory into its *internal registers*, processing on it, and then copying it back into memory. There are two different types of registers:

### Accumulators
Accumulators are used to perform most arithmetic operations

### Index registers
Index registers are used to point at data that is located in memory


## Machine Code VS Assembly Language

Both of these terms refer to the same thing: the program that is executed directly by the microprocessor

### Machine Code

Machine code usually refres to the raw data stored as microprocessor's program. This is commonly described in the hexadecimal notation we've been using.

Assembly language is a set of mnemonics, or names, and a notation that is a readable yet efficient wat of writing down the machine instructions. Usually, a program taht is written in assembly language is processed by an assembler program, that converts the mnemonic instructions into machine code. The output from the assembler program is often called the *object code*, which can then executed directly by the microprocessor.


## Signed and Unsigned Binary Numbers

There are methods of representing binary numbers that are commonly used by microprocessors.

### Unsigned Binary Numbers
The unsigned format is used to represent numbers in the range of 0 to 255 or 0 to 65535(one word of data). But, the unsgined format has the limitation that values less than zero cannot be represented.

### Signed Binary Numbers
The signed format is used to represent numbers in the range of -128 to 127 or -32768 to 32767(one word of data). The signed format has the limitation that values greater than 127 or 32767 cannot be represented.

