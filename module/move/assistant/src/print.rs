
use super::*;
use core::fmt;
use former::Former;

//=

/// Struct to hold options to print data as table.
#[ derive( Debug, Default, Former ) ]
pub struct Styles
{
  /// Delimiter for separating table columns.
  pub separator : String,
}

/// Struct for formatting tables.
pub struct Context< 'a >
{
  buf : &'a mut dyn fmt::Write,
  styles : Styles,
}

impl< 'a > Context< 'a >
{
  /// Just constructr.
  pub fn new( buf : &'a mut dyn fmt::Write, styles : Styles ) -> Self
  {
    Self { buf, styles }
  }
}

impl fmt::Debug for Context< '_ >
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f
    .debug_struct( "Context" )
    .field( "buf", &"dyn fmt::Write" )
    .field( "styles", &self.styles )
    .finish()
  }
}

/// A trait for converting tables to a string representation.
pub trait TableToString< 'a >
{
  /// Converts the table to a string representation.
  ///
  /// # Returns
  ///
  /// A `String` containing the formatted table.
  fn table_to_string( &'a self ) -> String;
}

impl< 'a, T > TableToString< 'a > for T
where
  T : TableFormatter< 'a >
{
  fn table_to_string( &'a self ) -> String
  {
    let mut output = String::new();
    let mut formatter = Context
    {
      buf : &mut output,
      styles : Styles::default(),
    };
    T::fmt( self, &mut formatter ).expect( "Formatting failed" );
    output
  }
}

/// A trait for formatting tables.
///
/// This trait defines a method for formatting tables, allowing implementations
/// to specify how a table should be formatted and displayed.
///

pub trait TableFormatter< 'b >
{
  /// Formats the table and writes the result to the given formatter.
  fn fmt< 'a >( &'b self, f : &mut Context< 'a > ) -> fmt::Result;
}

/// A trait for formatting tables.
impl< 'a, T, RowKey, Row, CellKey, Cell, Title > TableFormatter< 'a >
for AsTable< 'a, T, RowKey, Row, CellKey, Cell, Title >
where
  Self : TableRows< 'a, RowKey, Row, CellKey, Cell >,
  Self : TableHeader< 'a, CellKey, Title >,
  Self : TableSize< 'a >,
  Row : Clone + for< 'cell > Cells< 'cell, CellKey, Cell > + 'a,
  Title : fmt::Debug,
  Cell : fmt::Debug + Clone + 'a,
  CellKey : fmt::Debug + Clone,
{
  fn fmt( &'a self, f : &mut Context< '_ > ) -> fmt::Result
  {
    let table_size = self.table_size();
    let mut col_widths : Vec< usize > = vec![ 0 ; table_size[ 1 ] ];
    let separator = &f.styles.separator;

    // Write the header if provided
    if let Some( header ) = self.header()
    {
      let mut first = true;
      let mut i = 0;
      for ( _key, title ) in header
      {
        if !first
        {
          write!( f.buf, "{}", separator )?;
        }
        col_widths[ i ] = format!( "{:?}", title ).len();
        // zzz : avoid extra allocation of memory
        write!( f.buf, "{:?}", title )?;
        first = false;
        i += 1;
      }
      writeln!( f.buf )?;
    }

    // Collect rows
    let mut all_rows : Vec< Vec< String > > = Vec::new();
    for row in self.rows()
    {
      let fields : Vec< String > = row
      .cells()
      .map( | ( _key, cell ) | format!( "{:?}", &cell ) )
      .collect();
      all_rows.push( fields );
    }

    for row in &all_rows
    {
      for ( i, cell ) in row.iter().enumerate()
      {
        if col_widths.len() <= i
        {
          col_widths.push( cell.len() );
        }
        else if cell.len() > col_widths[ i ]
        {
          col_widths[ i ] = cell.len();
        }
      }
    }

    // Write rows with proper alignment
    for row in all_rows
    {
      let formatted_row : Vec< String > = row
      .iter()
      .enumerate()
      .map( | ( i, cell ) | format!( "{:width$}", cell, width = col_widths[ i ] ) )
      .collect();
      writeln!( f.buf, "{}", formatted_row.join( separator ) )?;
    }

    Ok(())
  }
}
