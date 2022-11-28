use std::fmt::{Formatter, Display, Result };

pub struct Structure {pub value: i32}

impl Display for Structure {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}", self.value)
	}
}
#[derive(Debug)]
pub struct MinMax(pub i64, pub i64);

impl Display for MinMax {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "({}, {})", self.0, self.1)
	}
}

#[derive(Debug)]
pub struct Point2D {
	pub x: f64,
	pub y: f64,
}

impl Display for Point2D {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "x: {}, y: {}", self.x, self.y)
	}
}

#[derive(Debug)]
pub struct UnPrintable{ pub number: i32 }

#[derive(Debug)]
pub struct Person<'a> {
	pub name: &'a str,
	pub age: u8,
}

#[derive(Debug)]
pub struct Complex {
	pub real: f64,
	pub imag: f64,
}

impl Display for Complex {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{} + {}i", self.real, self.imag)
	}
}

pub struct List(pub Vec<i32>);

impl Display for List {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let vec = &self.0;

		write!(f, "[")?;

		for (count, v) in vec.iter().enumerate() {
			if count != 0 { write!(f, ", ")?; }
			write!(f, "{}: {}", count, v)?;
		}

		write!(f, "]")
	}
}

pub struct City {
	pub name: &'static str,
	pub lat: f32,
	pub lon: f32,
}

impl Display for City {
	fn fmt(&self, f: &mut Formatter) -> Result {
		let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
		let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

		write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
	}
}

#[derive(Debug)]
pub struct Color {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
}

