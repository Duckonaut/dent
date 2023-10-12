BINDIR=$(CURDIR)/bin

CFLAGS = -Wall -Wextra -Werror -pedantic -std=c99 -g -I../include
LDLIBS = -lcdent
LDFLAGS = -L$(CURDIR)/../../target/debug

CFILES = $(wildcard *.c)
NAMES = $(CFILES:.c=)
# final executable names
# eg. simple.c becomes bin/simple
BINNAMES = $(addprefix $(BINDIR)/,$(NAMES))

.PHONY: all clean

all: $(BINNAMES)
	@echo "Done."

clean:
	$(RM) $(BINDIR)/*

$(BINDIR)/%: %.c
	@echo "Compiling $@"
	@mkdir -p $(BINDIR)
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $< $(LDLIBS)
