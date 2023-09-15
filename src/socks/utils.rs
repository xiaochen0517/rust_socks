pub fn check_version(version: &u8) -> Result<(), String> {
    if *version != 0x05u8 {
        return Err("不是socks5协议".to_string());
    }
    Ok(())
}

pub fn check_method_length(length: &u8) -> Result<usize, String> {
    if *length <= 0 {
        return Err("方法数长度字节数不能小于0".to_string());
    }
    Ok(*length as usize)
}