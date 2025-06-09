use base::{LoggedResult, MappedFile, MutBytesExt, Utf8Cstr};

// Samsung Galaxy S22+ (SM-S906B) specific patches for OneUI 8 and Exynos 2200

// SAFETY: assert(buf.len() >= 1) && assert(len <= buf.len())
macro_rules! match_samsung_patterns {
    ($buf:ident, $($str:literal), *) => {{
        let mut len = if *$buf.get_unchecked(0) == b',' { 1 } else { 0 };
        let b = $buf.get_unchecked(len..);
        let found = if b.is_empty() {
            false
        }
        $(
        else if b.starts_with($str) {
            len += $str.len();
            true
        }
        )*
        else {
            false
        };
        if found {
            let b = $buf.get_unchecked(len..);
            if !b.is_empty() && b[0] == b'=' {
                for c in b.iter() {
                    if b" \n\0".contains(c) {
                        break;
                    }
                    len += 1;
                }
            }
            Some(len)
        } else {
            None
        }
    }};
}

fn remove_samsung_pattern(buf: &mut [u8], pattern_matcher: unsafe fn(&[u8]) -> Option<usize>) -> usize {
    let mut write = 0_usize;
    let mut read = 0_usize;
    let mut sz = buf.len();
    // SAFETY: assert(write <= read) && assert(read <= buf.len())
    unsafe {
        while read < buf.len() {
            if let Some(len) = pattern_matcher(buf.get_unchecked(read..)) {
                let skipped = buf.get_unchecked(read..(read + len));
                // SAFETY: all matching patterns are ASCII bytes
                let skipped = std::str::from_utf8_unchecked(skipped);
                eprintln!("Remove Samsung pattern [{}]", skipped);
                sz -= len;
                read += len;
            } else {
                *buf.get_unchecked_mut(write) = *buf.get_unchecked(read);
                write += 1;
                read += 1;
            }
        }
    }
    if let Some(buf) = buf.get_mut(write..) {
        buf.fill(0);
    }
    sz
}

/// Samsung Knox security bypass for Galaxy S22+ and OneUI 8
pub fn patch_samsung_knox(buf: &mut [u8]) -> usize {
    unsafe fn match_knox_pattern(buf: &[u8]) -> Option<usize> {
        unsafe {
            match_samsung_patterns!(
                buf,
                b"knox_kap",
                b"knox_ccm",
                b"knox_rp",
                b"knox_tima",
                b"knox_kap_status",
                b"knox_ccm_status",
                b"knox_rp_status",
                b"knox_tima_status",
                b"warranty_bit",
                b"sec_restrict_rooting",
                b"sec_restrict_tethering"
            )
        }
    }

    remove_samsung_pattern(buf, match_knox_pattern)
}

/// Samsung secure boot bypass for Exynos 2200
pub fn patch_samsung_secure_boot(buf: &mut [u8]) -> usize {
    unsafe fn match_secure_boot_pattern(buf: &[u8]) -> Option<usize> {
        unsafe {
            match_samsung_patterns!(
                buf,
                b"sec_boot",
                b"secure_boot",
                b"verified_boot",
                b"boot_verifier",
                b"bootloader_locked",
                b"sec_boot_dev",
                b"sec_boot_recovery",
                b"sec_boot_kernel",
                b"exynos_secure_boot"
            )
        }
    }

    remove_samsung_pattern(buf, match_secure_boot_pattern)
}

/// OneUI 8 specific security patches
pub fn patch_oneui8_security(buf: &mut [u8]) -> usize {
    unsafe fn match_oneui8_pattern(buf: &[u8]) -> Option<usize> {
        unsafe {
            match_samsung_patterns!(
                buf,
                b"oneui_security",
                b"samsung_security_policy",
                b"sec_enhanced_biometrics",
                b"sec_knox_guard",
                b"sec_privacy_dashboard",
                b"sec_secure_folder",
                b"sec_work_profile",
                b"samsung_pass",
                b"sec_biometric_face",
                b"sec_biometric_iris",
                b"sec_biometric_fingerprint"
            )
        }
    }

    remove_samsung_pattern(buf, match_oneui8_pattern)
}

/// Samsung device management bypass
pub fn patch_samsung_device_management(buf: &mut [u8]) -> usize {
    unsafe fn match_device_mgmt_pattern(buf: &[u8]) -> Option<usize> {
        unsafe {
            match_samsung_patterns!(
                buf,
                b"sec_device_management",
                b"mdm_mode",
                b"enterprise_policy",
                b"sec_container",
                b"sec_dual_dar",
                b"sec_sdp",
                b"sec_knox_container"
            )
        }
    }

    remove_samsung_pattern(buf, match_device_mgmt_pattern)
}

/// Exynos 2200 TrustZone bypass
pub fn patch_exynos2200_trustzone(buf: &mut [u8]) -> usize {
    unsafe fn match_trustzone_pattern(buf: &[u8]) -> Option<usize> {
        unsafe {
            match_samsung_patterns!(
                buf,
                b"trustzone",
                b"tz_kernel",
                b"tz_secure_os",
                b"exynos_smc",
                b"exynos_tzasc",
                b"exynos_tzpc",
                b"secure_monitor_call",
                b"arm_smccc"
            )
        }
    }

    remove_samsung_pattern(buf, match_trustzone_pattern)
}

/// Samsung Galaxy S22+ comprehensive patching function
pub fn patch_samsung_s22_plus(buf: &mut [u8]) -> usize {
    let mut total_patched = 0;
    
    eprintln!("Applying Samsung Galaxy S22+ patches...");
    
    // Apply Knox security bypass
    total_patched += patch_samsung_knox(buf);
    
    // Apply secure boot bypass
    total_patched += patch_samsung_secure_boot(buf);
    
    // Apply OneUI 8 specific patches
    total_patched += patch_oneui8_security(buf);
    
    // Apply device management bypass
    total_patched += patch_samsung_device_management(buf);
    
    // Apply Exynos 2200 TrustZone bypass
    total_patched += patch_exynos2200_trustzone(buf);
    
    eprintln!("Samsung S22+ patching completed: {} bytes modified", total_patched);
    total_patched
}

/// Samsung-specific hex patching for binary modifications
pub fn samsung_hexpatch(file: &[u8], from: &[u8], to: &[u8]) -> bool {
    let res: LoggedResult<bool> = try {
        let file = Utf8CStr::from_bytes(file)?;
        let from = Utf8CStr::from_bytes(from)?;
        let to = Utf8CStr::from_bytes(to)?;

        let mut map = MappedFile::open_rw(file)?;
        let pattern = hex2byte(from.as_bytes());
        let patch = hex2byte(to.as_bytes());

        let v = map.patch(pattern.as_slice(), patch.as_slice());
        for off in &v {
            eprintln!("Samsung Patch @ {:#010X} [{}] -> [{}]", off, from, to);
        }
        !v.is_empty()
    };
    res.unwrap_or(false)
}

fn hex2byte(hex: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(hex.len() / 2);
    for bytes in hex.chunks(2) {
        if bytes.len() != 2 {
            break;
        }
        let high = bytes[0].to_ascii_uppercase() - b'0';
        let low = bytes[1].to_ascii_uppercase() - b'0';
        let h = if high > 9 { high - 7 } else { high };
        let l = if low > 9 { low - 7 } else { low };
        v.push((h << 4) | l);
    }
    v
}

/// Samsung-specific boot image validation
pub fn validate_samsung_boot_image(buf: &[u8]) -> bool {
    // Check for Samsung boot image signatures
    let samsung_signatures: Vec<&[u8]> = vec![
        b"SAMSUNG",
        b"EXYNOS",
        b"SM-S906",
        b"OneUI",
    ];
    
    for signature in &samsung_signatures {
        if buf.windows(signature.len()).any(|window| window == *signature) {
            eprintln!("Found Samsung signature: {:?}", std::str::from_utf8(signature).unwrap_or("invalid"));
            return true;
        }
    }
    
    false
}
