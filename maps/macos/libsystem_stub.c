// Stub libSystem.B.dylib for mwemu
// Build aarch64: clang -shared -o aarch64/libSystem.B.dylib -target arm64-apple-macos -Wl,-install_name,/usr/lib/libSystem.B.dylib libsystem_stub.c
// Build x86_64:  clang -shared -o x86_64/libSystem.B.dylib -target x86_64-apple-macos -Wl,-install_name,/usr/lib/libSystem.B.dylib libsystem_stub.c

#include <stdarg.h>

int printf(const char *fmt, ...) { (void)fmt; return 0; }
int fprintf(void *stream, const char *fmt, ...) { (void)stream; (void)fmt; return 0; }
int sprintf(char *dst, const char *fmt, ...) { (void)dst; (void)fmt; return 0; }
int snprintf(char *dst, unsigned long size, const char *fmt, ...) { (void)dst; (void)size; (void)fmt; return 0; }
int puts(const char *s) { (void)s; return 0; }
int putchar(int c) { (void)c; return 0; }
int vprintf(const char *fmt, va_list ap) { (void)fmt; (void)ap; return 0; }
int vfprintf(void *stream, const char *fmt, va_list ap) { (void)stream; (void)fmt; (void)ap; return 0; }
int vsprintf(char *dst, const char *fmt, va_list ap) { (void)dst; (void)fmt; (void)ap; return 0; }
int vsnprintf(char *dst, unsigned long size, const char *fmt, va_list ap) { (void)dst; (void)size; (void)fmt; (void)ap; return 0; }

void exit(int status) { (void)status; for (;;) {} }
void _exit(int status) { (void)status; for (;;) {} }
void abort(void) { for (;;) {} }

void *malloc(unsigned long size) { (void)size; return 0; }
void *calloc(unsigned long count, unsigned long size) { (void)count; (void)size; return 0; }
void *realloc(void *ptr, unsigned long size) { (void)ptr; (void)size; return 0; }
void free(void *ptr) { (void)ptr; }

int atexit(void (*func)(void)) { (void)func; return 0; }

long write(int fd, const void *buf, unsigned long count) { (void)fd; (void)buf; (void)count; return 0; }
long read(int fd, void *buf, unsigned long count) { (void)fd; (void)buf; (void)count; return 0; }
int open(const char *path, int flags, ...) { (void)path; (void)flags; return 3; }
int close(int fd) { (void)fd; return 0; }

unsigned long strlen(const char *s) { (void)s; return 0; }
int strcmp(const char *a, const char *b) { (void)a; (void)b; return 0; }
int strncmp(const char *a, const char *b, unsigned long n) { (void)a; (void)b; (void)n; return 0; }
char *strcpy(char *dst, const char *src) { (void)src; return dst; }
char *strncpy(char *dst, const char *src, unsigned long n) { (void)src; (void)n; return dst; }
char *strcat(char *dst, const char *src) { (void)src; return dst; }
char *strchr(const char *s, int c) { (void)s; (void)c; return 0; }
char *strrchr(const char *s, int c) { (void)s; (void)c; return 0; }
char *strstr(const char *haystack, const char *needle) { (void)haystack; (void)needle; return 0; }
char *strdup(const char *s) { (void)s; return 0; }

void *memcpy(void *dst, const void *src, unsigned long n) { (void)src; (void)n; return dst; }
void *memmove(void *dst, const void *src, unsigned long n) { (void)src; (void)n; return dst; }
void *memset(void *dst, int c, unsigned long n) { (void)c; (void)n; return dst; }
int memcmp(const void *a, const void *b, unsigned long n) { (void)a; (void)b; (void)n; return 0; }

void *mmap(void *addr, unsigned long len, int prot, int flags, int fd, long off) {
    (void)addr; (void)len; (void)prot; (void)flags; (void)fd; (void)off; return 0;
}
int munmap(void *addr, unsigned long len) { (void)addr; (void)len; return 0; }
