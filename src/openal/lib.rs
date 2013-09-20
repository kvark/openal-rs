#[link(name = "openal",
       vers = "0.1",
       uuid = "9450DF9F-7A40-4087-BF6C-0848693D15DC",
       author = "Brendan Zabarauskas",
       url = "https://github.com/bjz/openal-rs")];

#[comment = "OpenAL 1.1 bindings for Rust."];
#[crate_type = "lib"];

pub mod ll;
pub mod al;
pub mod alc;

/// Core OpenAL typedefs
pub mod types {
    use std::libc::*;

    pub type ALboolean              = c_char;
    pub type ALchar                 = c_char;
    pub type ALbyte                 = c_char;
    pub type ALubyte                = c_uchar;
    pub type ALshort                = c_short;
    pub type ALushort               = c_ushort;
    pub type ALint                  = c_int;
    pub type ALuint                 = c_uint;
    pub type ALsizei                = c_int;
    pub type ALenum                 = c_int;
    pub type ALfloat                = c_float;
    pub type ALdouble               = c_double;
    pub type ALvoid                 = c_void;

    pub struct ALCdevice;
    pub struct ALCcontext;

    pub type ALCboolean             = c_char;
    pub type ALCchar                = c_char;
    pub type ALCbyte                = c_char;
    pub type ALCubyte               = c_uchar;
    pub type ALCshort               = c_short;
    pub type ALCushort              = c_ushort;
    pub type ALCint                 = c_int;
    pub type ALCuint                = c_uint;
    pub type ALCsizei               = c_int;
    pub type ALCenum                = c_int;
    pub type ALCfloat               = c_float;
    pub type ALCdouble              = c_double;
    pub type ALCvoid                = c_void;
}