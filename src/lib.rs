//! A library to interface with libatasmart-sys.  For more information about libatasmart-sys see
//! [libatasmart-sys](https://github.com/cholcombe973/libatasmart-sys)
//! This library is useful for gathering ata smart information from your hard drives concerning
//! their remaining lifetime.  The underlying libatasmart doesn't expose every possible metric like
//! smartmontools but it does expose a few important ones like bad sector count and overall status.
//! This also has the advantage of avoiding CLI calls and scraping the text output which makes it
//! more reliable and also a lot more performant!
//!

extern crate libatasmart_sys;
extern crate nix;

use libatasmart_sys::*;
use nix::errno::Errno;
use std::ffi::CString;
use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests{
    use std::path::Path;
    use super::*;

    #[test]
    fn test_smart(){
        let mut disk = Disk::new(Path::new("/dev/sda")).unwrap();
        let ret = disk.get_smart_status();
        println!("Smart status: {:?}", ret);
        println!("Dumping disk stats");
        let ret = disk.dump();
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

    /// Returns a u64 representing the size of the disk in bytes.
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

    /// Returns a bool of true if sleep mode is supported, false otherwise.
    pub fn check_sleep_mode(&mut self) -> Result<bool,String> {
        unsafe{
            let mut mode: SkBool = 0;
            let ret = sk_disk_check_sleep_mode(self.skdisk, &mut mode);
            if ret < 0{
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail).desc().to_string());
            }
            if mode == 0{
                Ok(false)
            }else{
                Ok(true)
            }
        }
    }

    /// Returns a u64 representing the power on time in milliseconds
    pub fn get_power_on(&mut self) -> Result<u64, String>{
        unsafe{
            let mut power_on_time:u64 = 0;
            let ret = sk_disk_smart_get_power_on(self.skdisk, &mut power_on_time);
            if ret < 0{
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail).desc().to_string());
            }
            Ok(power_on_time)
        }
    }

    /// Returns a u64 representing the number of power on cycles
    pub fn get_power_cycle_count(&mut self) -> Result<u64,String> {
        unsafe{
            let mut power_cycle_count:u64 = 0;
            let ret = sk_disk_smart_get_power_cycle(self.skdisk, &mut power_cycle_count);
            if ret < 0{
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail).desc().to_string());
            }
            Ok(power_cycle_count)
        }
    }

    /// Returns a u64 representing the number of bad sections on the disk
    pub fn get_bad_sectors(&mut self) -> Result<u64,String> {
        unsafe{
            let mut bad_sector_count: u64 = 0;
            let ret = sk_disk_smart_get_bad(self.skdisk, &mut bad_sector_count);
            if ret < 0{
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail).desc().to_string());
            }
            Ok(bad_sector_count)
        }
    }

    /// Returns a u64 representing the mkelvin of the disk
    pub fn get_temperature(&mut self) -> Result<u64,String> {
        unsafe{
            let mut mkelvin: u64 = 0;
            let ret = sk_disk_smart_get_temperature(self.skdisk, &mut mkelvin);
            if ret < 0{
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail).desc().to_string());
            }
            Ok(mkelvin)
        }
    }

    /// Returns true if the disk passed smart, false otherwise.
    pub fn get_smart_status(&mut self) -> Result<bool,String> {
        unsafe{
            let mut good: SkBool = 0;
            let ret = sk_disk_smart_status(self.skdisk, &mut good);
            if ret < 0{
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail).desc().to_string());
            }
            if good == 0{
                Ok(false)
            }else{
                Ok(true)
            }
        }
    }

    /// This will dump all available information to stdout about the drive
    pub fn dump(&mut self)->Result<(), String>{
        unsafe{
            let ret = sk_disk_dump(self.skdisk);
            if ret < 0{
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail).desc().to_string());
            }
            Ok(())
        }
    }

    pub fn identify_is_available(&mut self)->Result<bool, String>{
        unsafe{
            let mut available: SkBool = 0;
            let ret = sk_disk_identify_is_available(self.skdisk, &mut available);
            if ret < 0{
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail).desc().to_string());
            }
            if available == 0{
                Ok(false)
            }else{
                Ok(true)
            }
        }
    }

    pub fn smart_is_available(&mut self)->Result<bool, String>{
        unsafe{
            let mut available: SkBool = 0;
            let ret = sk_disk_smart_is_available(self.skdisk, &mut available);
            if ret < 0{
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail).desc().to_string());
            }
            if available == 0{
                Ok(false)
            }else{
                Ok(true)
            }
        }
    }

    pub fn execute_smart_self_test(&mut self, test_type: SkSmartSelfTest)->Result<(), String>{
        unsafe{
            let ret = sk_disk_smart_self_test(self.skdisk, test_type);
            if ret < 0{
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail).desc().to_string());
            }
            Ok(())
        }
    }

    pub fn smart_get_overall(&mut self)->Result<SkSmartOverall, String>{
        unsafe{
            let mut overall: SkSmartOverall = SkSmartOverall::SK_SMART_OVERALL_GOOD;
            let ret = sk_disk_smart_get_overall(self.skdisk, &mut overall);
            if ret < 0{
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail).desc().to_string());
            }
            Ok(overall)
        }
    }
}

/*
pub fn sk_smart_self_test_execution_status_to_string(status: SkSmartSelfTestExecutionStatus) -> *const ::libc::c_char;
pub fn sk_smart_offline_data_collection_status_to_string(status: SkSmartOfflineDataCollectionStatus) -> *const ::libc::c_char;
pub fn sk_smart_self_test_to_string(test: SkSmartSelfTest) -> *const ::libc::c_char;
pub fn sk_smart_self_test_polling_minutes(d: *const SkSmartParsedData, test: SkSmartSelfTest) -> uint32_t;

pub fn sk_smart_self_test_available(d: *const SkSmartParsedData, test: SkSmartSelfTest) -> SkBool;
pub fn sk_disk_identify_parse(d: *mut *mut SkDisk, data: *const SkIdentifyParsedData) -> ::libc::c_int;
pub fn sk_disk_smart_read_data(d: *mut SkDisk) -> ::libc::c_int;
pub fn sk_disk_get_blob(d: *mut *mut SkDisk, blob: *const ::libc::c_void, size: *mut size_t) -> ::libc::c_int;
pub fn sk_disk_set_blob(d: *mut SkDisk, blob: *const ::libc::c_void, size: size_t) -> ::libc::c_int;
pub fn sk_disk_smart_parse(d: *mut *mut SkDisk, data: *const SkSmartParsedData) -> ::libc::c_int;
*/
