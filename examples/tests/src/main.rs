use nokhwa::{Camera, native_api_backend, query};
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{RequestedFormat, RequestedFormatType, CameraIndex, CameraInfo, ApiBackend};

fn get_cameras() -> Vec<CameraInfo> {
    let backend : ApiBackend = native_api_backend().unwrap();
     query(backend).unwrap()
}

fn main() {
    for camera_info in get_cameras() {
        println!("Caméra {:#?}", camera_info.human_name());

        let mut index: CameraIndex = camera_info.index().clone();
        let format: RequestedFormat = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
        let mut camera = Camera::new(index, format).unwrap();
        for camera_format in camera.compatible_camera_formats().unwrap(){
            let mut index: CameraIndex = camera_info.index().clone();
            let mut format: RequestedFormat = RequestedFormat::new::<RgbFormat>(RequestedFormatType::Exact(camera_format));
            let mut camera = Camera::new(index, format).unwrap();
        }





    }
}
