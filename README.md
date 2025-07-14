# comfy

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

| Supported? | Syscall # | Syscall Name | Wrapper Function     | Description                    | Return Value                          |
|------------|-----------|--------------|----------------------|--------------------------------|---------------------------------------|
| ✅         | 1         | `exit`       | `$exit(status)`      | Terminate the calling process. | Does not return.                      |
| ✅         | 3         | `read`       | `$read(fd, buf)`     | Read from a file descriptor.   | Number of bytes read, or -1 on error.|
| ✅         | 4         | `write`      | `$write(fd, buf)`    | Write to a file descriptor.    | Number of bytes written, or -1 on error.|
| ⏳ In dev. | 5         | `open`       | `$open(path, flags)` | Open a file.                   | File descriptor, or -1 on error.     |



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
    buf[128] comfySpace;                         // Declare a 128-byte buffer
    let inputSize = $read(0, comfySpace);        // Read from stdin into comfySpace and return the amount of bytes read

    let code = 0;
    $exit(code);
}
```


## Progress

- [ ] Tokenizer
- [ ] Parser
- [ ] Generator
- [ ] Support for all arm32 syscalls
- [ ] Direct asm support
- [ ] stdlib for helper functions
- [ ] Marry luna


## Todos

- [ ] Write abstraction layer to avoid inline asm (maybe platform specific modules?)