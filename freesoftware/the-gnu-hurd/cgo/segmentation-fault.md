# Segmentation fault

## Overview
In computing, a **segmentation fault** (often shortened to **segfault**) or **access violation** is a fault, or failure condition, raised by hardware with memory protection, notifying an operation system the software has attempted to access a restricted area of memory (a memory access violation).

The operating system kernel will in response, usually perform some corrective action, generally passing the fault on to the offending process by sending the process a signal. And the OS default signal handler generally causing abnormal termination of the process, and sometimes a core dump.

Many language employ mechnisms designed to avoid segmentation faults and improve memory safety. For example:
* Rust ***employs*** an **ownership-based model** to ensure memory safety.
* Golang ***employs*** a **garbage collector** to ensure memory safety.
* Python ***employs*** a **garbage collector** to ensure memory safety.

## Memory protection
Memory protection is a way to control memory access rights on a computer, and **is a part of most modern instruction set architectures and operating systems**. The **main purpose of memory protection** is to **prevent a process from accessing memory that has not been allocated to it**. 

This **prevents a bug or malware** within a process from affecting other processes, or the operating system itself. 

**Memory protection can also be used to help prevent a process from accessing memory that it should not be able to see.** For example, password checking programs work by comparing a password string input by the user to a stored copy of the password. If the password is stored in plaintext, then the password checking program must be able to read the stored password. However, if the password is stored as a cryptographic hash, then the password checking program should not be able to read the stored password, because the program does not need to know the password itself, only whether the password entered by the user is the same as the stored password. If the password checking program is able to read the stored password, then it is possible for a malicious user to exploit a bug in the password checking program to read the stored password, and then use it to gain access to the system. Memory protection can be used to prevent this from happening.

### Methods

#### Segmentation

Segmentation refers to dividing a computer's memory into segments. 

A reference to a memory location includes:
* a value that identifies a segment
* an offset within that segment

A segment descriptor may limit access rights, e.g:, read-only, only from certain rings.

**x86 architecture**
It has multiple segmentation features, the Global Descriptor Table (GDT) and Local Descriptor Table (LDT) for x86 processors, and the Interrupt Descriptor Table (IDT) for x86 processors. **Pointers to memory segments on x86 processors** can also be stored in the processor's segment register, like CS (code segment), DS (data segment), ES (extra segment), SS (stack segment), FS (general purpose), and GS (general purpose).


## Source
https://github.com/go-skynet/LocalAI/issues/622

## Credit
https://en.wikipedia.org/wiki/Segmentation_fault
