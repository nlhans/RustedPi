extern crate native;
extern crate libc;


use std::mem;
use std::rt::rtio::{Open, ReadWrite};
use std::os::{MemoryMap, MapReadable, MapWritable, MapFd, MapOffset, MapNonStandardFlags};

fn main() {

	let memory_addr_ph = 0x20000000 + 0x00200000;

	let memory_path = "/dev/mem".to_c_str();
	let memory_file = match native::io::file::open(&memory_path, Open, ReadWrite) {
	    Ok(f) => f,
	    Err(e) => {
			println!("ERROR open {} ", e);
			panic!("file error: {}", e)
		}
	};
	let memory_map = match MemoryMap::new(4096u, [MapReadable, MapWritable, MapFd(memory_file.fd()), MapOffset(memory_addr_ph), MapNonStandardFlags(0x01)]) {
	    Ok(f) => f,
	    Err(e) => {
			println!("ERROR mmap {} ", e);
			panic!("could not map memory: {}", e)
		}
	};
	
	unsafe
	{
		let memory_ptr: *mut uint = mem::transmute(memory_map.data()); // as *mut u32;

		let gpfsel0 = memory_ptr;
		let gpfsel1 = memory_ptr.offset(1);
		let gpfsel2 = memory_ptr.offset(2);
		let gpfsel3 = memory_ptr.offset(3);
		let gpfsel4 = memory_ptr.offset(4);
		let gpfsel5 = memory_ptr.offset(5);
							// 6 = reserved
		let gpset0  = memory_ptr.offset(7);
		let gpset1  = memory_ptr.offset(8);
							// 9 = reserved
		let gpclr0  = memory_ptr.offset(10);
		let gpclr1  = memory_ptr.offset(11);
							// 12 = reserved
		let gplev0  = memory_ptr.offset(13);
		let gplev1  = memory_ptr.offset(14);

		// Set GPIO24 pinmode as output
		*gpfsel2 &= !(7u << 12u);
		*gpfsel2 |= 1u << 12u;

		// Write GPIO24 high and low
		loop {
			*gpset0 = 1u << 24u;
			*gpclr0 = 1u << 24u;
		}

		for i in range(0u, 32) {
			let k = i;
			let b0 = *(memory_ptr.offset((k + 0) as int));
			let p = memory_ptr.offset(k as int).to_uint();			
			println!("{:X} {} = {:X}", p, i, b0);
		}
	}

	println!("All your memory are belong to us. v2");
}


