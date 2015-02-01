-module(rustertest).


-export([native_add/2]).
-on_load(init/0).

init() ->
    ok = erlang:load_nif("./rustertest", 0).

native_add(_X, _Y) ->
    exit(nif_library_not_loaded).
