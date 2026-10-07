use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};
use windows_sys::Win32::{
    Foundation::LocalFree,
    Security::{Authorization::*, *},
    System::SystemInformation::GetSystemDirectoryW,
};

pub fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().chain(Some(0)).collect()
}

pub fn system_tool(name: &str) -> Result<PathBuf, String> {
    let mut buf = [0u16; 32768];
    let len = unsafe { GetSystemDirectoryW(buf.as_mut_ptr(), buf.len() as u32) } as usize;
    if len == 0 || len >= buf.len() {
        return Err(std::io::Error::last_os_error().to_string());
    }
    let root = PathBuf::from(String::from_utf16_lossy(&buf[..len]));
    Ok(if name == "powershell.exe" {
        root.join("WindowsPowerShell/v1.0/powershell.exe")
    } else {
        root.join(name)
    })
}

pub fn engine_path(resources: &Path) -> Result<PathBuf, String> {
    let root = if cfg!(debug_assertions) {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../BinTools/x86_64")
    } else {
        resources.join("BinTools/x86_64")
    };
    verify_engine(&root)?;
    root.join("goodbyefirewall-daemon.exe")
        .canonicalize()
        .map_err(|err| err.to_string())
}

const FILES: [(&str, &str); 3] = [
    (
        "goodbyefirewall-daemon.exe",
        "8d412b094bb9c137ff25ba9a794d1122ecc84bb776debff6c249723a13cc31cd",
    ),
    (
        "WinDivert.dll",
        "6110bfa44667405179c3e15e12af1b62037e447ed59b054b19042032995e6c7e",
    ),
    (
        "WinDivert64.sys",
        "e69b5ba3f0cd6cfb2983e442636e7f0b342b61b15264b0328317d4559c82cf50",
    ),
];

pub fn verify_engine(root: &Path) -> Result<(), String> {
    let root = root
        .canonicalize()
        .map_err(|err| format!("Moteur introuvable : {err}"))?;
    if !cfg!(debug_assertions) {
        require_protected(&root)?;
    }
    for (name, expected) in FILES {
        let file = root
            .join(name)
            .canonicalize()
            .map_err(|err| format!("{name} : {err}"))?;
        if file.parent() != Some(root.as_path()) {
            return Err(format!("Chemin du moteur non autorisé : {name}"));
        }
        if !cfg!(debug_assertions) {
            require_protected(&file)?;
        }
        let bytes = fs::read(&file).map_err(|err| err.to_string())?;
        if format!("{:x}", Sha256::digest(bytes)) != expected {
            return Err(format!("Intégrité du moteur invalide : {name}"));
        }
    }
    Ok(())
}

// Fail closed on permissions we cannot interpret. An elevated release may only
// execute files whose owner/write permissions belong to privileged Windows SIDs.
pub fn require_protected(path: &Path) -> Result<(), String> {
    let name = wide(&path.to_string_lossy());
    let mut owner = std::ptr::null_mut();
    let mut dacl = std::ptr::null_mut();
    let mut descriptor = std::ptr::null_mut();
    let error = unsafe {
        GetNamedSecurityInfoW(
            name.as_ptr(),
            SE_FILE_OBJECT,
            OWNER_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION,
            &mut owner,
            std::ptr::null_mut(),
            &mut dacl,
            std::ptr::null_mut(),
            &mut descriptor,
        )
    };
    if error != 0 {
        return Err(format!("Vérification des droits impossible : {error}"));
    }
    let checked = unsafe {
        (|| {
            if dacl.is_null() || !privileged_sid(owner) {
                return Err("Propriétaire ou droits non sécurisés.".into());
            }
            for i in 0..(*dacl).AceCount {
                let mut ace = std::ptr::null_mut();
                if GetAce(dacl, i as u32, &mut ace) == 0 {
                    return Err(std::io::Error::last_os_error().to_string());
                }
                let header = &*(ace as *const ACE_HEADER);
                if header.AceFlags as u32 & INHERIT_ONLY_ACE != 0 || header.AceType == 1 {
                    continue;
                }
                if header.AceType != 0 {
                    return Err("Type de permission non pris en charge.".into());
                }
                let allow = &*(ace as *const ACCESS_ALLOWED_ACE);
                // Write/add/append/delete children, DELETE, WRITE_DAC/OWNER, GENERIC_WRITE/ALL.
                let modifies = allow.Mask & 0x500d0156 != 0;
                let sid = (&allow.SidStart as *const u32).cast_mut().cast();
                if modifies && !privileged_sid(sid) {
                    return Err(
                        "Installation modifiable par un utilisateur non administrateur.".into(),
                    );
                }
            }
            Ok(())
        })()
    };
    unsafe {
        LocalFree(descriptor);
    }
    checked.map_err(|err: String| {
        format!("{} : {err} Réinstallez dans Program Files.", path.display())
    })
}

unsafe fn privileged_sid(sid: PSID) -> bool {
    if sid.is_null() {
        return false;
    }
    if IsWellKnownSid(sid, WinLocalSystemSid) != 0
        || IsWellKnownSid(sid, WinBuiltinAdministratorsSid) != 0
    {
        return true;
    }
    let mut text = std::ptr::null_mut();
    if ConvertSidToStringSidW(sid, &mut text) == 0 {
        return false;
    }
    let mut len = 0;
    while *text.add(len) != 0 {
        len += 1;
    }
    let value = String::from_utf16_lossy(std::slice::from_raw_parts(text, len));
    LocalFree(text.cast());
    value == "S-1-5-80-956008885-3418522649-1831038044-1853292631-2271478464"
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checked_in_engine_matches_pinned_hashes() {
        verify_engine(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../BinTools/x86_64"))
            .unwrap();
    }
    #[test]
    fn missing_or_replaced_engine_fails_closed() {
        assert!(verify_engine(Path::new("nonexistent-engine")).is_err());
        let root = std::env::temp_dir().join(format!("gbf-engine-test-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("goodbyefirewall-daemon.exe"), b"replaced").unwrap();
        assert!(verify_engine(&root).is_err());
        fs::remove_file(root.join("goodbyefirewall-daemon.exe")).unwrap();
        fs::remove_dir(root).unwrap();
    }
    #[test]
    fn user_writable_files_cannot_back_permanent_elevation() {
        let path = std::env::temp_dir().join(format!("gbf-acl-test-{}", std::process::id()));
        fs::write(&path, b"untrusted").unwrap();
        let checked = require_protected(&path);
        fs::remove_file(path).unwrap();
        assert!(checked.is_err());
    }
}
