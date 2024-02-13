use nokhwa::{Camera};
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{RequestedFormat, RequestedFormatType, CameraIndex};

fn main() {
    let index: CameraIndex = CameraIndex::Index(1);
    let format: RequestedFormat = RequestedFormat::new::<RgbFormat>(RequestedFormatType::HighestFrameRate(30));

    let mut camera = Camera::new(index, format).unwrap();
    let known = camera.camera_controls_known_camera_controls().unwrap();

    println!("{:?}", known)
}
