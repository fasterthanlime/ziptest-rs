use humansize::{file_size_opts, FileSize};
use log::*;
use std::sync::Once;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_logger;

#[wasm_bindgen]
pub fn list_files(fl: web_sys::FileList) {
    setup_logger_once();
    console_error_panic_hook::set_once();

    let file = fl.item(0).expect("at least one file");
    info!("File name: {:#?}", file.name());
    let reader = web_sys::FileReader::new().unwrap();

    let ev_reader = reader.clone();
    let onload = Closure::wrap(Box::new(move || {
        if let Ok(result) = ev_reader.result() {
            let mut lines = Vec::<String>::new();
            let mut log = |msg: String| {
                info!("{}", msg);
                lines.push(msg)
            };

            let array_buf: js_sys::ArrayBuffer = result.into();
            let len = array_buf.byte_length() as usize;
            log(format!(
                "File size: {:#?}",
                len.file_size(file_size_opts::BINARY).unwrap()
            ));

            let mut v = vec![0; len];
            {
                let dv = js_sys::DataView::new(&array_buf, 0, array_buf.byte_length() as usize);
                for i in 0..len {
                    v[i] = dv.get_uint8(i);
                }
            }

            let cursor = std::io::Cursor::new(&v[..]);
            let mut archive = zip::ZipArchive::new(cursor).expect("should be a zip archive");

            log(format!("Is a valid zip archive!"));
            log(format!("Archive has {} files:", archive.len()));

            for i in 0..archive.len() {
                let entry = archive.by_index(i).expect("to get zip entry");
                let mode = entry.unix_mode().unwrap_or(0);
                let is_dir = mode & 0o40000 != 0;

                let line = format!(
                    "{symbol} {name} {mode:#o} ({size} compressed)",
                    symbol = if is_dir { "📂" } else { "📄" },
                    name = entry.sanitized_name().display(),
                    size = entry
                        .compressed_size()
                        .file_size(file_size_opts::BINARY)
                        .unwrap(),
                    mode = mode,
                );
                log(line);
            }

            for line in &lines {
                info!("{}", line);
            }

            let window = web_sys::window().expect("should have a global window");
            let document = window.document().expect("should have a document on window");
            let list_div = document
                .query_selector("#list")
                .expect("should be able to query selector")
                .expect("list div should exist");
            list_div.set_inner_html(&format!("<pre>{}</pre>", lines.join("\n")));

            info!("Done listing files");
        }
    }) as Box<dyn Fn()>);
    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
    onload.forget();

    reader.read_as_array_buffer(&file).unwrap();
}

fn setup_logger_once() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    });
}
