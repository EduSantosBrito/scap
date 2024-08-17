use std::slice;

use screencapturekit::cm_sample_buffer::CMSampleBuffer;
use screencapturekit_sys::cm_sample_buffer_ref::CMSampleBufferGetImageBuffer;

use crate::frame::{
    convert_bgra_to_rgb, get_cropped_data, remove_alpha_channel, BGRAFrame, BGRFrame, RGBFrame,
};
use core_video_sys::{
    CVPixelBufferGetBaseAddress, CVPixelBufferGetBytesPerRow, CVPixelBufferGetHeight,
    CVPixelBufferGetWidth, CVPixelBufferLockBaseAddress, CVPixelBufferRef,
    CVPixelBufferUnlockBaseAddress,
};

pub unsafe fn create_bgr_frame(sample_buffer: CMSampleBuffer) -> Option<BGRFrame> {
    let buffer_ref = &(*sample_buffer.sys_ref);
    let epoch = sample_buffer.sys_ref.get_presentation_timestamp().value;
    let pixel_buffer = CMSampleBufferGetImageBuffer(buffer_ref) as CVPixelBufferRef;

    CVPixelBufferLockBaseAddress(pixel_buffer, 0);

    let width = CVPixelBufferGetWidth(pixel_buffer);
    let height = CVPixelBufferGetHeight(pixel_buffer);
    if width == 0 || height == 0 {
        return None;
    }

    let base_address = CVPixelBufferGetBaseAddress(pixel_buffer);
    let bytes_per_row = CVPixelBufferGetBytesPerRow(pixel_buffer);

    let data = slice::from_raw_parts(base_address as *mut u8, bytes_per_row * height).to_vec();

    let cropped_data = get_cropped_data(
        data,
        (bytes_per_row / 4) as i32,
        height as i32,
        width as i32,
    );

    CVPixelBufferUnlockBaseAddress(pixel_buffer, 0);

    Some(BGRFrame {
        display_time: epoch as u64,
        width: width as i32, // width does not give accurate results - https://stackoverflow.com/questions/19587185/cvpixelbuffergetbytesperrow-for-cvimagebufferref-returns-unexpected-wrong-valu
        height: height as i32,
        data: remove_alpha_channel(cropped_data),
    })
}

pub unsafe fn create_bgra_frame(sample_buffer: CMSampleBuffer) -> Option<BGRAFrame> {
    let buffer_ref = &(*sample_buffer.sys_ref);
    let epoch = sample_buffer.sys_ref.get_presentation_timestamp().value;
    let pixel_buffer = CMSampleBufferGetImageBuffer(buffer_ref) as CVPixelBufferRef;

    CVPixelBufferLockBaseAddress(pixel_buffer, 0);

    let width = CVPixelBufferGetWidth(pixel_buffer);
    let height = CVPixelBufferGetHeight(pixel_buffer);
    if width == 0 || height == 0 {
        return None;
    }

    let base_address = CVPixelBufferGetBaseAddress(pixel_buffer);
    let bytes_per_row = CVPixelBufferGetBytesPerRow(pixel_buffer);

    let mut data: Vec<u8> = vec![];
    for i in 0..height {
        let start = (base_address as *mut u8).wrapping_add(i * bytes_per_row);
        data.extend_from_slice(slice::from_raw_parts(start, 4 * width));
    }

    CVPixelBufferUnlockBaseAddress(pixel_buffer, 0);

    Some(BGRAFrame {
        display_time: epoch as u64,
        width: width as i32, // width does not give accurate results - https://stackoverflow.com/questions/19587185/cvpixelbuffergetbytesperrow-for-cvimagebufferref-returns-unexpected-wrong-valu
        height: height as i32,
        data,
    })
}

pub unsafe fn create_rgb_frame(sample_buffer: CMSampleBuffer) -> Option<RGBFrame> {
    let buffer_ref = &(*sample_buffer.sys_ref);
    let epoch = sample_buffer.sys_ref.get_presentation_timestamp().value;
    let pixel_buffer = CMSampleBufferGetImageBuffer(buffer_ref) as CVPixelBufferRef;

    CVPixelBufferLockBaseAddress(pixel_buffer, 0);

    let width = CVPixelBufferGetWidth(pixel_buffer);
    let height = CVPixelBufferGetHeight(pixel_buffer);
    if width == 0 || height == 0 {
        return None;
    }

    let base_address = CVPixelBufferGetBaseAddress(pixel_buffer);
    let bytes_per_row = CVPixelBufferGetBytesPerRow(pixel_buffer);

    let data = slice::from_raw_parts(base_address as *mut u8, bytes_per_row * height).to_vec();

    let cropped_data = get_cropped_data(
        data,
        (bytes_per_row / 4) as i32,
        height as i32,
        width as i32,
    );

    CVPixelBufferUnlockBaseAddress(pixel_buffer, 0);

    Some(RGBFrame {
        display_time: epoch as u64,
        width: width as i32, // width does not give accurate results - https://stackoverflow.com/questions/19587185/cvpixelbuffergetbytesperrow-for-cvimagebufferref-returns-unexpected-wrong-valu
        height: height as i32,
        data: convert_bgra_to_rgb(cropped_data),
    })
    // (y_width, y_height, data)
}
