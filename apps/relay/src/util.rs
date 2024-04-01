use axum::extract::ws::Message;

pub struct ErrorFormatter {}

impl ErrorFormatter {
    pub fn format_axum_error(err: axum::Error) -> String {
        format!("{:?}", err)
    }

    pub fn format_serde_error(err: serde_json::Error) -> String {
        format!("{:?}", err)
    }

    pub fn format_join_error(err: tokio::task::JoinError) -> String {
        format!("{:?}", err)
    }

    pub fn format_ws_send_error(err: tokio::sync::mpsc::error::SendError<Message>) -> String {
        format!("{:?}", err)
    }
}
