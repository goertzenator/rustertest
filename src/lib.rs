extern crate ruster;
extern crate core;

use ruster::raw::*;
use ruster::rnif::*;
use std::mem::transmute;

#[no_mangle]
pub extern "C" fn native_add(env: *mut ErlNifEnv,
                          argc: c_int,
                          args: *const ERL_NIF_TERM) -> ERL_NIF_TERM
{
    unsafe {
         transmute(native_add_wrapped(&*env, std::slice::from_raw_buf(transmute(&args), argc as usize)))
    }
}


//fn native_add_wrapped(env: & Environment, args: &[Term]) -> Term
fn native_add_wrapped<'a>(env: &'a Environment, args: &[Term<'a>]) -> Term<'a>
{
    make_int(env, 123)
}


// #[no_mangle]
// pub extern "C" fn native_add(env: *mut ErlNifEnv,
//                              argc: c_int,
//                              args: *const ERL_NIF_TERM) -> ERL_NIF_TERM
// {

//     if argc != 2
//         { unsafe { return enif_make_badarg(env); } }
//     let mut a: c_int = 0;
//     let mut b: c_int = 0;
//     unsafe {
//         if enif_get_int(env, *args.offset(0), &mut a) == 0
//             { return enif_make_badarg(env); }
//         if enif_get_int(env, *args.offset(1), &mut b) == 0
//             { return enif_make_badarg(env); }
//         enif_make_int(env, a + b + 10)
//     }
// }






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


//const FUNCS: [ErlNifFunc; 1] = [
// const FUNCS: [ErlNifFunc] = &[
//     ErlNifFunc{ name:     &b"native_add\0"[0],
//                 arity:    2,
//                 function: native_add,
//                 flags:    0,
//               },
//     // ErlNifFunc{ name:     &"native_add\0".as_bytes()[0],
//     //             arity:    2,
//     //             function: native_add,
//     //             flags:    0,
//     //           },
// ];

// static SARRAY: [i32; 1] = [11];

// struct MyStruct {
//     pub arr: *const [i32],
// }
// unsafe impl Sync for MyStruct {}

// static mystruct: MyStruct = MyStruct {
//     arr: &SARRAY
// };


// const ENTRY: ErlNifEntry = ErlNifEntry{
//         major : NIF_MAJOR_VERSION,
//         minor : NIF_MINOR_VERSION,
//         name : &b"rustnif\0"[0],
//         num_of_funcs : 2,
//         funcs : &FUNCS[0],
//         load :    None,
//         reload :  None,
//         upgrade : None,
//         unload :  None,
//         vm_variant : &b"beam.vanilla\0"[0],
//         options: 0,
//     };

// #[no_mangle]
// pub extern "C" fn nif_init() -> *const ErlNifEntry {
//     &ENTRY
// }


// use std::mem::size_of_val;

// #[no_mangle]
// pub extern "C" fn info() {
//     println!("major {}",        size_of_val(&ENTRY.major));
//     println!("minor {}",        size_of_val(&ENTRY.minor));
//     println!("name {}",         size_of_val(&ENTRY.name));
//     println!("num_of_funcs {}", size_of_val(&ENTRY.num_of_funcs));
//     println!("funcs {}",        size_of_val(&ENTRY.funcs));
//     println!("load {}",         size_of_val(&ENTRY.load));
//     println!("reload {}",       size_of_val(&ENTRY.reload));
//     println!("upgrade {}",      size_of_val(&ENTRY.upgrade));
//     println!("unload {}",       size_of_val(&ENTRY.unload));
//     println!("vm_variant {}",   size_of_val(&ENTRY.vm_variant));
//     println!("options {}",      size_of_val(&ENTRY.options));

// }
