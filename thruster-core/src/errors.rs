pub use thruster_core_async_await::errors::Error;
use crate::context::Context;

#[derive(Debug)]
pub struct ThrusterError<C> {
  pub context: C,
  pub message: String,
  pub status: u32
}

impl<C: Context + 'static> Error<C> for ThrusterError<C> {
  fn build_context(self: Box<Self>) -> C {
    let mut context = self.context;

    context.set_body(format!("{{\"message\": \"{}\",\"success\":false}}", self.message).as_bytes().to_vec());
    context
  }
}
