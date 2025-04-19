use sycamore::prelude::*;
use sycamore::web::tags::HtmlVideo;
use sycamore_futures::spawn_local_scoped;
use webcam_live::VideoStream;
#[derive(Props)]
struct HelloProps {
    name: String,
}

#[component]
fn Greeting(name: HelloProps) -> View {
    view! {
        p { "hello" (name.name) "!"}
    }
}

#[component]
fn Video() -> View {
    let video_ref = create_node_ref();
    spawn_local_scoped(async move {
        let mut video_stream = VideoStream::new(video_ref);
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
        div(class="flex justify-center") {
           video(
                ref=video_ref,
                class="border-2 border-gray-300 rounded-md",
                autoplay=true,
                width="1280",
                height="720",
            )
       }
    }
}
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    sycamore::render(|| {
        view! {
            div(class="container p-2") {
                Video()
            }
        }
    });
}
