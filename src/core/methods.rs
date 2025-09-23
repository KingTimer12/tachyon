pub enum Method {
  Get(u8),
  Post(u8),
  Put(u8),
  Delete(u8),
  Patch(u8),
}

impl Method {
  pub fn new(id: u8) -> Self {
    match id {
      0 => Method::Get(id),
      1 => Method::Post(id),
      2 => Method::Put(id),
      3 => Method::Delete(id),
      4 => Method::Patch(id),
      _ => panic!("Invalid method ID"),
    }
  }

  pub fn id(&self) -> u8 {
    match self {
      Method::Get(id) => *id,
      Method::Post(id) => *id,
      Method::Put(id) => *id,
      Method::Delete(id) => *id,
      Method::Patch(id) => *id,
    }
  }
}

impl PartialEq for Method {
  fn eq(&self, other: &Self) -> bool {
    self.id() == other.id()
  }
}

impl From<&hyper::Method> for Method {
  fn from(method: &hyper::Method) -> Self {
    match *method {
      hyper::Method::GET => Method::Get(0),
      hyper::Method::POST => Method::Post(1),
      hyper::Method::PUT => Method::Put(2),
      hyper::Method::DELETE => Method::Delete(3),
      hyper::Method::PATCH => Method::Patch(4),
      _ => panic!("Invalid method"),
    }
  }
}

impl From<hyper::Method> for Method {
  fn from(method: hyper::Method) -> Self {
    match method {
      hyper::Method::GET => Method::Get(0),
      hyper::Method::POST => Method::Post(1),
      hyper::Method::PUT => Method::Put(2),
      hyper::Method::DELETE => Method::Delete(3),
      hyper::Method::PATCH => Method::Patch(4),
      _ => panic!("Invalid method"),
    }
  }
}
