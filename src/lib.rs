use walkdir::WalkDir;

pub mod reg_tool {
    use std::{ffi::OsStr, iter::once, os::windows::prelude::OsStrExt, ptr::null_mut};

    use winapi::{
        shared::{
            minwindef::{DWORD, HKEY},
            winerror::SEC_E_OK,
        },
        um::winreg::{RegOpenKeyW, RegQueryValueExW},
    };
    /// Open the registry table
    pub fn reg_open(main_hkey: HKEY, sub_key: &str) -> Result<HKEY, String> {
        unsafe {
            let mut hkey: HKEY = null_mut();
            let status = RegOpenKeyW(main_hkey, str_to_lpcwstr(sub_key).as_ptr(), &mut hkey);
            if status == SEC_E_OK {
                return Result::Ok(hkey);
            }
            return Result::Err(format!("status == {}", status));
        }
    }

    unsafe fn str_to_lpcwstr(str: &str) -> Vec<u16> {
        let result: Vec<u16> = OsStr::new(str).encode_wide().chain(once(0)).collect();
        return result;
    }

    /// Check the registry value
    pub fn reg_query_binary(hkey: &HKEY, key_name: &str) -> Vec<u8> {
        unsafe {
            let mut dword: DWORD = 0;
            let mut dtype: DWORD = 0;

            //查询
            let status = RegQueryValueExW(
                *hkey,
                str_to_lpcwstr(key_name).as_ptr(),
                null_mut(),
                &mut dtype,
                null_mut(),
                &mut dword,
            );

            let mut data_binary: Vec<u8> = vec![0; dword as usize];
            if status == SEC_E_OK {
                // 存在值

                RegQueryValueExW(
                    *hkey,
                    str_to_lpcwstr(key_name).as_ptr(),
                    null_mut(),
                    &mut dtype,
                    data_binary.as_mut_ptr(),
                    &mut dword,
                );
            }
            return data_binary;
        }
    }
}

pub fn ttf_finder(folder_path: &str) -> Vec<String> {
    let mut ttf_files: Vec<String> = Vec::new();
    
    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        if let Some(extension) = entry.path().extension() {
            if extension == "ttf" {
                let file_path = entry.path().to_path_buf();
                if let Some(file_path_str) = file_path.to_str() {
                    ttf_files.push(file_path_str.to_string());
                }
            }
        }
    }

    ttf_files
}
pub mod pack{
    pub struct PackConfig{
        
    }
    impl PackConfig{
        
    }
}