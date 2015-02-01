
#include <erl_nif.h>
#include <stdio.h>



ERL_NIF_TERM enif_make_int(ErlNifEnv *env, int x)
{ return 0; }

int enif_get_int(ErlNifEnv *env, ERL_NIF_TERM term, int *x)
{ return 0; }

ERL_NIF_TERM enif_make_badarg(ErlNifEnv *env)
{ return 0; }

ErlNifEntry *nif_init();
void info();

int main()
{
	ErlNifEntry *entry;
	entry = nif_init();

	printf("major %d\n", entry->major);
	printf("minor %d\n", entry->minor);
	printf("name %s\n", entry->name);
	printf("num_of_funcs %d\n", entry->num_of_funcs);
	printf("funcs %p\n", entry->funcs);
	printf("load %p\n", entry->load);
	printf("reload %p\n", entry->reload);
	printf("upgrade %p\n", entry->upgrade);
	printf("unload %p\n", entry->unload);
	printf("vm_variant %s\n", entry->vm_variant);
	printf("options %d\n", entry->options);
	printf("\n");

	info();


	return 0;
}