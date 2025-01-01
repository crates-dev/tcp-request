use crate::*;
use color_output::*;

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
        .builder();
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
fn readme() {
    let mut request_builder = RequestBuilder::new()
        .host("127.0.0.1")
        .port(80)
        .data("tcp send")
        .builder();
    request_builder
        .send()
        .and_then(|response| {
            println!("ResponseTrait => {:?}", response.text());
            Ok(())
        })
        .unwrap_or_else(|e| output("Error => ", &format!("{:?}", e), Color::Red));
}
