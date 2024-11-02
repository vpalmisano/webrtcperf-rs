use neon::prelude::*;
use webrtcperf_vmaf_utils::{process_video, watermark_video};
use crossbeam_channel::unbounded;

fn watermark(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let file_path: Handle<JsString> = cx.argument::<JsString>(0)?;
    let file_path: String = file_path.value(&mut cx);
    let id: Handle<JsString> = cx.argument::<JsString>(1)?;
    let id: String = id.value(&mut cx);

    let (sender, receiver) = unbounded();
    ctrlc::set_handler(move || {
        sender.send("stop").expect("Error sending signal");
    })
    .expect("Error setting Ctrl-C handler");

    let (deferred, promise) = cx.promise();
    let channel = cx.channel();

    std::thread::spawn(move || {
        match watermark_video(&file_path, &id, receiver) {
            Ok(_) => {
                deferred.settle_with(&channel, |mut cx| {
                    Ok(cx.null())
                });
            },
            Err(e) => {
                let error_message = e.to_string();
                if let Err(send_err) = channel.try_send(move |mut cx| {
                    let value = cx.error(&error_message)?;
                    deferred.reject(&mut cx, value);
                    Ok(())
                }) {
                    eprintln!("Failed to send error message: {}", send_err);
                }
            }
        };
    });

    Ok(promise)
}

fn process(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let file_path: Handle<JsString> = cx.argument::<JsString>(0)?;
    let file_path: String = file_path.value(&mut cx);

    let (sender, receiver) = unbounded();
    ctrlc::set_handler(move || {
        sender.send("stop").expect("Error sending signal");
    })
    .expect("Error setting Ctrl-C handler");

    let (deferred, promise) = cx.promise();
    let channel = cx.channel();

    std::thread::spawn(move || {
        match process_video(&file_path, receiver) {
            Ok(_) => {
                deferred.settle_with(&channel, |mut cx| {
                    Ok(cx.null())
                });
            },
            Err(e) => {
                let error_message = e.to_string();
                if let Err(send_err) = channel.try_send(move |mut cx| {
                    let value = cx.error(&error_message)?;
                    deferred.reject(&mut cx, value);
                    Ok(())
                }) {
                    eprintln!("Failed to send error message: {}", send_err);
                }
            }
        };
    });

    Ok(promise)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    env_logger::init();
    cx.export_function("watermark", watermark)?;
    cx.export_function("process", process)?;
    Ok(())
}
