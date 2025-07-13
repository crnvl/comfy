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
    $exit(703);
}
```

## Currently supported syscalls

**comfy** provides direct wrappers to arm32 syscalls, stripping away
boilerplate code such as setting up registers manually.
Syscall wrappers are prefixed with `$`.
The following syscalls are currently supported:

| Supported? | Syscall Name | Wrapper Function  | Description                    |
| ---------- | ------------ | ----------------- | ------------------------------ |
| ✅         | `exit`       | `$exit(status)`   | Terminate the calling process. |
| ✅         | `write`      | `$write(fd, buf)` | Write to a file descriptor.    |

## Progress

- [ ] Tokenizer
- [ ] Parser
- [ ] Generator
- [ ] Support for all arm32 syscalls
- [ ] Direct asm support
- [ ] stdlib for helper functions
- [ ] Marry luna