use downcast_rs::Downcast;

pub trait Error<C>: Downcast {
  fn build_context(self: Box<Self>) -> C;
}

impl_downcast!(Error<C>);
