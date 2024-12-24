use crate::test_case::{Section, TestCase, Verifier};
use crate::util::{assert_connection_error, default_headers, default_headers_plus};
use h3i::{
    self,
    actions::h3::{send_headers_frame, Action},
    config::Config,
    frame::ExpectedFrame,
    quiche::h3::{frame::Frame, Header, WireErrorCode},
};

pub fn generate_sections(host: String) -> Vec<Section> {
    let base_config = Config::new()
        .with_host_port(host.clone())
        .with_idle_timeout(2000)
        .build()
        .unwrap();

    let four = Section::new(
        "4. Expressing HTTP Semantics in HTTP/3",
        vec![
            TestCase::new(
                // TODO: codify test naming
                "4.1. Receipt of an invalid sequence of frames MUST be treated as a connection error of type H3_FRAME_UNEXPECTED.",
                base_config.clone(),
                vec![
                    Action::SendFrame {
                        stream_id: 0,
                        fin_stream: false,
                        frame: Frame::Data {
                            payload: b"invalid".to_vec(),
                        }
                    },
                    send_headers_frame(0, true, default_headers(&host))
                ],
                Verifier::Fn(assert_connection_error(true, WireErrorCode::FrameUnexpected))
            ),
            TestCase::new(
                "4.1.2. A request or response containing uppercase characters in field names MUST be treated as malformed.",
                base_config.clone(),
                vec![send_headers_frame(0, true, default_headers_plus(&host, Header::new(b"UpperCaseField", b"UpperCaseValue")))],
                Verifier::ExpectedFrames(vec![ExpectedFrame::new(
                   0,
                   vec![Header::new(b":status", b"400")].into()
                )])
            ),
        ],
    );

    let six = Section::new(
        "6. Stream Mapping and Usage",
        vec![
            TestCase::new(
                "6.2.2. If a server receives a client-initiated push stream, this MUST be treated as a connection error of type H3_STREAM_CREATION_ERROR.",
                base_config.clone(),
                vec![
                    Action::SendFrame { stream_id: 0, fin_stream: true, frame: Frame::PushPromise { push_id: 0, header_block: vec![] } }
                ],
                Verifier::Fn(assert_connection_error(true, WireErrorCode::StreamCreationError))
            )
        ],
    );

    vec![four, six]
}
