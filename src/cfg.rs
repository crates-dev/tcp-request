use crate::*;
use color_output::*;
use std::{
    thread::{spawn, JoinHandle},
    time::Instant,
};
use std_macro_extensions::*;

#[allow(dead_code)]
fn output(title: &str, msg: &str, color: Color) {
    OutputListBuilder::new()
        .add(
            OutputBuilder::new()
                .text(title)
                .bg_color(ColorType::Use(Color::Cyan))
                .blod(true)
                .build(),
        )
        .add(
            OutputBuilder::new()
                .text(msg)
                .bg_color(ColorType::Use(color))
                .blod(true)
                .endl(true)
                .build(),
        )
        .run();
}

#[test]
fn test_http_post_request() {
    let mut request_builder = RequestBuilder::new()
        .host("127.0.0.1")
        .port(80)
        .data("tcp send")
        .build();
    request_builder
        .send()
        .and_then(|response| {
            output(
                "ResponseTrait => ",
                &format!("{:?}", response.text()),
                Color::Green,
            );
            Ok(())
        })
        .unwrap_or_else(|e| output("Error => ", &format!("{:?}", e), Color::Red));
}

#[test]
fn test_readme_text() {
    let mut request_builder = RequestBuilder::new()
        .host("127.0.0.1")
        .port(80)
        .data("tcp send")
        .build();
    request_builder
        .send()
        .and_then(|response| {
            println!("ResponseTrait => {:?}", response.text());
            Ok(())
        })
        .unwrap_or_else(|e| println!("Error => {:?}", e));
}

#[test]
fn test_readme_binary() {
    let mut request_builder = RequestBuilder::new()
        .host("127.0.0.1")
        .port(80)
        .data("tcp send".as_bytes())
        .build();
    request_builder
        .send()
        .and_then(|response| {
            println!("ResponseTrait => {:?}", response.text());
            Ok(())
        })
        .unwrap_or_else(|e| println!("Error => {:?}", e));
}

#[test]
fn test_thread_http_get_request() {
    let num_threads: i32 = 10;
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let request_builder: Arc<Mutex<BoxRequestTrait>> = Arc::new(Mutex::new(
        RequestBuilder::new()
            .host("127.0.0.1")
            .port(8080)
            .timeout(10)
            .buffer(1_024_000)
            .build(),
    ));
    for _ in 0..num_threads {
        let request_builder: Arc<
            Mutex<
                Box<
                    dyn RequestTrait<
                        RequestResult = Result<
                            Box<dyn ResponseTrait<OutputText = String, OutputBinary = Vec<u8>>>,
                            RequestError,
                        >,
                    >,
                >,
            >,
        > = Arc::clone(&request_builder);
        let handle: JoinHandle<()> = spawn(move || {
            let mut request_builder = request_builder.lock().unwrap();
            let start_time: Instant = Instant::now();
            match request_builder.send() {
                Ok(response) => {
                    let duration: std::time::Duration = start_time.elapsed();
                    output(
                        "Thread finished in: ",
                        &format!("{:?}", duration),
                        Color::Blue,
                    );
                    let response_text = response.text();
                    output(
                        "ResponseTrait => ",
                        &format!("{:?}", response_text),
                        Color::Green,
                    );
                }
                Err(e) => {
                    let duration: std::time::Duration = start_time.elapsed();
                    output(
                        "Thread finished in: ",
                        &format!("{:?}", duration),
                        Color::Blue,
                    );
                    output("Error => ", &format!("{:?}", e), Color::Red);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
