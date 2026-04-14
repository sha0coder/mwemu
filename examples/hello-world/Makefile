CC_MACARM64 = clang -target arm64-apple-darwin
CC_MACX64   = clang -target x86_64-apple-darwin
CC_LINUX86  = i686-unknown-linux-gnu-gcc
CC_LINUX64  = x86_64-unknown-linux-gnu-gcc
CC_LINARM64 = aarch64-unknown-linux-gnu-gcc
CC_WINX86   = i686-w64-mingw32-clang
CC_WINX64   = x86_64-w64-mingw32-clang
CC_WINARM64 = aarch64-w64-mingw32-clang

SRC = main.c
OUT = ../../crates/libmwemu/src/tests/fixtures

TARGETS = \
	$(OUT)/hello_mac_arm64 \
	$(OUT)/hello_mac_x64 \
	$(OUT)/hello_linux_x86 \
	$(OUT)/hello_linux_x64 \
	$(OUT)/hello_linux_arm64 \
	$(OUT)/hello_win_x86.exe \
	$(OUT)/hello_win_x64.exe \
	$(OUT)/hello_win_arm64.exe

all: $(TARGETS)

$(OUT)/hello_mac_arm64: $(SRC)
	$(CC_MACARM64) $(SRC) -o $@

$(OUT)/hello_mac_x64: $(SRC)
	$(CC_MACX64) $(SRC) -o $@

$(OUT)/hello_linux_x86: $(SRC)
	$(CC_LINUX86) $(SRC) -o $@

$(OUT)/hello_linux_x64: $(SRC)
	$(CC_LINUX64) $(SRC) -o $@

$(OUT)/hello_linux_arm64: $(SRC)
	$(CC_LINARM64) $(SRC) -o $@

$(OUT)/hello_win_x86.exe: $(SRC)
	$(CC_WINX86) $(SRC) -o $@

$(OUT)/hello_win_x64.exe: $(SRC)
	$(CC_WINX64) $(SRC) -o $@

$(OUT)/hello_win_arm64.exe: $(SRC)
	$(CC_WINARM64) $(SRC) -o $@

clean:
	rm -f $(TARGETS)

.PHONY: all clean
