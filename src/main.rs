use leptos::{html, prelude::*, task::spawn_local_scoped};

use webcam_live::VideoStream;

#[component]
fn App() -> impl IntoView {
    let video_ref: NodeRef<html::Video> = NodeRef::new();
    spawn_local_scoped(async move {
        let mut video_stream = VideoStream::new(&video_ref);
        video_stream
            .set_video_src(&serde_json::json!({
                "audio": false,
                "video": {
                    "facingMode": "environment",
                    "width": 1280,
                    "height": 720,
                }
            }))
            .await;
    });

    view! {
        <div class="flex justify-center">
           <video
                node_ref=video_ref
                class="border-2 border-gray-300 rounded-md"
                autoplay=false
                width="100"
                height="50"
                // src="https://www.w3schools.com/html/mov_bbb.mp4"
                // controls=true
            />
       </div>
    }
}
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    leptos::mount::mount_to_body(App)
}
