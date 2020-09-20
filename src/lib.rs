//! `projection` provides you easy access struct fields of a Option<T>.
//!
//! # Examples
//!
//! To use `projection`, you first need to annotate your type with the `projection` attribute
//!
//! ```
//! use ::projection::projection;
//! #[projection]
//! struct AType {
//!     a: u32,
//!     b: u32
//! }
//! ```
//!
//! Then, you can use `project()` to access fields of `AType` from a `Option<AType>`
//!
//! ```
//! # use ::projection::projection;
//! # #[projection]
//! # struct AType {
//! #     a: u32,
//! #     b: u32
//! # }
//! use ::projection::prelude::*;
//!
//! let var: Option<AType> = None;
//! let var = var.project();
//! assert_eq!(var.a, None);
//!
//! let var = Some(AType { a: 1, b: 1 });
//! let var = var.project();
//! assert_eq!(var.a, Some(1));
//!
//! // You can choose to borrow the values
//! let mut var = Some(AType { a: 1, b: 1 });
//! let var = var.as_mut().project();
//! assert_eq!(var.a, Some(&mut 1));
//!
//! let var = Some(AType { a: 1, b: 1 });
//! let var = var.as_ref().project();
//! assert_eq!(var.a, Some(&1));
//! ```

pub use projection_macros::projection;

pub mod prelude {
	/// Implemented for `T` where a projection can be created for `Option<T>`
	pub trait OptionProjectable: Sized {
		type P;
		fn project(f: Option<Self>) -> Self::P;
	}
	#[doc(hidden)]
	pub trait ResultProjectable<E>: Sized {
		type P;
		fn project(f: Result<Self, E>) -> Self::P;
	}

	/// Provides `.project()` for `Option<T>`
	pub trait Projectable {
		type P;
		fn project(self) -> Self::P;
	}

	impl<T: OptionProjectable> Projectable for Option<T> {
		type P = <T as OptionProjectable>::P;
		fn project(self) -> <T as OptionProjectable>::P {
			OptionProjectable::project(self)
		}
	}

	impl<E, T: ResultProjectable<E>> Projectable for Result<T, E> {
		type P = <T as ResultProjectable<E>>::P;
		fn project(self) -> <T as ResultProjectable<E>>::P {
			ResultProjectable::project(self)
		}
	}
}

pub use crate::prelude::*;
