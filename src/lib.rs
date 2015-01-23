extern crate ruster;
extern crate core;

use ruster::raw::*;

use core::marker::Sync;

// #[no_mangle]
// pub extern "C" fn native_add() -> ERL_NIF_TERM
// {
//     //let x = unsafe {enif_make_badarg(env)};
 
//     let x:c_int = 0; //ERL_NIF_TERM(0);
//     x 

// }

#[no_mangle]
pub extern "C" fn native_add(env: *mut ErlNifEnv,
                             argc: c_int,
                             args: *const ERL_NIF_TERM) -> ERL_NIF_TERM
{

    if argc != 2
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
}



// pub extern "C" fn init() -> *const MyStruct<'static> {
//     static ENTRY: MyStruct<'static> =
//         MyStruct { name: "hello\0" };
//     &ENTRY
// }


// // this is ok
// static raw_string: *const str = "why!!!\0";


// pub struct MyStruct {
//     pub name: *const str,
// }

// unsafe impl Sync for MyStruct {}

// static mystruct: MyStruct = MyStruct {name: "why!!!\0"};

// src/lib.rs:52:29: 52:56 error: the trait `core::marker::Sync` is not implemented for the type `*const str`
// src/lib.rs:52 static mystruct: MyStruct = MyStruct {name: "why!!!\0"};
//                                           ^~~~~~~~~~~~~~~~~~~~~~~~~~~





// struct MyStruct<'a> {
//     name: (*const)'static str,
// }
// static mystruct: MyStruct<'static> = MyStruct {name: "why!!!\0"};


// static mytest: ErlNifFunc = ErlNifFunc {
//     name:     "native_add\0",
//     arity:    2,
//     function: native_add,
//     flags:    0,
// };


static FUNCS: [ErlNifFunc; 1] = [
    ErlNifFunc{ name:     "native_add\0",
                arity:    2,
                function: native_add,
                flags:    0,
              }
];

#[no_mangle]
pub extern "C" fn nif_init() -> *const ErlNifEntry {
	static ENTRY: ErlNifEntry = ErlNifEntry{
		major : NIF_MAJOR_VERSION,
		minor : NIF_MINOR_VERSION,
		name : "rustnif\0",
		num_of_funcs : 1,
		funcs : &FUNCS,
		load :    None,
		reload :  None,
		upgrade : None,
		unload :  None,
		vm_variant : "beam.vanilla\0",
        options: 0,
	};

    &ENTRY
}
