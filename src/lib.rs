#[macro_use]
extern crate lazy_static;

extern crate libc;
extern crate kernel32;
extern crate winapi;

use std::sync::RwLock;

static doVerify : bool = true;
static ZeroInitialized : u32 = 0x00000008;
static NoSerialization : u32 = 0x00000001;
static NoFlag : u32  = 0x00000000;
static null_ptr : usize = 0x00000000;

lazy_static!{
    static ref heapg : Heap =  Heap::new(768);
}

struct Heap {
    heap_pointer: winapi::HANDLE,
    size : RwLock<u32>
}

unsafe impl Sync for Heap{}
unsafe impl Send for Heap{}

impl Heap { 
    fn new(size : u32) -> Heap{
        let mut ch;
        unsafe{ch = kernel32::GetProcessHeap()};
        Heap{heap_pointer : ch,  size : RwLock::new(size)}
    }


    fn SetHeapSize(&self, size : u32){
        *self.size.write().unwrap() = size
    }

    fn GetHeapProcess(&self) -> winapi::HANDLE{
        return self.heap_pointer;
    }

    fn IsHeapValid(&self, pointer  : *const std::os::raw::c_void) -> i32{
        let mut valid;
        unsafe{valid = kernel32::HeapValidate(self.heap_pointer ,NoFlag,pointer);}
        return valid; 
    }

    fn Allocate(&self, size : usize) -> *const std::os::raw::c_void{
        let pointer;
        unsafe{ pointer = kernel32::HeapAlloc(self.heap_pointer, ZeroInitialized, size as u32);}
        return pointer;
    }

    fn Reallocate(&self,size : usize , pointe : *mut std::os::raw::c_void) -> *const std::os::raw::c_void{
        let pointer;
        unsafe{ pointer = kernel32::HeapReAlloc(self.heap_pointer, ZeroInitialized, pointe ,size as u32);}
        return pointer;
    }

    fn GetHeapSize(&self) -> u32{
        let mut valid;
        unsafe{valid = kernel32::HeapSize(self.heap_pointer, NoFlag, 0 as *const std::os::raw::c_void);}
        return valid;
    }

    fn FreeHeap(&self, obj : *mut std::os::raw::c_void) -> (){
        unsafe{kernel32::HeapFree(self.heap_pointer, NoFlag ,obj);}
    }
}

#[no_mangle]
pub extern "C"  fn obAlloc(size : usize) -> *const std::os::raw::c_void{
    if size == 0 {
        return 0x00000000 as *const std::os::raw::c_void; //Let the program handle. If we got at this  maybe its better to just crash
    }
    heapg.Allocate(size)
}


#[no_mangle]
pub extern "C" fn obFree(allocObj : *mut std::os::raw::c_void) -> (){
    if(allocObj as usize == 0){return;}
    let  valid = heapg.IsHeapValid(allocObj);
    if(valid != 0) {heapg.FreeHeap(allocObj);}
}



#[no_mangle]
pub extern "C" fn obSize() -> usize {
    heapg.GetHeapSize() as usize
}


#[no_mangle]
pub extern "C"  fn obRealloc(point : *mut std::os::raw::c_void, size : usize) -> *const std::os::raw::c_void{
    if size == 0 {
        return 0x00000000 as *const std::os::raw::c_void;
    }
    let realloc = heapg.IsHeapValid(point);
    if(realloc != 0){
        heapg.Reallocate(size, point)
    }
    else{
        heapg.Allocate(size)
    }
}



#[no_mangle]
pub extern "C" fn obIsInHeap(validate : *const std::os::raw::c_void) -> bool{
    let valid = heapg.IsHeapValid(validate); 
    if(valid == 0) {return false;}
    else  {return true;}
}

#[no_mangle]
pub extern "C" fn obInit(size: u32, log: *const std::os::raw::c_char) -> bool{
    heapg.SetHeapSize(size);
    return true;
}

