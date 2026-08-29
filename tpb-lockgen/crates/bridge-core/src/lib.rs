#![forbid(unsafe_code)]

// Public dependency-only probe. No private TPB source is copied here.
// Trigger the offline toolchain export used only inside this repair session.
pub fn lockfile_probe() -> bool { true }
