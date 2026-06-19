use std::ffi::CString;

use anyhow::anyhow;

use crate::Result;

pub fn embed_assets(
    binary_path: &std::path::Path,
    assets_dir: &std::path::Path,
    output_path: &std::path::Path,
) -> Result<()> {
    log::debug!("embed_assets");
    unsafe {
        let binary_path_c = CString::new(binary_path.to_str().unwrap())?;
        let assets_dir_c = CString::new(assets_dir.to_str().unwrap())?;
        let output_path_c = CString::new(output_path.to_str().unwrap())?;

        let result = zig_embed_assets(
            binary_path_c.as_ptr(),
            assets_dir_c.as_ptr(),
            output_path_c.as_ptr(),
        );

        log::debug!("end of embed_assets");
        if result != 0 {
            Err(anyhow!("Failed to embed assets"))
        } else {
            Ok(())
        }
    }
}

pub fn generate_wrapper(
    binary_path: &std::path::Path,
    output_path: &std::path::Path,
    port: i32,
) -> Result<()> {
    log::debug!("generate_wrapper");
    unsafe {
        let binary_path_c = CString::new(binary_path.to_str().unwrap())?;
        let output_path_c = CString::new(output_path.to_str().unwrap())?;

        let result = zig_generate_wrapper(binary_path_c.as_ptr(), output_path_c.as_ptr(), port);

        log::debug!("end of generate_wrapper");
        if result != 0 {
            Err(anyhow!("Failed to generate wrapper"))
        } else {
            Ok(())
        }
    }
}

#[link(name = "binfuse_zig", kind = "static")]
unsafe extern "C" {
    pub fn zig_embed_assets(
        binary_path: *const std::os::raw::c_char,
        assets_dir: *const std::os::raw::c_char,
        output_path: *const std::os::raw::c_char,
    ) -> std::os::raw::c_int;

    pub fn zig_generate_wrapper(
        binary_path: *const std::os::raw::c_char,
        output_path: *const std::os::raw::c_char,
        port: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
}
