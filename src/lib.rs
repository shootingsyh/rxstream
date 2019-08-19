#![feature(type_alias_impl_trait)]
/// A library which provide similar functionality of rxjs in rust.
/// See usage in https://rxjs-dev.firebaseapp.com/guide/operators
/// It supports most of the operators in rxjs. But due to language difference,
/// a couple of things need to notice
/// 0. Observables are implemented as Stream in rust. Currently I'm using
/// Stream in futures 0.1. And many operators are just alias of the ones in 
/// Stream library.
/// 1. Creation operators in rxjs doc is put to module source.
/// 2. Ajax and event based creation are not supported
/// 3. For combining operators such as merge and concat, currently this 
/// library only support two operands. I'm still considering how to support 
/// random number of operands. There are many ways to do so, but none is 
/// perfect. One way is to use Vec, which requires all operands are with 
/// same type. Another way is to use Vec<Box<>>, which requires allocation.
/// A third way is to use hlist with some macro. The last one is most 
/// promising one, but I'm still evaluating. 
/// One way to mitigate the problem is to do the operation multiple times. 
/// Say merge(merge(s1, s2), s3) etc. 


#[macro_use]
extern crate futures;

pub mod source;
pub mod operators;