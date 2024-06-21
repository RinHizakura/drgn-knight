CC = gcc
CFLAGS = -Wall -Wextra -I../drgn/libdrgn
LDFLAGS=-ldw -lelf -fopenmp

OUT ?= build
BIN = $(OUT)/load_debug_info
LIB=$(OUT)/libdrgnimpl.a
SHELL_HACK := $(shell mkdir -p $(OUT))

C_FILES = $(wildcard ./*.c)

CSRCS = $(shell find . -name '*.c')
_COBJ =  $(notdir $(CSRCS))
COBJ = $(_COBJ:%.c=$(OUT)/%.o)

vpath %.c $(sort $(dir $(CSRCS)))

all: $(BIN)

$(BIN): $(LIB) $(COBJ)
	@echo "  LD\t$@"
	@$(CC) -o $@ $(COBJ) $(LIB) $(LDFLAGS)

$(OUT)/%.o: %.c
	@echo "  CC\t$@"
	@$(CC) -c $(CFLAGS) $< -o $@

$(OUT)/%.o: ../%.c
	@echo "  CC\t$@"
	@$(CC) -c $(CFLAGS) $< -o $@

# FIXME: Improve the flow to get the build taget
$(LIB):
	cd ../drgn; python3 setup.py build
	cp ../drgn/build/temp.linux-x86_64-3.10/.libs/libdrgnimpl.a $@

clean:
	@$(RM) $(BIN) $(COBJ) $(OUT)/*.d

-include $(OUT)/*.d
