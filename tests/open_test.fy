fn main() {
    $write(1, "hello comfy, open syscall test!\n");

    let fPath = "/home/asm/testfile";
    let fd = $open(fPath, 577, 420);

    $exit(69);
}