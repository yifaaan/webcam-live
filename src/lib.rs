use leptos::html;
use leptos::prelude::*;
use tracing::info;
use wasm_bindgen::{JsCast, JsValue};

pub struct VideoStream<'a> {
    node_ref: &'a NodeRef<html::Video>,
}

impl<'a> VideoStream<'a> {
    pub fn new(node_ref: &'a NodeRef<html::Video>) -> Self {
        Self { node_ref }
    }

    pub async fn set_video_src(&mut self, video_constraints: &serde_json::Value) {
        let window = web_sys::window().expect("no global window");
        let navigator = window.navigator();
        let media_devices = navigator.media_devices().expect("no media devices");
        info!("devices(tracing_wasm): {:?}", media_devices);
        web_sys::console::log_1(&media_devices);
        let constraints = web_sys::MediaStreamConstraints::new();
        // web_sys::console::log_1(&constraints);
        constraints.set_video(&serde_wasm_bindgen::to_value(video_constraints).unwrap());
        constraints.set_audio(&false.into());

        match media_devices.get_user_media_with_constraints(&constraints) {
            Ok(promise) => {
                match wasm_bindgen_futures::JsFuture::from(promise).await {
                    Ok(media) => {
                        let media_stream = media.unchecked_into::<web_sys::MediaStream>();

                        if let Some(video_element) = self.node_ref.get() {
                            let video_js_value = video_element.clone();

                            // 使用JavaScript的Set方法设置srcObject属性
                            let _ = js_sys::Reflect::set(
                                &video_js_value.into(),
                                &JsValue::from_str("srcObject"),
                                &media_stream,
                            );

                            // 尝试播放视频
                            if let Some(video) =
                                video_element.dyn_ref::<web_sys::HtmlVideoElement>()
                            {
                                match video.play() {
                                    Ok(_) => info!("视频播放成功"),
                                    Err(e) => {
                                        web_sys::console::log_1(&e);
                                        info!("视频播放失败: {:?}", e);
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        web_sys::console::log_1(&e);
                        info!("获取媒体流失败: {:?}", e);
                    }
                }
            }
            Err(e) => {
                web_sys::console::log_1(&e);
                info!("请求摄像头权限失败: {:?}", e);
            }
        }
    }
}
#[test]
fn it_works() {
    // 一个简单的 smoke‑test
    assert_eq!(2 + 2, 4);
}
