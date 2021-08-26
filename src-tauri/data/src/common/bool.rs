pub fn default_bool_false() -> bool {
  false
}

pub fn is_default_bool_false(t: &bool) -> bool {
  t == &default_bool_false()
}

pub fn default_bool_true() -> bool {
  true
}

pub fn is_default_bool_true(t: &bool) -> bool {
  t == &default_bool_true()
}
