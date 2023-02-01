//! A library to interface with libatasmart-sys.  For more information about libatasmart-sys see
//! [libatasmart-sys](https://github.com/cholcombe973/libatasmart-sys)
//! This library is useful for gathering ata smart information from your hard drives concerning
//! their remaining lifetime.  The underlying libatasmart doesn't expose every possible metric like
//! smartmontools but it does expose a few important ones like bad sector count and overall status.
//! This also has the advantage of avoiding CLI calls and scraping the text output which makes it
//! more reliable and also a lot more performant!
//!

use libatasmart_sys::*;
use nix::{errno::Errno, libc::user};
use std::{ffi::{CString, CStr, c_void}, path::{Path, PathBuf}, mem::MaybeUninit, ptr::null};
pub use libatasmart_sys::SkSmartSelfTest;
pub extern crate nix;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_new_failure() {
        match Disk::new(Path::new("/dev/null")) {
            Ok(_) => panic!("Opening /dev/null succeeded"),
            Err(e) => assert_eq!(Errno::ENODEV, e),
        }
    }
    /*
    #[test]
    fn test_smart(){
        let mut disk = Disk::new(Path::new("/dev/sda")).unwrap();
        let ret = disk.get_smart_status();
        println!("Smart status: {:?}", ret);
        println!("Dumping disk stats");
        let ret = disk.dump();
    }
    */
}

/// Our ata smart disk
pub struct Disk {
    /// The path in the filesystem to the hard drive
    pub disk: PathBuf,
    skdisk: *mut SkDisk,
}

#[derive(Debug)]
pub struct IdentifyParsedData{
    pub serial: String,
    pub firmware: String,
    pub model: String,
}

impl Disk {
    /// This will initialize a new Disk by asking libatasmart to open it.
    /// Note that this requires root permissions usually to succeed.
    pub fn new(disk_path: &Path) -> Result<Disk, Errno> {
        let device = CString::new(disk_path.to_str().unwrap()).unwrap();
        let mut disk = MaybeUninit::<SkDisk>::uninit().as_mut_ptr();

        unsafe {
            let ret = libatasmart_sys::sk_disk_open(device.as_ptr(), &mut disk);
            if ret < 0 {
                // Do not call sk_disk_free here, sk_disk_open already did that.
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail));
            }

            let ret = libatasmart_sys::sk_disk_smart_read_data(disk);
            if ret < 0 {
                let fail = nix::errno::errno();
                sk_disk_free(disk);
                return Err(Errno::from_i32(fail));
            }
            Ok(Disk {
                disk: disk_path.to_path_buf(),
                skdisk: disk,
            })
        }
    }

    /// Refreshes cached SMART attribute values.
    ///
    /// SMART attribute values are read once in Disk::new and cached. Methods such as
    /// `get_temperature` use these cached values and do not access the disk. Call this method to
    /// refresh the cached values.
    ///
    /// Note: calling this method might cause the disk to wake up from sleep. Consider checking if
    /// the disk is asleep using `check_sleep_mode` before calling this method to avoid this.
    pub fn refresh_smart_data(&mut self) -> Result<(), Errno> {
        unsafe {
            let ret = sk_disk_smart_read_data(self.skdisk);
            if ret < 0 {
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail));
            }
            Ok(())
        }
    }

    /// Returns a u64 representing the size of the disk in bytes.
    pub fn get_disk_size(&mut self) -> Result<u64, Errno> {
        unsafe {
            let mut bytes: u64 = 0;
            let ret = sk_disk_get_size(self.skdisk, &mut bytes);
            if ret < 0 {
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail));
            }
            return Ok(bytes);
        }
    }

    /// Returns a bool of true if sleep mode is supported, false otherwise.
    pub fn check_sleep_mode(&mut self) -> Result<bool, Errno> {
        unsafe {
            let mut mode: SkBool = 0;
            let ret = sk_disk_check_sleep_mode(self.skdisk, &mut mode);
            if ret < 0 {
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail));
            }
            if mode == 0 {
                Ok(false)
            } else {
                Ok(true)
            }
        }
    }

    /// Returns a u64 representing the power on time in milliseconds
    pub fn get_power_on(&mut self) -> Result<u64, Errno> {
        unsafe {
            let mut power_on_time: u64 = 0;
            let ret = sk_disk_smart_get_power_on(self.skdisk, &mut power_on_time);
            if ret < 0 {
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail));
            }
            Ok(power_on_time)
        }
    }

    /// Returns a u64 representing the number of power on cycles
    pub fn get_power_cycle_count(&mut self) -> Result<u64, Errno> {
        unsafe {
            let mut power_cycle_count: u64 = 0;
            let ret = sk_disk_smart_get_power_cycle(self.skdisk, &mut power_cycle_count);
            if ret < 0 {
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail));
            }
            Ok(power_cycle_count)
        }
    }

    /// Returns a u64 representing the number of bad sections on the disk
    pub fn get_bad_sectors(&mut self) -> Result<u64, Errno> {
        unsafe {
            let mut bad_sector_count: u64 = 0;
            let ret = sk_disk_smart_get_bad(self.skdisk, &mut bad_sector_count);
            if ret < 0 {
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail));
            }
            Ok(bad_sector_count)
        }
    }

    /// Returns a u64 representing the mkelvin of the disk
    pub fn get_temperature(&mut self) -> Result<u64, Errno> {
        unsafe {
            let mut mkelvin: u64 = 0;
            let ret = sk_disk_smart_get_temperature(self.skdisk, &mut mkelvin);
            if ret < 0 {
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail));
            }
            Ok(mkelvin)
        }
    }

    /// Returns true if the disk passed smart, false otherwise.
    pub fn get_smart_status(&mut self) -> Result<bool, Errno> {
        unsafe {
            let mut good: SkBool = 0;
            let ret = sk_disk_smart_status(self.skdisk, &mut good);
            if ret < 0 {
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail));
            }
            if good == 0 {
                Ok(false)
            } else {
                Ok(true)
            }
        }
    }

    /// This will dump all available information to stdout about the drive
    pub fn dump(&mut self) -> Result<(), Errno> {
        unsafe {
            let ret = sk_disk_dump(self.skdisk);
            if ret < 0 {
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail));
            }
            Ok(())
        }
    }

    pub fn identify_is_available(&mut self) -> Result<bool, Errno> {
        unsafe {
            let mut available: SkBool = 0;
            let ret = sk_disk_identify_is_available(self.skdisk, &mut available);
            if ret < 0 {
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail));
            }
            if available == 0 {
                Ok(false)
            } else {
                Ok(true)
            }
        }
    }

    /// Query the device and return whether or not smart is supported on it
    pub fn smart_is_available(&mut self) -> Result<bool, Errno> {
        unsafe {
            let mut available: SkBool = 0;
            let ret = sk_disk_smart_is_available(self.skdisk, &mut available);
            if ret < 0 {
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail));
            }
            if available == 0 {
                Ok(false)
            } else {
                Ok(true)
            }
        }
    }

    // This is a lower level function that is used to build new smart functions
    pub fn parse_attributes(&mut self, parser_callback: extern "C" fn(*mut SkDisk, *const SkSmartAttributeParsedData, *mut std::ffi::c_void), userdata: *mut c_void ) -> Result<(), Errno> 
    {
        unsafe {
            let ret = sk_disk_smart_parse_attributes(self.skdisk, parser_callback, userdata);
            if ret < 0 {
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail));
            }
            Ok(())
        }
    }

    /// Query the device and return whether or not a particular smart test is supported on it
    pub fn smart_test_available(
        &mut self,
        test_attributes: &mut SkSmartParsedData,
        test_type: SkSmartSelfTest,
    ) -> Result<bool, Errno> {
        unsafe {
            let available = sk_smart_self_test_available(test_attributes, test_type);
            if available == 0 {
                Ok(false)
            } else {
                Ok(true)
            }
        }
    }

    pub fn execute_smart_self_test(&mut self, test_type: SkSmartSelfTest) -> Result<(), Errno> {
        unsafe {
            let ret = sk_disk_smart_self_test(self.skdisk, test_type);
            if ret < 0 {
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail));
            }
            Ok(())
        }
    }

    pub fn smart_get_overall(&mut self) -> Result<SkSmartOverall, Errno> {
        unsafe {
            let mut overall: SkSmartOverall = SkSmartOverall::SK_SMART_OVERALL_GOOD;
            let ret = sk_disk_smart_get_overall(self.skdisk, &mut overall);
            if ret < 0 {
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail));
            }
            Ok(overall)
        }
    }

    /// Get the model, firmware, and serial of the disk as a IdentifyParsedDatastruct
    /// If Errno::EINVAL gets returned there is a problem with the C string parser
    pub fn identify_parse(&mut self) -> Result<IdentifyParsedData, Errno> {
        let mut available: SkBool = 0;
        unsafe {
            sk_disk_identify_is_available(self.skdisk, &mut available);
            if available == 1{
                let parsed_data_pointer: *const SkIdentifyParsedData = null();
                let ret = sk_disk_identify_parse(self.skdisk, &parsed_data_pointer);
                if ret < 0 {
                    let fail = nix::errno::errno();
                    return Err(Errno::from_i32(fail));
                }
                let model = CStr::from_ptr((*parsed_data_pointer).model.as_ptr()).to_str().map_err(|_| Errno::EINVAL)?;
                let firmware = CStr::from_ptr((*parsed_data_pointer).firmware.as_ptr()).to_str().map_err(|_| Errno::EINVAL)?;
                let serial = CStr::from_ptr((*parsed_data_pointer).serial.as_ptr()).to_str().map_err(|_| Errno::EINVAL)?;
                
                let parsed_data: IdentifyParsedData = IdentifyParsedData {
                    serial: String::from(serial),
                    firmware: String::from(firmware),
                    model: String::from(model)
                };
                Ok(parsed_data)
            }
            else{
                let fail = nix::errno::errno();
                return Err(Errno::from_i32(fail));
            }
        }
    }
}

impl Drop for Disk {
    fn drop(&mut self) {
        unsafe {
            sk_disk_free(self.skdisk);
        }
    }
}

/// Helper fn. I believe this function returns how many minutes it takes to run a particular type of smart test
/// but it's not entirely clear and the original source code doesn't have a comment
pub fn smart_test_polling_minutes(test_attributes: &SkSmartParsedData, test: SkSmartSelfTest) -> u32 {
    unsafe {
        sk_smart_self_test_polling_minutes(test_attributes, test)
    }
}

/// Helper fn. Transforms a SkSmartSelfTest into a String
pub fn smart_test_to_string(test: SkSmartSelfTest) -> String {
    unsafe {
        let str_ptr = sk_smart_self_test_to_string(test);
        let c_str = CStr::from_ptr(str_ptr);
        c_str.to_string_lossy().into_owned()
    }
}

/// Helper fn. Transforms an SkSmartOfflineDataCollectionStatus into a String
pub fn get_offline_collection_status_as_string(status: SkSmartOfflineDataCollectionStatus) -> String {
    unsafe {
        let str_ptr = sk_smart_offline_data_collection_status_to_string(status);
        let c_str = CStr::from_ptr(str_ptr);
        c_str.to_string_lossy().into_owned()
    }
}

/// Helper fn. Transforms an SkSmartSelfTestExecutionStatus into a String
pub fn get_smart_status_as_string(status: SkSmartSelfTestExecutionStatus) -> String {
    unsafe {
        let str_ptr = sk_smart_self_test_execution_status_to_string(status);
        let c_str = CStr::from_ptr(str_ptr);
        c_str.to_string_lossy().into_owned()
    }
}


/*
pub fn sk_disk_identify_parse(d: *mut *mut SkDisk, data: *const SkIdentifyParsedData) -> ::libc::c_int;
pub fn sk_disk_get_blob(d: *mut *mut SkDisk, blob: *const ::libc::c_void, size: *mut size_t) -> ::libc::c_int;
pub fn sk_disk_set_blob(d: *mut SkDisk, blob: *const ::libc::c_void, size: size_t) -> ::libc::c_int;
pub fn sk_disk_smart_parse(d: *mut *mut SkDisk, data: *const SkSmartParsedData) -> ::libc::c_int;
*/
