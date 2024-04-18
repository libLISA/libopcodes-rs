#[allow(unused, non_snake_case, non_camel_case_types, non_upper_case_globals, deref_nullptr)]
pub mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

include!(concat!(env!("OUT_DIR"), "/init_disassemble_info_snippet.rs"));