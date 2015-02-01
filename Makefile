
CFLAGS = -I $(ERTS_INCLUDE_DIR) -fpic
LDFLAGS = 

ERL = erl

.PHONY: all
all: rustertest.so

include env.mk

dumpentry: dumpentry.o
	cc -o $@ $< $(LDFLAGS)

.PHONY: librustertest.a
librustertest.a:
	rm -f $@
	cargo build
	ln -s target/librustertest*.a $@

rustertest.o: rustertest.c env.mk
	$(CC) -c $(CFLAGS) -o $@ $<

rustertest.so: rustertest.o librustertest.a
	$(CC) -shared -o $@ $^ $(LDFLAGS)

env.mk:
	$(ERL) -eval "file:write_file(\"$@\", \
		io_lib:format( \
			\"ERTS_INCLUDE_DIR ?= ~s/erts-~s/include/~n\" \
			\"ERL_INTERFACE_INCLUDE_DIR ?= ~s~n\" \
			\"ERL_INTERFACE_LIB_DIR ?= ~s~n\", \
			[code:root_dir(), erlang:system_info(version), \
			code:lib_dir(erl_interface, include), \
			code:lib_dir(erl_interface, lib)])), \
		halt()."
