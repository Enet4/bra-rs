//! Buffered Random Access (BRA) provides easy random memory access to a
//! sequential source of data. This is achieved by greedily retaining all
//! memory read from a given source, or by keeping a way to reset the
//! source to the beginning for multiple passes.
//!
//! # Examples
//! 
//! [`GreedyAccessReader`] can be either used as a buffered reader or as
//! a random memory access descriptor. The amount of data read does
//! not influence the relative index of the data unless the method
//! [`clear`] is called.
//! 
//! [`GreedyAccessReader`]: ./struct.GreedyAccessReader.html
//! [`clear`]: ./struct.GreedyAccessReader.html#method.clear
//!
//! ```
//! # use bra::GreedyAccessReader;
//! # use std::io::Read;
//! # fn get_reader() -> impl Read {
//! #     std::io::repeat(1)
//! # }
//! # fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let reader = get_reader();
//! let mut reader = GreedyAccessReader::new(reader);
//! 
//! // random access to bytes!
//! let k: u8 = reader.get(12)?;
//! // random slicing!
//! let s: &[u8] = reader.slice(20..48)?;
//! assert_eq!(s.len(), 28);
//! // also functions as a buffered reader
//! let mut chunk = [0; 20];
//! reader.read_exact(&mut chunk)?;
//! # Ok(())
//! # }
//! # run().unwrap();
//! ```

mod greedy;
pub use greedy::GreedyAccessReader;
