use ::projection::projection;
#[projection]
struct Test {
	a: u32,
	b: u32,
}

#[test]
fn test1() {
	use ::projection::Projectable;
	let mut a = Some(Test { a: 1, b: 1 });
	{
		let b = a.as_ref().project();
		assert_eq!(b.a, Some(&1));
	}
	{
		let mut b = a.as_mut().project();
		assert_eq!(b.a, Some(&mut 1));
		b.a.as_mut().map(|t| **t = 2);
		assert_eq!(b.a, Some(&mut 2));
	}

	let b= a.project();
	assert_eq!(b.a, Some(2));
}
