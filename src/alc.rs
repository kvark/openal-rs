use core::libc::*;

// TODO: not sure what types these are meant to be...
pub static ALC_INVALID           : c_int = 0;
pub static ALC_VERSION_0_1       : c_int = 1;

pub struct ALCdevice {}
pub struct ALCcontext {}

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

pub static ALC_FALSE                                : ALCenum = 0;
pub static ALC_TRUE                                 : ALCenum = 1;
pub static ALC_FREQUENCY                            : ALCenum = 0x1007;
pub static ALC_REFRESH                              : ALCenum = 0x1008;
pub static ALC_SYNC                                 : ALCenum = 0x1009;
pub static ALC_MONO_SOURCES                         : ALCenum = 0x1010;
pub static ALC_STEREO_SOURCES                       : ALCenum = 0x1011;

pub static ALC_NO_ERROR                             : ALCenum = ALC_FALSE;
pub static ALC_INVALID_DEVICE                       : ALCenum = 0xA001;
pub static ALC_INVALID_CONTEXT                      : ALCenum = 0xA002;
pub static ALC_INVALID_ENUM                         : ALCenum = 0xA003;
pub static ALC_INVALID_VALUE                        : ALCenum = 0xA004;
pub static ALC_OUT_OF_MEMORY                        : ALCenum = 0xA005;

pub static ALC_DEFAULT_DEVICE_SPECIFIER             : ALCenum = 0x1004;
pub static ALC_DEVICE_SPECIFIER                     : ALCenum = 0x1005;
pub static ALC_EXTENSIONS                           : ALCenum = 0x1006;
pub static ALC_MAJOR_VERSION                        : ALCenum = 0x1000;
pub static ALC_MINOR_VERSION                        : ALCenum = 0x1001;
pub static ALC_ATTRIBUTES_SIZE                      : ALCenum = 0x1002;
pub static ALC_ALL_ATTRIBUTES                       : ALCenum = 0x1003;
pub static ALC_DEFAULT_ALL_DEVICES_SPECIFIER        : ALCenum = 0x1012;
pub static ALC_ALL_DEVICES_SPECIFIER                : ALCenum = 0x1013;
pub static ALC_CAPTURE_DEVICE_SPECIFIER             : ALCenum = 0x310;
pub static ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER     : ALCenum = 0x311;
pub static ALC_CAPTURE_SAMPLES                      : ALCenum = 0x312;

pub extern "C" {
    pub fn alcCreateContext(device: *ALCdevice, attrlist: *ALCint) -> *ALCcontext;
    pub fn alcMakeContextCurrent(context: *ALCcontext) -> ALCboolean;
    pub fn alcProcessContext(context: *ALCcontext);
    pub fn alcSuspendContext(context: *ALCcontext);
    pub fn alcDestroyContext(context: *ALCcontext);
    pub fn alcGetCurrentContext() -> *ALCcontext;
    pub fn alcGetContextsDevice(context: *ALCcontext) -> *ALCdevice;
    pub fn alcOpenDevice(devicename: *ALCchar) -> *ALCdevice;
    pub fn alcCloseDevice(device: *ALCdevice) -> ALCboolean;
    pub fn alcGetError(device: *ALCdevice) -> ALCenum;
    pub fn alcIsExtensionPresent(device: *ALCdevice, extname: *ALCchar) -> ALCboolean;
    pub fn alcGetProcAddress(device: *ALCdevice, funcname: *ALCchar) -> *c_void;
    pub fn alcGetEnumValue(device: *ALCdevice, enumname: *ALCchar) -> ALCenum;
    pub fn alcGetString(device: *ALCdevice, param: ALCenum) -> *ALCchar;
    pub fn alcGetIntegerv(device: *ALCdevice, param: ALCenum, size: ALCsizei, data: *ALCint);
    pub fn alcCaptureOpenDevice(devicename: *ALCchar, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> *ALCdevice;
    pub fn alcCaptureCloseDevice(device: *ALCdevice) -> ALCboolean;
    pub fn alcCaptureStart(device: *ALCdevice);
    pub fn alcCaptureStop(device: *ALCdevice);
    pub fn alcCaptureSamples(device: *ALCdevice, buffer: ALCvoid, samples: ALCsizei);
}
