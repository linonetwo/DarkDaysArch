pub fn default_false() -> bool {
  false
}

pub fn is_default_false(t: &bool) -> bool {
  t == &default_false()
}

pub fn default_true() -> bool {
  true
}

pub fn is_default_true(t: &bool) -> bool {
  t == &default_true()
}
