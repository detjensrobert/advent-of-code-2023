pub fn smash(pair: (u32, u32)) -> u64 {
  // u64 is huge (comparatively) so put both coords into one u64
  let high = (pair.0 as u64) << 32;
  let low = pair.1;

  return high | low as u64;
}

pub fn unsmash(smashed: u64) -> (u32, u32) {
  return ((smashed >> 32) as u32, smashed as u32)
}


pub fn from_usize(pair: (usize, usize)) -> u64 {
  assert!(pair.0 <= u32::MAX as usize);
  assert!(pair.1 <= u32::MAX as usize);

  smash((pair.0 as u32, pair.1 as u32))
}
