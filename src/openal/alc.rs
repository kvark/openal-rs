// Copyright 2013 The openal-rs Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::str;
use std::vec;

use self::types::*;

pub mod types {
    use std::libc::*;
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

pub mod ffi {
    use super::types::*;

    // Boolean values
    pub static FALSE                                : ALCboolean = 0;
    pub static TRUE                                 : ALCboolean = 1;

    // Context management
    pub static FREQUENCY                            : ALCint = 0x1007;
    pub static REFRESH                              : ALCint = 0x1008;
    pub static SYNC                                 : ALCint = 0x1009;
    pub static MONO_SOURCES                         : ALCint = 0x1010;
    pub static STEREO_SOURCES                       : ALCint = 0x1011;

    // Errors
    pub static NO_ERROR                             : ALCenum = FALSE as ALCenum;
    pub static INVALID_DEVICE                       : ALCenum = 0xA001;
    pub static INVALID_CONTEXT                      : ALCenum = 0xA002;
    pub static INVALID_ENUM                         : ALCenum = 0xA003;
    pub static INVALID_VALUE                        : ALCenum = 0xA004;
    pub static OUT_OF_MEMORY                        : ALCenum = 0xA005;

    pub static DEFAULT_DEVICE_SPECIFIER             : ALCenum = 0x1004;
    pub static DEVICE_SPECIFIER                     : ALCenum = 0x1005;
    pub static EXTENSIONS                           : ALCenum = 0x1006;

    pub static MAJOR_VERSION                        : ALCenum = 0x1000;
    pub static MINOR_VERSION                        : ALCenum = 0x1001;

    pub static ATTRIBUTES_SIZE                      : ALCenum = 0x1002;
    pub static ALL_ATTRIBUTES                       : ALCenum = 0x1003;

    // ALC_ENUMERATE_ALL_EXT enums
    pub static DEFAULT_ALL_DEVICES_SPECIFIER        : ALCenum = 0x1012;
    pub static ALL_DEVICES_SPECIFIER                : ALCenum = 0x1013;

    // Capture extension
    pub static CAPTURE_DEVICE_SPECIFIER             : ALCenum = 0x310;
    pub static CAPTURE_DEFAULT_DEVICE_SPECIFIER     : ALCenum = 0x311;
    pub static CAPTURE_SAMPLES                      : ALCenum = 0x312;

    pub struct ALCdevice;
    pub struct ALCcontext;

    extern "C" {
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
        pub fn alcGetProcAddress(device: *ALCdevice, funcname: *ALCchar) -> Option<extern "C" fn()>;
        pub fn alcGetEnumValue(device: *ALCdevice, enumname: *ALCchar) -> ALCenum;
        pub fn alcGetString(device: *ALCdevice, param: ALCenum) -> *ALCchar;
        pub fn alcGetIntegerv(device: *ALCdevice, param: ALCenum, size: ALCsizei, data: *mut ALCint);
        pub fn alcCaptureOpenDevice(devicename: *ALCchar, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> *ALCdevice;
        pub fn alcCaptureCloseDevice(device: *ALCdevice) -> ALCboolean;
        pub fn alcCaptureStart(device: *ALCdevice);
        pub fn alcCaptureStop(device: *ALCdevice);
        pub fn alcCaptureSamples(device: *ALCdevice, buffer: *ALCvoid, samples: ALCsizei);
    }
}

pub struct Context {
    priv ptr: *ffi::ALCcontext,
}

// // pub fn get_current_context() -> Context {
//     Context { ptr: unsafe { ffi::alcGetCurrentContext() } }
// }

impl Context {
        pub fn make_current(&self) -> bool {
        unsafe { ffi::alcMakeContextCurrent(self.ptr) == ffi::TRUE }
    }

        pub fn process(&self) {
        unsafe { ffi::alcProcessContext(self.ptr); }
    }

        pub fn suspend(&self) {
        unsafe { ffi::alcSuspendContext(self.ptr); }
    }

        pub fn destroy(self) {}

    //     // pub fn get_device(&self) -> Device {
    //     Device { ptr: unsafe { ffi::alcGetContextsDevice(self.ptr) } }
    // }

        pub fn is_current(&self) -> bool {
        unsafe { ffi::alcGetCurrentContext() == self.ptr }
    }
}

impl Drop for Context {
        fn drop(&mut self) {
        unsafe { ffi::alcDestroyContext(self.ptr); }
    }
}

pub struct Device {
    priv ptr: *ffi::ALCdevice,
}

impl Device {
        pub fn open(devicename: &str) -> Option<Device> {
        let ptr = unsafe { devicename.with_c_str(|c_str| ffi::alcOpenDevice(c_str)) };
        if ptr.is_null() { None }
        else { Some(Device { ptr: ptr  }) }
    }

    /// Closes the device.
    ///
    /// The device will not be closed if it contains any contexts or buffers.
    /// If this is the case, the device will be returned again, wrapped in `Err`.
        pub fn close(self) -> Result<(), Device> {
        if unsafe { ffi::alcCloseDevice(self.ptr) == ffi::TRUE } { Ok(()) }
        else { Err(self) }
    }

        pub fn get_error(&self) -> ALCenum {
        unsafe { ffi::alcGetError(self.ptr) }
    }

        pub fn get_string(&self, param: ALCenum) -> ~str {
        unsafe { str::raw::from_c_str(ffi::alcGetString(self.ptr, param)) }
    }

    //     // pub fn GetIntegerv(&self, param: ALCenum, size: ALCsizei, data: *ALCint) {
    //     unsafe { ffi::alcGetIntegerv(); }
    // }

        pub fn create_context(&self, attr_list: &[ALCint]) -> Option<Context> {
        let attrs_terminated = vec::append_one(attr_list.to_owned(), 0);  // teminate attributes with a 0
        let ptr = unsafe { ffi::alcCreateContext(self.ptr, attrs_terminated.as_ptr()) };
        if ptr.is_null() { None }
        else { Some(Context { ptr: ptr  }) }
    }
}

pub struct CaptureDevice {
    priv ptr: *ffi::ALCdevice,
}

impl CaptureDevice {
        pub fn open(devicename: &str, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> Option<CaptureDevice> {
        let ptr = unsafe { devicename.with_c_str(|c_str| ffi::alcCaptureOpenDevice(c_str, frequency, format, buffersize)) };
        if ptr.is_null() { None }
        else { Some(CaptureDevice { ptr: ptr  }) }
    }

    /// Closes the capture device.
    ///
    /// If an error occurs, the device will be returned again, wrapped in `Err`.
        pub fn close(self) -> Result<(), CaptureDevice> {
        if unsafe { ffi::alcCaptureCloseDevice(self.ptr) == ffi::TRUE } { Ok(()) }
        else { Err(self) }
    }

        pub fn start(&self) {
        unsafe { ffi::alcCaptureStart(self.ptr); }
    }

        pub fn stop(&self) {
        unsafe { ffi::alcCaptureStop(self.ptr); }
    }

    //     // pub fn CaptureSamples(&self, buffer: *ALCvoid, samples: ALCsizei) {
    //     unsafe { ffi::alcCaptureSamples(); }
    // }
}
