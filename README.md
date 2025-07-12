# comfy

**comfy** is a low-level, compiled scripting language with direct [arm32](https://en.wikipedia.org/wiki/ARM_architecture_family#32-bit_architecture) syscall access.

## Syntax

```
fn main() {
    $write(1, "hello comfy!\n");
    $exit(703);
}
```

## Progress

- [ ] Tokenizer
- [ ] Parser
- [ ] Generator
- [ ] Support for all arm32 syscalls
- [ ] stdlib for helper functions
