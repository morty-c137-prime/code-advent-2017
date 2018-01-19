/**
 * @author Richard Alvarez
 * @problem http://adventofcode.com/2017/day/1
 */
#[allow(dead_code)]
#[allow(unused_variables)]

fn captcha(numbers: Vec<u32>) -> u32
{
	println!("{:?}", numbers);
	match numbers.len() 
	{
		0 => 0,
		1 => 0,
		_ => 
		{
			let (left, right) = numbers.split_at(1);
			if numbers[0] == numbers[1]
			{
				left[0] + captcha(right.to_vec())
			}
			else
			{
				captcha(right.to_vec())
			}
		}
	}
}

fn main()
{
	let numbers: Vec<u32> = 
	{
		let mut string_input = String::from("1111");
		let last = string_input.chars().nth(0).unwrap();
		string_input.push(last);
		let characters = string_input.chars();
		characters
			.map(|ch| ch.to_digit(10).unwrap()).collect()
	};

	let result = captcha(numbers);
	println!("The result is: {}", result);
}