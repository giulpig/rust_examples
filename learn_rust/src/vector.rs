

pub fn run(){

	println!("Testing collections");

	{
		// Create an empty vector - I need to specify the type
		let mut v: Vec<i32> = Vec::new();

		// Add elements - v must be mutable
		v.push(4);
	
	// Here destructor gets called
	}

	// Macro to create a vector - rust can infer the type
	let mut v = vec![1, 2, 3];

	// Unsafe get
	let third: &i32 = &v[2];
	
	// Safe get, should be used when risking access to O.OfB.
	let third: Option<&i32> = v.get(2);
	match third {
		Some(third) => {
			println!("The third element is {}", third);
		},
		None => println!("index {} out of bounds", 2)
	}

	// I can't do this! I have active references (even if are un-mutable refs)
	//v.push(4);

	for i in &mut v {
		*i = *i + 5;
	}

	for i in &v {
		println!("{}", i);
	}


	/// Arrays with multiple data types - just use an enum

	enum VariousTypes {
		Int(i32),
		Ano(f64),
	}

	let array_misto = vec![VariousTypes::Ano(4.0), VariousTypes::Int(4)];

}

