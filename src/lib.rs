#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! c {
        ($str: expr) => {
            concat!($str, "\0").as_ptr() as *const std::os::raw::c_char
        };
    }

    #[test]
    fn test_bindings() -> Result<(), std::io::Error> {
        use std::fs::File;
        use std::io::prelude::*;

        let mut f = File::open("test/sample.yuv")?;
        let mut sample = Vec::new();
        f.read_to_end(&mut sample)?;

        unsafe {
            let api = &*x265_api_get_176(10);
            let param_ptr = api.param_alloc.unwrap()();
            let param = &mut *param_ptr;
            api.param_default_preset.unwrap()(param_ptr, c!("medium"), c!(""));
            param.internalCsp = X265_CSP_I420 as i32;
            param.internalBitDepth = 10;
            param.sourceWidth = 1920;
            param.sourceHeight = 1080;
            param.fpsNum = 24000;
            param.fpsDenom = 1001;
            let encoder = api.encoder_open.unwrap()(param_ptr);

            let pic_ptr = api.picture_alloc.unwrap()();
            api.picture_init.unwrap()(param_ptr, pic_ptr);
            let pic = &mut *pic_ptr;
            pic.stride[0] = 1080;
            pic.stride[1] = 1080 / 2;
            pic.stride[2] = 1080 / 2;

            let pp_nal: *mut *mut x265_nal = std::ptr::null_mut();
            let pi_nal: *mut u32 = std::ptr::null_mut();
            api.encoder_headers.unwrap()(encoder, pp_nal, pi_nal);

            api.encoder_encode.unwrap()(encoder, pp_nal, pi_nal, pic_ptr, std::ptr::null_mut());

            api.encoder_close.unwrap()(encoder);
            api.picture_free.unwrap()(pic_ptr);
            api.param_free.unwrap()(param_ptr);
            api.cleanup.unwrap()();
        }

        Ok(())
    }
}
