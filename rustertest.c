
#include <erl_nif.h>

ERL_NIF_TERM native_add(ErlNifEnv* env, int argc, const ERL_NIF_TERM argv[]);

static ErlNifFunc nif_funcs[] = {
    {"native_add", 2, native_add}
};

ERL_NIF_INIT(rustertest, nif_funcs, NULL, NULL, NULL, NULL)
