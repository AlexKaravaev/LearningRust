fn main() {
   	let _guess: u32 = "42".parse().expect("Not a number!");
   	/* But this: let guess = "42".parse().expect("Not a number!")
   	*  produces an error, because it cannot infer the type
   	*/

   	//INTEGER TYPES
   	/*
	 * Length	Signed	Unsigned
	 * 8-bit	i8	u8
	 * 16-bit	i16	u16
	 * 32-bit	i32	u32
	 * 64-bit	i64	u64
	 * arch	isize	usize
   	*/

    let _x = 2.0; // f64

    let _y: f32 = 3.0 // f32




}
