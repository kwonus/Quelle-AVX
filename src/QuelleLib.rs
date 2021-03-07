use std::ptr;

type quelle_alloc= fn(u16) -> &mut *const u8;

#[no_mangle]
pub extern "C" unsafe fn search(request: &*const u8, size: u16, allocator: quelle_alloc) -> *const u8 {
    &mut mem = quelle_alloc(13+6);
    ptr::copy(&"Hello from search!\0".as_ptr(), mem.as_ptr(), 13+6);
    return mem
}

#[no_mangle]
pub extern "C" unsafe fn fetch(request: &*const u8, size: u16, allocator: quelle_alloc) -> *const u8 {
    &mut mem = quelle_alloc(13+5);
    ptr::copy(&"Hello from fetch!\0".as_ptr(), mem.as_ptr(), 13+6);
    return mem}

#[no_mangle]
pub extern "C" unsafe fn page(request: &*const u8, size: u16, allocator: quelle_alloc) -> *const u8 {
    &mut mem = quelle_alloc(13+4);
    ptr::copy(&"Hello from page!\0".as_ptr(), mem.as_ptr(), 13+6);
    return mem}

#[no_mangle]
pub extern "C" unsafe fn status(request: &*const u8, size: u16, allocator: quelle_alloc) -> *const u8 {
    &mut mem = quelle_alloc(13+5);
    ptr::copy(&"Hello from status!\0".as_ptr(), mem.as_ptr(), 13+6);
    return mem}
