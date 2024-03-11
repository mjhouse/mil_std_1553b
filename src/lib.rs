// #![no_std]
#![forbid(
    arithmetic_overflow,
    absolute_paths_not_starting_with_crate,
    box_pointers,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    ffi_unwind_calls,
    keyword_idents,
    let_underscore_drop,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_results,
    unused_tuple_struct_fields,
    variant_size_differences
)]
#![doc = include_str!("../README.md")]

mod errors;
mod flags;
mod message;
mod word;

pub(crate) mod fields;

pub use crate::message::{Message, MessageDirection, MessageSide, MessageType, Packet};

pub use crate::errors::{Error, MessageError, Result, SubsystemError, SystemError, TerminalError};

pub use crate::word::{CommandWord, DataWord, StatusWord, Word, WordType};

pub use crate::flags::{
    Address, BroadcastReceived, DynamicBusAcceptance, Instrumentation, ModeCode, Reserved,
    ServiceRequest, SubAddress, TerminalBusy, TransmitReceive,
};
