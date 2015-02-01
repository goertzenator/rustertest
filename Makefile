
CFLAGS = -I /usr/local/lib/erlang/erts-6.3/include/
LDFLAGS = -lrustertest -L.

dumpentry: dumpentry.o
	cc -o $@ $< $(LDFLAGS)
