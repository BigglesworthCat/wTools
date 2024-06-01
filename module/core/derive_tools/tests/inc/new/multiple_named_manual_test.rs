use super::*;

#[ derive( Debug, PartialEq, Eq ) ]
struct StructNamedFields
{
  a: i32,
  b: bool,
}

impl StructNamedFields
{
  #[ inline( always ) ]
  fn new( src : ( i32, bool ) ) -> Self
  {
    Self{ a: src.0, b: src.1 }
  }
}

include!( "./only_test/multiple_named.rs" );
