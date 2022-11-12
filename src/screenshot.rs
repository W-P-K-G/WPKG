use crate::info_crypt;
use anyhow::anyhow;
use captrs::*;

pub struct Screenshot {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u8>,
}

pub fn screenshot() -> anyhow::Result<Screenshot> {
    info_crypt!("Creating screenshot...");

    let mut capturer = match Capturer::new(0) {
        Ok(capt) => capt,
        Err(err) => return Err(anyhow!(err)),
    };

    let (w, h) = capturer.geometry();

    let ps = match capturer.capture_frame() {
        Ok(buf) => buf,
        Err(err) => return Err(anyhow!("Capture frame error: {:?}", err)),
    };

    let mut img: Vec<u8> = Vec::with_capacity((w * h * 3) as usize);

    for Bgr8 { r, g, b, .. } in ps.into_iter() {
        img.push(r);
        img.push(g);
        img.push(b);
    }

    Ok(Screenshot {
        width: w,
        height: h,
        buffer: img,
    })
}
