BIN := arc
SRC := arc.rs

RUSTC := rustc
RUSTFLAGS := -g -C opt-level=0 -C debuginfo=2

.PHONY: all run gdb clean

all: $(BIN)

$(BIN): $(SRC)
	$(RUSTC) $(RUSTFLAGS) $< -o $@

run: $(BIN)
	./$(BIN)

gdb: $(BIN)
	rust-gdb ./$(BIN)

clean:
	rm -f $(BIN)
