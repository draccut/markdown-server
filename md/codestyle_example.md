# Code Blocks

### Hello World in x86-64bit-asm

```
; -------------------------------------------------------
; DUMMY FILE FOR TESTS
; -------------------------------------------------------
section .data
    msg     db "Hello, World!", 10, 0
    msg_len equ $ - msg

section .text
    global _start

_start:                 ; sys_write system call
    mov rax, 1          ; system call number for sys_write
    mov rdi, 1          ; file descriptor 1 (stdout)
    mov rsi, msg        ; message to write
    mov rdx, msg_len    ; number of bytes
    syscall             ; call kernel

                        ; sys_exit system call
    mov rax, 60         ; system call number for sys_exit
    mov rdi, 0          ; exit status
    syscall

```

### Compilation and linking

nasm -f elf64 -o dummy.o dummy.asm && ld -o dummy dummy.o

----

| Type | Offset | VirtAddr |
| :--- | :--- | :--- |
| R    | 0x00000000000000e8 | 0x0000000000400000 |
| R X  | 0x0000000000000027 | 0x0000000000401000 |
| R W  | 0x000000000000000f | 0x0000000000402000 |

### Without linker

```
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>

#define ENTRY_VADDR  0x400000ULL
#define PAGE_ALIGN   0x1000ULL

#pragma pack(push, 1)
typedef struct {
    uint8_t  e_ident[16];
    uint16_t e_type;
    uint16_t e_machine;
    uint32_t e_version;
    uint64_t e_entry;
    uint64_t e_phoff;
    uint64_t e_shoff;
    uint32_t e_flags;
    uint16_t e_ehsize;
    uint16_t e_phentsize;
    uint16_t e_phnum;
    uint16_t e_shentsize;
    uint16_t e_shnum;
    uint16_t e_shstrndx;
} Elf64_Ehdr;

typedef struct {
    uint32_t p_type;
    uint32_t p_flags;
    uint64_t p_offset;
    uint64_t p_vaddr;
    uint64_t p_paddr;
    uint64_t p_filesz;
    uint64_t p_memsz;
    uint64_t p_align;
} Elf64_Phdr;
#pragma pack(pop)

/* x86-64 Machinecode: write(1, msg, len); exit(0); */
static const uint8_t code[] = {
    /* write(1, msg, 13) */
    0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00,       /* mov rax, 1 */
    0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00,       /* mov rdi, 1 */
    0x48, 0x8d, 0x35, 0x13, 0x00, 0x00, 0x00,       /* lea rsi, [rip+0x13] */
    0x48, 0xc7, 0xc2, 0x0d, 0x00, 0x00, 0x00,       /* mov rdx, 13 */
    0x0f, 0x05,                                     /* syscall */
    /* exit(0) */
    0x48, 0xc7, 0xc0, 0x3c, 0x00, 0x00, 0x00,       /* mov rax, 60 */
    0x48, 0x31, 0xff,                               /* xor rdi, rdi */
    0x0f, 0x05,                                     /* syscall */
    /* msg @ rip-relative +0x13 from lea */
    'H','e','l','l','o',' ','W','o','r','l','d','\n',0
};

int main(int argc, char **argv)
{
    const char *out = (argc > 1) ? argv[1] : "hello_rwx";
    FILE *f = fopen(out, "wb");
    if (!f) { perror(out); return 1; }

    const uint64_t ehdr_sz = sizeof(Elf64_Ehdr);
    const uint64_t phdr_sz = sizeof(Elf64_Phdr);
    const uint64_t phoff   = ehdr_sz;
    const uint64_t load_off = 0; /* ganzes File = eine Page-Abbildung */
    /* Code direkt hinter Header+Phdr */
    const uint64_t code_off = ehdr_sz + phdr_sz;
    const uint64_t file_sz  = code_off + sizeof(code);
    /* vaddr des Codes = ENTRY */
    const uint64_t code_va  = ENTRY_VADDR + code_off;

    Elf64_Ehdr eh;
    memset(&eh, 0, sizeof(eh));
    eh.e_ident[0] = 0x7f; eh.e_ident[1] = 'E'; eh.e_ident[2] = 'L'; eh.e_ident[3] = 'F';
    eh.e_ident[4] = 2;    /* ELFCLASS64 */
    eh.e_ident[5] = 1;    /* ELFDATA2LSB */
    eh.e_ident[6] = 1;    /* EV_CURRENT */
    eh.e_type      = 2;    /* ET_EXEC */
    eh.e_machine   = 0x3e; /* EM_X86_64 */
    eh.e_version   = 1;
    eh.e_entry     = code_va;
    eh.e_phoff     = phoff;
    eh.e_shoff     = 0;
    eh.e_flags     = 0;
    eh.e_ehsize    = (uint16_t)ehdr_sz;
    eh.e_phentsize = (uint16_t)phdr_sz;
    eh.e_phnum     = 1;
    eh.e_shentsize = 0;
    eh.e_shnum     = 0;
    eh.e_shstrndx  = 0;

    Elf64_Phdr ph;
    memset(&ph, 0, sizeof(ph));
    ph.p_type   = 1;              /* PT_LOAD */
    ph.p_flags  = 7;              /* PF_R | PF_W | PF_X  = RWX */
    ph.p_offset = load_off;
    ph.p_vaddr  = ENTRY_VADDR;
    ph.p_paddr  = ENTRY_VADDR;
    ph.p_filesz = file_sz;
    ph.p_memsz  = file_sz;        /* eine RWX-„Page“ (Segment) */
    ph.p_align  = PAGE_ALIGN;

    if (fwrite(&eh, 1, sizeof(eh), f) != sizeof(eh) ||
        fwrite(&ph, 1, sizeof(ph), f) != sizeof(ph) ||
        fwrite(code, 1, sizeof(code), f) != sizeof(code)) {
        perror("write");
    fclose(f);
    return 1;
        }
        fclose(f);

        fprintf(stderr,
                "Wrote '%s' (%llu bytes)\n"
                "  entry = 0x%llx\n"
                "  PT_LOAD RWX @ 0x%llx filesz=0x%llx\n",
                out,
                (unsigned long long)file_sz,
                (unsigned long long)code_va,
                (unsigned long long)ENTRY_VADDR,
                (unsigned long long)file_sz);
        return 0;
}
```
