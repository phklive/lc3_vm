pub enum Opcodes {
    Br = 0,    // branch
    Add = 1,   // add
    Ld = 2,    // load
    St = 3,    // store
    Jsr = 4,   // jump register
    And = 5,   // bitwise and
    Ldr = 6,   // load register
    Str = 7,   // store register
    Rti = 8,   // unused
    Not = 9,   // bitwise not
    Ldi = 10,  // load indirect
    Sti = 11,  // store indirect
    Jmp = 12,  // jump
    Res = 13,  // reserved (unused)
    Lea = 14,  // load effective address
    Trap = 15, // execute trap
}
