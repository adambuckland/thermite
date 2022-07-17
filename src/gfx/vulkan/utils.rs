#[cfg(feature = "ash-molten")]
pub fn create_entry() -> ash::Entry {
    ash_molten::load()
}

#[cfg(not(feature = "ash-molten"))]
pub fn create_entry() -> ash::Entry {
    unsafe { ash::Entry::load().unwrap() }
}
