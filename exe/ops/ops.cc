#define WIN32_LEAN_AND_MEAN
#define STRICT
#include <windows.h>
#include <stdint.h>

int str_len(const char* str) {
  for (int i = 0; ; i++) {
    if (str[i] == 0) return i;
  }
}

void print(const char* buf) {
  WriteFile(GetStdHandle(STD_OUTPUT_HANDLE), buf, str_len(buf), nullptr, nullptr);
}

void print_hex(uint32_t x) {
  char buf[16];
  buf[15] = 0;
  int pos = 15;
  do {
    pos--;
    buf[pos] = "0123456789abcdef"[x & 0xf];
    x = x >> 4;
  } while (x > 0);
  print(buf + pos);
}

void print_flags(uint16_t flags) {
  if ((flags & 1) != 0)
    print(" CF");
  if ((flags & (1<<6)) != 0)
    print(" ZF");
  if ((flags & (1<<7)) != 0)
    print(" SF");
  if ((flags & (1<<10)) != 0)
    print(" DF");
  if ((flags & (1<<11)) != 0)
    print(" OF");
}

#define asm_start(desc) { \
  print(desc); \
  uint32_t result; \
  uint16_t flags = 0; \
  __asm { \
    __asm push flags \
    __asm popf \

#define asm_end() \
    __asm mov result,eax \
    __asm pushf \
    __asm pop flags \
  } \
  print(" => "); \
  print_hex(result); \
  print_flags(flags); \
  print("\n"); \
}

void add() {
#define add(x,y) \
  asm_start("add " #x "," #y) \
    __asm mov eax,x \
    __asm add eax,y \
  asm_end();
  add(3, 5);
  add(3, -3);
  add(3, -5);
#undef add
}

void shr() {
#define shr(x,y) \
  asm_start("shr " #x "," #y) \
    __asm mov eax,x \
    __asm shr eax,y \
  asm_end();
  shr(3, 0);
  shr(3, 1);
  shr(3, 2);
  shr(0x80000000, 1);
  shr(0x80000000, 2);
  shr(0x80000001, 1);
  shr(0x80000001, 2);
#undef shr
}

void sar() {
#define sar(x,y) \
  asm_start("sar " #x "," #y) \
    __asm mov eax,x \
    __asm sar eax,y \
  asm_end();
  sar(3, 0);
  sar(3, 1);
  sar(3, 2);
  sar(0x80000000, 1);
  sar(0x80000000, 2);
  sar(0x80000001, 1);
  sar(0x80000001, 2);
  sar(0x80000002, 1);
  sar(0x80000002, 2);
#undef sar
}

void shl() {
#define shl(x,y) \
  asm_start("shl " #x "," #y) \
    __asm mov eax,x \
    __asm shl eax,y \
  asm_end();
  shl(3, 0);
  shl(3, 1);
  shl(3, 2);
  shl(0x80000000, 1);
  shl(0x80000000, 2);
  shl(0xD0000001, 1);
  shl(0xD0000001, 2);
  shl(0xE0000002, 1);
  shl(0xE0000002, 2);
#undef shl
}

void mainCRTStartup(void) {
  add();
  shr();
  sar();
  shl();
}
