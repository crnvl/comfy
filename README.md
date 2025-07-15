
<center>

<img  src="./assets/comfylang.png"  alt="comfy logo">

</center>

  
  

**comfy** is a low-level, compiled scripting language with direct [arm32](https://en.wikipedia.org/wiki/ARM_architecture_family#32-bit_architecture) syscall access.

  

## Features

  

- Direct access to arm32 syscalls

- Simple syntax for low-level programming

- Compiles to arm32 assembly

  

## Example

  

```

fn main() {

$write(1, "hello comfy!\n");

  

let hello_text = ":3\n";

$write(1, hello_text);

  

$exit(703);

}

```

  

## Currently supported syscalls

  

**comfy** provides direct wrappers to arm32 syscalls, stripping away

boilerplate code such as setting up registers manually.

Syscall wrappers are prefixed with `$`.

The following syscalls are currently supported or next in development:

  

| Supported? | Syscall # | Syscall Name | Wrapper Function | Description | Return Value |
| --- | --- | --- | --- | --- | --- |
| ✅ | 1 | `exit` | `$exit(status)` | Terminate the calling process. | Does not return. |
| ✅ | 3 | `read` | `$read(fd, buf)` | Read from a file descriptor. | Number of bytes read, or -1 on error.|
| ✅ | 4 | `write` | `$write(fd, buf)` | Write to a file descriptor. | Number of bytes written, or -1 on error.|
| ✅  | 5 | `open` | `$open(path, flags)` | Open a file. | File descriptor, or -1 on error. |

  
  
  

## Variables

  

**comfy** supports simple variable declarations using the `let` keyword. Variables can hold string literals, or numbers. They can also be used to refer to empty buffers in memory.

  

### Example

  

```comfy

fn main() {

let text = "hello!\n";

$write(1, text);

  

let code = 0;

$exit(code);

}

```

  

You can also reference fixed-size buffers using square brackets. This is especially useful for working with `read()` and other syscalls that require writeable memory pointers. Assigning return values to variables in one line is also possible.

  

```comfy

fn main() {

buf[128] comfySpace; // Declare a 128-byte buffer

let inputSize = $read(0, comfySpace); // Read from stdin into comfySpace and return the amount of bytes read

  

let code = 0;

$exit(code);

}

```

  
  
 

## Project Progress

### Core Language Infrastructure

-  [x]  Tokenizer — Lexes input into tokens
    
-  [x] Parser — Builds AST from tokens
    
-  [x] Generator — Generates raw instructions from AST

### 🔤 Language Features

- [ ] **Variable Declarations**
  - [x] Number values
  - [x] String values
  - [x] Syscall return values
  - [ ] Expressions / arithmetic
- [ ] **Buffer Declarations**
  - [x] Static length buffers (`.lcomm`)
  - [ ] Dynamic/heap-allocated buffers (planned)
- [ ] **Function Definitions**
  - [ ] Basic function definition syntax
  - [ ] Parameter support
  - [ ] Return value support
  - [ ] Nested functions or closures
- [ ] **Type System**
  - [x] Implicit primitive types (`number`, `string`)
  - [ ] Explicit type annotations
  - [ ] Type inference
  - [ ] Type checking
- [ ] **Expressions**
  - [x] Literal values
  - [ ] Binary operations (`+`, `-`, etc.)
  - [ ] Comparison operators (`==`, `<`, etc.)
- [ ] **Control Flow**
  - [ ] If/else
  - [ ] While loops
  - [ ] For loops (planned)

    

### ⚙️ Backend / Architecture Support

-  [x]  ARM32 syscall abstraction layer
    
-  [ ] Complete support for all ARM32 syscalls 
`[█░░░░░░░░░] 10%`
    
-  [ ] Direct inline assembly support (`asm { ... }` blocks)
    

### 🧰 Standard Library

- [ ] Minimal stdlib with helper functions (e.g. print, exit, alloc)
    

### 🛠️ Tooling / Compiler Features

-  [x] Configuration via `.comfx` files
    
-   [x] Architecture targeting system (ARM32 now, future: x86, AArch64, etc.)
    

###  Life Goals

- [ ]  Marry Luna ( Important! )


