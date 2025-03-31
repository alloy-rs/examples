use alloy_json_rpc as j;
use serde::Serialize;
use std::{
    borrow::Cow,
    collections::VecDeque,
    sync::{Arc, PoisonError, RwLock},
};

/// A mock response that can be pushed into an [`Asserter`].
pub type MockResponse = j::ResponsePayload;

#[derive(Debug, Clone, Default)]
pub struct Asserter {
    responses: Arc<RwLock<VecDeque<MockResponse>>>,
}

impl Asserter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&self, response: MockResponse) {
        self.write_q().push_back(response);
    }

    #[track_caller]
    pub fn push_success<R: Serialize>(&self, response: &R) {
        let s = serde_json::to_string(response).unwrap();
        self.push(MockResponse::Success(
            serde_json::value::RawValue::from_string(s).unwrap(),
        ));
    }

    pub fn push_failure(&self, error: j::ErrorPayload) {
        self.push(MockResponse::Failure(error));
    }

    pub fn push_failure_msg(&self, msg: impl Into<Cow<'static, str>>) {
        self.push_failure(j::ErrorPayload::internal_error_message(msg.into()));
    }

    pub fn pop_response(&self) -> Option<MockResponse> {
        self.write_q().pop_front()
    }

    pub fn read_q(&self) -> impl std::ops::Deref<Target = VecDeque<MockResponse>> + '_ {
        self.responses.read().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn write_q(&self) -> impl std::ops::DerefMut<Target = VecDeque<MockResponse>> + '_ {
        self.responses.write().unwrap_or_else(PoisonError::into_inner)
    }
}

#[derive(Clone, Debug)]
pub struct MockTransport {
    asserter: Asserter,
}

impl MockTransport {
    pub fn new(asserter: Asserter) -> Self {
        Self { asserter }
    }

    pub fn asserter(&self) -> &Asserter {
        &self.asserter
    }

    async fn handle(self, req: j::RequestPacket) -> crate::TransportResult<j::ResponsePacket> {
        Ok(match req {
            j::RequestPacket::Single(req) => j::ResponsePacket::Single(self.map_request(req)?),
            j::RequestPacket::Batch(reqs) => j::ResponsePacket::Batch(
                reqs.into_iter()
                    .map(|req| self.map_request(req))
                    .collect::<crate::TransportResult<_>>()?,
            ),
        })
    }

    fn map_request(&self, req: j::SerializedRequest) -> crate::TransportResult<j::Response> {
        Ok(j::Response {
            id: req.id().clone(),
            payload: self
                .asserter
                .pop_response()
                .ok_or_else(|| crate::TransportErrorKind::custom_str("empty asserter response queue"))?,
        })
    }
}

impl std::ops::Deref for MockTransport {
    type Target = Asserter;

    fn deref(&self) -> &Self::Target {
        &self.asserter
    }
}

impl tower::Service<j::RequestPacket> for MockTransport {
    type Response = j::ResponsePacket;
    type Error = crate::TransportError;
    type Future = crate::TransportFut<'static>;

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: j::RequestPacket) -> Self::Future {
        Box::pin(self.clone().handle(req))
    }
}
