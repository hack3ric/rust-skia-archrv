use crate::prelude::*;
use skia_bindings::{C_SkString_c_str, C_SkString_destruct, C_SkString_size, SkString};
use std::{slice, str};

pub type String = Handle<SkString>;

impl NativeDrop for SkString {
    fn drop(&mut self) {
        unsafe {
            C_SkString_destruct(self);
        }
    }
}

impl ToString for String {
    fn to_string(&self) -> std::string::String {
        self.as_str().into()
    }
}

impl Default for Handle<SkString> {
    fn default() -> Self {
        Self::from_str("")
    }
}

impl Handle<SkString> {
    pub fn from_str(str: impl AsRef<str>) -> String {
        let bytes = str.as_ref().as_bytes();
        Handle::from_native(unsafe { SkString::new3(bytes.as_ptr() as _, bytes.len()) })
    }

    pub fn set(&mut self, string: &Self) {
        let bytes = string.as_str().as_bytes();
        unsafe {
            self.native_mut().set1(bytes.as_ptr() as _, bytes.len());
        }
    }

    pub fn as_str(&self) -> &str {
        let slice = unsafe {
            slice::from_raw_parts(
                C_SkString_c_str(self.native()) as _,
                C_SkString_size(self.native()),
            )
        };
        str::from_utf8(slice).unwrap()
    }
}

#[test]
fn string_from_rust_and_back() {
    let str = "Hello";
    let string = String::from_str(str);
    assert_eq!(str, string.as_str())
}

#[test]
fn set_string() {
    let mut hello = String::from_str("Hello");
    hello.set(&String::from_str("World"));
    assert_eq!("World", hello.as_str());
}
