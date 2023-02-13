use openai_flows::{CompletionRequest, completion}
use slack_flows::{listen_to_channel, send_message_to_channel};

#[no_mangle]
pub fn run() {
    listen_to_channel("WasmEdge", "general", |sm| {
        let cr = CompletionRequest {
            prompt: sm.text,
            ..Default::default()
        };
        let r = create_completion("openapi", cr);
        r.iter().for_each(|c| {
            send_message_to_channel("WasmEdge", "general", c.to_string());
        });
    });
}
