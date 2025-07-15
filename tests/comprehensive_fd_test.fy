fn test_numeric_fd() {
    $write(1, "Test 1: Numeric FD\n");
}

fn test_variable_fd(fd: 4) {
    $write(fd, "Test 2: Variable FD\n");
}

fn main() {
    $write(1, "Starting tests...\n");
    $write(2, "This goes to stderr\n");
    $exit(0);
}