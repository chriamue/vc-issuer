use anyhow::Result;
use qrcode::QrCode;

pub async fn create_qr_code(id: String, base_url: String) -> Result<String> {
    let didcomm_inv = format!("{}/id?={}", base_url, id);
    let code = QrCode::new(&didcomm_inv)?;
    let string = code
        .render::<char>()
        .quiet_zone(true)
        .module_dimensions(2, 1)
        .dark_color('█')
        .light_color(' ')
        .build();
    Ok(string)
}
