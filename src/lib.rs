extern crate ruster;

use ruster::raw::*;



#[no_mangle]
pub extern "C" fn native_add(env: *mut ErlNifEnv,
                             argc: c_int,
                             args: *const ERL_NIF_TERM) -> ERL_NIF_TERM
{
/*    if argc != 2
        { unsafe { return enif_make_badarg(env); } }
    let mut a: c_int = 0;
    let mut b: c_int = 0;
    unsafe {
        if enif_get_int(env, *args.offset(0), &mut a) == 0
            { return enif_make_badarg(env); }
        if enif_get_int(env, *args.offset(1), &mut b) == 0
            { return enif_make_badarg(env); }
        enif_make_int(env, a + b)
    }
    */
    ERL_NIF_TERM
}

static funcs: [ErlNifFunc] = [
    ErlNifFunc{ name:     "native_add\0",
                arity:    2,
                function: native_add }
];
/*
static mut nif_entry: ErlNifEntry = ErlNifEntry{
    major           : 2,
    minor           : 6,
    name            : 0 as *const c_char,
    num_of_funcs    : 0,
    funcs           : 0 as *mut ErlNifFunc,
    load            : None,
    reload          : None,
    upgrade         : None,
    unload          : None,
    vm_variant      : 0 as *const c_char
};


#[no_mangle]
pub extern "C" fn nif_init() -> *mut ErlNifEntry
{
    unsafe {
        funcs[0].name = CString::from_slice("native_add".as_bytes()).as_ptr();
        nif_entry.name = CString::from_slice("er".as_bytes()).as_ptr();
        nif_entry.num_of_funcs = funcs.len() as i32;
        nif_entry.funcs = funcs.as_mut_ptr();
        nif_entry.vm_variant = CString::from_slice("beam.vanilla".as_bytes()).as_ptr();
        &mut nif_entry
    }
}
*/




pub extern "C" fn nif_init() -> *mut ErlNifEntry
{
	static mut entry: ErlNifEntry = ErlNifEntry{
		major : NIF_MAJOR_VERSION,
		minor : NIF_MINOR_VERSION,
		name : "rustnif\0",
		num_of_funcs : 0,
		funcs : 0 as *mut ErlNifFunc,
		load :    None,
		reload :  None,
		upgrade : None,
		unload :  None,
		vm_variant : "beam.vanilla\0",
	};
	// unsafe {
	// 	funcs[0].name = CString::from_slice("native_add".as_bytes()).as_ptr();
	// 	entry.name = CString::from_slice("er".as_bytes()).as_ptr();
	// 	entry.num_of_funcs = funcs.len() as i32;
	// 	entry.funcs = funcs.as_mut_ptr();
	
	// }
	unsafe{ &mut entry }
}