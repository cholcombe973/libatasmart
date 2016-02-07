extern crate libatasmart_sys;
extern crate nix;

use libatasmart_sys::*;
use nix::errno::Errno;
use std::ffi::CString;
use std::path::{Path, PathBuf};

#[test]
fn test_smart(){
    let device = CString::new("/dev/sda").unwrap();
    let mut disk: *mut SkDisk = unsafe { std::mem::uninitialized() };
    let mut good: SkBool = 0;
    unsafe{
        //NOTE: Requires root privs
        println!("opening disk");
        let ret = libatasmart_sys::sk_disk_open(device.as_ptr(), &mut disk);
        if ret < 0{
            let fail = nix::errno::errno();
            println!("sk_disk_open failed with error: {:?}", Errno::from_i32(fail));
        }
        println!("Success opening disk");
        let ret = libatasmart_sys::sk_disk_smart_status(disk, &mut good);
        if ret < 0{
            let fail = nix::errno::errno();
            println!("sk_disk_open failed with error: {:?}", Errno::from_i32(fail));
        }
        println!("Success getting smart status of {:?}", good);
        println!("Dumping disk");
        let ret = libatasmart_sys::sk_disk_dump(disk);
        if ret < 0{
            let fail = nix::errno::errno();
            println!("sk_disk_dump failed with error: {:?}", Errno::from_i32(fail));
        }
    }
}

pub struct Disk{
    pub disk: PathBuf,
    skdisk: *mut SkDisk,
}

impl Disk{
    pub fn new(disk_path: &Path) -> Result<Disk, String>{

        let device = CString::new(disk_path.to_str().unwrap()).unwrap();
        let mut disk: *mut SkDisk = unsafe { std::mem::uninitialized() };

        unsafe{
            let ret = libatasmart_sys::sk_disk_open(device.as_ptr(), &mut disk);
            if ret < 0{
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail).desc().to_string());
            }

            Ok(Disk{
                disk: disk_path.to_path_buf(),
                skdisk: disk,
            })
        }
    }
    fn drop(&mut self){
        unsafe{
            sk_disk_free(self.skdisk);
        }
    }

    pub fn get_disk_size(&mut self)->Result<u64, String>{
        unsafe{
            let mut bytes: u64 = 0;
            let ret = sk_disk_get_size(self.skdisk, &mut bytes);
            if ret < 0{
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail).desc().to_string());
            }
            return Ok(bytes);
        }
    }

}

/*
pub fn sk_smart_self_test_execution_status_to_string(status: SkSmartSelfTestExecutionStatus) -> *const ::libc::c_char;
pub fn sk_smart_offline_data_collection_status_to_string(status: SkSmartOfflineDataCollectionStatus) -> *const ::libc::c_char;
pub fn sk_smart_self_test_to_string(test: SkSmartSelfTest) -> *const ::libc::c_char;
pub fn sk_smart_self_test_polling_minutes(d: *const SkSmartParsedData, test: SkSmartSelfTest) -> uint32_t;
pub fn sk_smart_attribute_unit_to_string(unit: SkSmartAttributeUnit) -> *const ::libc::c_char;
pub fn sk_smart_overall_to_string(overall: SkSmartOverall) -> *const ::libc::c_char;
pub fn sk_disk_open(name: *const ::libc::c_char, d: *mut *mut SkDisk) -> ::libc::c_int;
pub fn sk_disk_get_size(d: *mut SkDisk, bytes: *mut uint64_t) -> ::libc::c_int;

pub fn sk_disk_check_sleep_mode(d: *mut SkDisk, awake: *mut SkBool) -> ::libc::c_int;
pub fn sk_disk_identify_is_available(d: *mut SkDisk, available: *mut SkBool) -> ::libc::c_int;
pub fn sk_disk_identify_parse(d: *mut *mut SkDisk, data: *const SkIdentifyParsedData) -> ::libc::c_int;
pub fn sk_disk_smart_is_available(d: *mut SkDisk, available: *mut SkBool) -> ::libc::c_int;
pub fn sk_disk_smart_status(d: *mut SkDisk, good: *mut SkBool) -> ::libc::c_int;
pub fn sk_disk_smart_read_data(d: *mut SkDisk) -> ::libc::c_int;
pub fn sk_disk_get_blob(d: *mut *mut SkDisk, blob: *const ::libc::c_void, size: *mut size_t) -> ::libc::c_int;
pub fn sk_disk_set_blob(d: *mut SkDisk, blob: *const ::libc::c_void, size: size_t) -> ::libc::c_int;
pub fn sk_disk_smart_parse(d: *mut *mut SkDisk, data: *const SkSmartParsedData) -> ::libc::c_int;
pub fn sk_disk_smart_self_test(d: *mut SkDisk, test: SkSmartSelfTest) -> ::libc::c_int;
pub fn sk_disk_smart_get_power_on(d: *mut SkDisk, mseconds: *mut uint64_t) -> ::libc::c_int;
pub fn sk_disk_smart_get_power_cycle(d: *mut SkDisk, count: *mut uint64_t) -> ::libc::c_int;
pub fn sk_disk_smart_get_bad(d: *mut SkDisk, sectors: *mut uint64_t) -> ::libc::c_int;
pub fn sk_disk_smart_get_temperature(d: *mut SkDisk, mkelvin: *mut uint64_t ) -> ::libc::c_int;
pub fn sk_disk_smart_get_overall(d: *mut SkDisk, overall: *mut SkSmartOverall) -> ::libc::c_int;
pub fn sk_disk_dump(d: *mut SkDisk) -> ::libc::c_int;
pub fn sk_disk_free(d: *mut SkDisk) -> ::libc::c_void;
pub fn sk_smart_self_test_execution_status_to_string(status: SkSmartSelfTestExecutionStatus) -> *const ::libc::c_char;
pub fn sk_smart_offline_data_collection_status_to_string(status: SkSmartOfflineDataCollectionStatus) -> *const ::libc::c_char;
pub fn sk_smart_self_test_to_string(test: SkSmartSelfTest) -> *const ::libc::c_char;
pub fn sk_smart_self_test_available(d: *const SkSmartParsedData, test: SkSmartSelfTest) -> SkBool;
pub fn sk_smart_self_test_polling_minutes(d: *const SkSmartParsedData, test: SkSmartSelfTest) -> uint32_t;
pub fn sk_smart_attribute_unit_to_string(unit: SkSmartAttributeUnit) -> *const ::libc::c_char;
pub fn sk_smart_overall_to_string(overall: SkSmartOverall) -> *const ::libc::c_char;
pub fn sk_disk_open(name: *const ::libc::c_char, d: *mut *mut SkDisk) -> ::libc::c_int;
pub fn sk_disk_get_size(d: *mut SkDisk, bytes: *mut uint64_t) -> ::libc::c_int;
pub fn sk_disk_check_sleep_mode(d: *mut SkDisk, awake: *mut SkBool) -> ::libc::c_int;
pub fn sk_disk_identify_is_available(d: *mut SkDisk, available: *mut SkBool) -> ::libc::c_int;
pub fn sk_disk_identify_parse(d: *mut *mut SkDisk, data: *const SkIdentifyParsedData) -> ::libc::c_int;
pub fn sk_disk_smart_is_available(d: *mut SkDisk, available: *mut SkBool) -> ::libc::c_int;
pub fn sk_disk_smart_status(d: *mut SkDisk, good: *mut SkBool) -> ::libc::c_int;
pub fn sk_disk_smart_read_data(d: *mut SkDisk) -> ::libc::c_int;
pub fn sk_disk_get_blob(d: *mut *mut SkDisk, blob: *const ::libc::c_void, size: *mut size_t) -> ::libc::c_int;
pub fn sk_disk_set_blob(d: *mut SkDisk, blob: *const ::libc::c_void, size: size_t) -> ::libc::c_int;
pub fn sk_disk_smart_parse(d: *mut *mut SkDisk, data: *const SkSmartParsedData) -> ::libc::c_int;
pub fn sk_disk_smart_self_test(d: *mut SkDisk, test: SkSmartSelfTest) -> ::libc::c_int;
pub fn sk_disk_smart_get_power_on(d: *mut SkDisk, mseconds: *mut uint64_t) -> ::libc::c_int;
pub fn sk_disk_smart_get_power_cycle(d: *mut SkDisk, count: *mut uint64_t) -> ::libc::c_int;
pub fn sk_disk_smart_get_bad(d: *mut SkDisk, sectors: *mut uint64_t) -> ::libc::c_int;
pub fn sk_disk_smart_get_temperature(d: *mut SkDisk, mkelvin: *mut uint64_t ) -> ::libc::c_int;
pub fn sk_disk_smart_get_overall(d: *mut SkDisk, overall: *mut SkSmartOverall) -> ::libc::c_int;
pub fn sk_disk_dump(d: *mut SkDisk) -> ::libc::c_int;
pub fn sk_disk_free(d: *mut SkDisk) -> ::libc::c_void;
*/
