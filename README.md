# comfy
**comfy** is a low-level, compiled scripting language with direct [arm32](https://en.wikipedia.org/wiki/ARM_architecture_family#32-bit_architecture) syscall access.

## Syntax
```
fn main() {                         // main() is the starting point of each program
  let text = "hello";
  let number = 703;

  print(text + " " + number);
  raw_print(text + " " + number);          
  exit(0);                          // directly calls the exit syscall
}

// both versions result in the same output
fn print(str) {
  std::std_w(str);                  // std lib function to write to std out
}

fn raw_print(str) {
  write(1, str, str.len);           // syscall write to std out
}


fn raw_open(path) {
  let fd = open(path, 0, 0);        // syscall open with file descriptor return value
}
```

## Progress
- [ ] Tokenizer
- [ ] Parser
- [ ] Generator
- [ ] Support for all arm32 syscalls
- [ ] stdlib for helper functions
