fn main() {
    $write(1, "hello comfy, inline asm test!\n");

    asm {
        section bss {
            ".lcomm grrr, 4"
        },
        "ldr r8, =grrr",
        "mov r6, #420" 
    };
    
    $write(1, "^Above this should be inline asm^\n");

    $exit(69);
}