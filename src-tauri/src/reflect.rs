// A read-only WinDivert REFLECT observer confirms the owned daemon's final
// network filter, even when its C stdout is buffered. It captures no packets.
use std::{
    ffi::{c_void, CString},
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::JoinHandle,
};
use windows_sys::Win32::{Foundation::HANDLE, System::LibraryLoader::*};

type Open = unsafe extern "system" fn(*const u8, u32, i16, u64) -> HANDLE;
type Recv = unsafe extern "system" fn(HANDLE, *mut c_void, u32, *mut u32, *mut Address) -> i32;
type Shutdown = unsafe extern "system" fn(HANDLE, u32) -> i32;
type Close = unsafe extern "system" fn(HANDLE) -> i32;
type Format = unsafe extern "system" fn(*const u8, u32, *mut u8, u32) -> i32;
#[repr(C)]
struct Address {
    timestamp: i64,
    control: u32,
    reserved: u32,
    data: [u8; 64],
}
struct Api {
    module: usize,
    open: Open,
    recv: Recv,
    shutdown: Shutdown,
    close: Close,
    format: Format,
}
impl Drop for Api {
    fn drop(&mut self) {
        unsafe {
            windows_sys::Win32::Foundation::FreeLibrary(self.module as _);
        }
    }
}

impl Api {
    fn load(dll: &Path) -> Result<Arc<Self>, String> {
        let path = crate::paths::wide(&dll.to_string_lossy());
        let module = unsafe {
            LoadLibraryExW(
                path.as_ptr(),
                std::ptr::null_mut(),
                LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_SYSTEM32,
            )
        };
        if module.is_null() {
            return Err(std::io::Error::last_os_error().to_string());
        }
        macro_rules! symbol {
            ($name:literal, $kind:ty) => {
                match unsafe { GetProcAddress(module, concat!($name, "\0").as_ptr()) } {
                    Some(ptr) => unsafe {
                        std::mem::transmute::<unsafe extern "system" fn() -> isize, $kind>(ptr)
                    },
                    None => {
                        unsafe {
                            windows_sys::Win32::Foundation::FreeLibrary(module);
                        }
                        return Err(concat!("Fonction WinDivert absente : ", $name).into());
                    }
                }
            };
        }
        Ok(Arc::new(Self {
            module: module as usize,
            open: symbol!("WinDivertOpen", Open),
            recv: symbol!("WinDivertRecv", Recv),
            shutdown: symbol!("WinDivertShutdown", Shutdown),
            close: symbol!("WinDivertClose", Close),
            format: symbol!("WinDivertHelperFormatFilter", Format),
        }))
    }
}

pub struct Observer {
    api: Arc<Api>,
    handle: usize,
    reader: Option<JoinHandle<()>>,
}
impl Observer {
    pub fn start(
        dll: &Path,
        pid: u32,
        ready: Arc<AtomicBool>,
        failed: Arc<AtomicBool>,
    ) -> Result<Self, String> {
        let api = Api::load(dll)?;
        let filter = CString::new(format!("processId == {pid} and layer == NETWORK")).unwrap();
        // REFLECT=4, SNIFF=1, RECV_ONLY=4. Historical OPEN events are also delivered.
        let handle = unsafe { (api.open)(filter.as_ptr().cast(), 4, 0, 1 | 4) };
        if handle as isize == -1 || handle.is_null() {
            return Err(std::io::Error::last_os_error().to_string());
        }
        let handle = handle as usize;
        let reader_api = api.clone();
        let reader = std::thread::spawn(move || {
            let mut packet = vec![0u8; 65536];
            let mut formatted = vec![0u8; 65536];
            let mut address = Address {
                timestamp: 0,
                control: 0,
                reserved: 0,
                data: [0; 64],
            };
            let mut len = 0;
            loop {
                if unsafe {
                    (reader_api.recv)(
                        handle as _,
                        packet.as_mut_ptr().cast(),
                        packet.len() as u32,
                        &mut len,
                        &mut address,
                    )
                } == 0
                {
                    break;
                }
                // The compiled filter object is NUL-terminated. Validate its bounds.
                if len as usize >= packet.len() {
                    continue;
                }
                packet[len as usize] = 0;
                if unsafe {
                    (reader_api.format)(
                        packet.as_ptr(),
                        0,
                        formatted.as_mut_ptr(),
                        formatted.len() as u32,
                    )
                } == 0
                {
                    continue;
                }
                let text = String::from_utf8_lossy(
                    &formatted[..formatted
                        .iter()
                        .position(|&b| b == 0)
                        .unwrap_or(formatted.len())],
                );
                if !is_main_filter(&text) {
                    continue;
                }
                match (address.control >> 8) & 0xff {
                    8 => ready.store(true, Ordering::Release),
                    9 => {
                        failed.store(true, Ordering::Release);
                        break;
                    }
                    _ => {}
                }
            }
            failed.store(true, Ordering::Release);
        });
        Ok(Self {
            api,
            handle,
            reader: Some(reader),
        })
    }
}
impl Drop for Observer {
    fn drop(&mut self) {
        let closed = unsafe {
            if (self.api.shutdown)(self.handle as _, 1) == 0 {
                (self.api.close)(self.handle as _);
                true
            } else {
                false
            }
        };
        if let Some(reader) = self.reader.take() {
            let _ = reader.join();
        }
        if !closed {
            unsafe {
                (self.api.close)(self.handle as _);
            }
        }
    }
}
fn is_main_filter(filter: &str) -> bool {
    let filter = filter.to_ascii_lowercase();
    // The pinned GoodbyeDPI engine opens this final filter after its optional
    // passive/QUIC filters; neither of those has a TCP destination-port clause.
    filter.contains("tcp.dstport") && filter.contains("tcp.srcport") && filter.contains("tcp.ack")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn final_filter_is_distinct_from_partial_startup() {
        assert!(is_main_filter(
            "tcp.Ack and (tcp.SrcPort == 80 or tcp.DstPort == 443)"
        ));
        assert!(!is_main_filter(
            "inbound and tcp.SrcPort == 443 and tcp.Rst"
        ));
        assert!(!is_main_filter("outbound and udp.DstPort == 443"));
    }
    #[test]
    fn address_layout_matches_windivert_22() {
        assert_eq!(std::mem::size_of::<Address>(), 80);
    }
    #[test]
    fn shipped_dll_formats_the_final_filter_without_loading_the_driver() {
        let dll = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../BinTools/x86_64/WinDivert.dll")
            .canonicalize()
            .unwrap();
        let api = Api::load(&dll).unwrap();
        let filter = CString::new("tcp.Ack and (tcp.SrcPort == 80 or tcp.DstPort == 443)").unwrap();
        let mut output = [0u8; 8192];
        assert_ne!(
            unsafe {
                (api.format)(
                    filter.as_ptr().cast(),
                    0,
                    output.as_mut_ptr(),
                    output.len() as u32,
                )
            },
            0
        );
        let text = String::from_utf8_lossy(&output[..output.iter().position(|&b| b == 0).unwrap()]);
        assert!(is_main_filter(&text), "{text}");
    }
}
