use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let data: Vec<u8> = fs::read("./demojavaprogram/Addition.class")?;
    assert_eq!(data[0..4], [0xCA, 0xFE, 0xBA, 0xBE]);
    Ok(())
}
