fn helper(stdout: 4, stderr: 4) {
    $write(stdout, "Output to stdout\n");
    $write(stderr, "Output to stderr\n");
    $write(1, "Direct numeric fd\n");
}

fn main() {
    $write(1, "Hello from main!\n");
    $exit(0);
}