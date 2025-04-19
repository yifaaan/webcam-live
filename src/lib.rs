use js_sys::wasm_bindgen::JsValue;
use sycamore::{
    prelude::MaybeDyn,
    web::{
        GlobalProps, NodeRef, SetAttribute,
        tags::{HtmlVideo, video},
    },
};
use tracing::info;
use wasm_bindgen::JsCast;

pub struct VideoStream {
    el: HtmlVideo,
}

impl VideoStream {
    pub fn new(node_ref: NodeRef) -> Self {
        let el = video();
        Self {
            el: el.r#ref(node_ref),
        }
    }

    pub async fn set_video_src(&mut self, video_constraints: &serde_json::Value) {
        let window = web_sys::window().expect("no global window");
        let navigator = window.navigator();
        let media_devices = navigator.media_devices().expect("no media devices");
        let mut constraints = web_sys::MediaStreamConstraints::new();
        constraints.set_video(&JsValue::from_bool(true));
        info!("devices(tracing_wasm): {:?}", media_devices);
        web_sys::console::log_1(&media_devices);
        let mut constraints = web_sys::MediaStreamConstraints::new();
        // web_sys::console::log_1(&constraints);
        let json_string = serde_json::to_string(video_constraints).unwrap();
        let js_value = js_sys::JSON::parse(&json_string).unwrap();
        constraints.set_video(&js_value);
        constraints.set_audio(&false.into());
        let media = wasm_bindgen_futures::JsFuture::from(
            media_devices
                .get_user_media_with_constraints(&constraints)
                .unwrap(),
        )
        .await
        .unwrap();
        // let media_stream = media.unchecked_into::<web_sys::MediaStream>();
        // info!("media_stream(tracing_wasm): {:?}", media_stream);
        let a = MaybeDyn::from(media);
        self.el.set_attribute("srcObject", a);
    }
}
