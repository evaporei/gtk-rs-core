// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Cancellable;
use crate::OutputStream;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GPollableOutputStream")]
    pub struct PollableOutputStream(Interface<ffi::GPollableOutputStream, ffi::GPollableOutputStreamInterface>) @requires OutputStream;

    match fn {
        type_ => || ffi::g_pollable_output_stream_get_type(),
    }
}

impl PollableOutputStream {
    pub const NONE: Option<&'static PollableOutputStream> = None;
}

pub trait PollableOutputStreamExt: 'static {
    #[doc(alias = "g_pollable_output_stream_can_poll")]
    fn can_poll(&self) -> bool;

    #[doc(alias = "g_pollable_output_stream_is_writable")]
    fn is_writable(&self) -> bool;

    #[doc(alias = "g_pollable_output_stream_write_nonblocking")]
    fn write_nonblocking(
        &self,
        buffer: &[u8],
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<isize, glib::Error>;

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //#[doc(alias = "g_pollable_output_stream_writev_nonblocking")]
    //fn writev_nonblocking(&self, vectors: /*Ignored*/&[OutputVector], cancellable: Option<&impl IsA<Cancellable>>) -> Result<(/*Ignored*/PollableReturn, usize), glib::Error>;
}

impl<O: IsA<PollableOutputStream>> PollableOutputStreamExt for O {
    fn can_poll(&self) -> bool {
        unsafe {
            from_glib(ffi::g_pollable_output_stream_can_poll(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_writable(&self) -> bool {
        unsafe {
            from_glib(ffi::g_pollable_output_stream_is_writable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn write_nonblocking(
        &self,
        buffer: &[u8],
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<isize, glib::Error> {
        let count = buffer.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_pollable_output_stream_write_nonblocking(
                self.as_ref().to_glib_none().0,
                buffer.to_glib_none().0,
                count,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //fn writev_nonblocking(&self, vectors: /*Ignored*/&[OutputVector], cancellable: Option<&impl IsA<Cancellable>>) -> Result<(/*Ignored*/PollableReturn, usize), glib::Error> {
    //    unsafe { TODO: call ffi:g_pollable_output_stream_writev_nonblocking() }
    //}
}

impl fmt::Display for PollableOutputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PollableOutputStream")
    }
}
