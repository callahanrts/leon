//
// Parsing Model
//
// Network
//   \/
// Byte stream decoder
//   \/
// Input stream processor <------------------+
//   \/                    document.write()  /\
// Tokenizer                                 /\
//   \/                                      /\
// Tree construction  ---------------> Script execution
//   \/
//  DOM
//
//
// Input byte stream
// http://w3c.github.io/html/syntax.html#the-input-byte-stream
// This will likely be part of the network module.
pub mod tokenizer;
pub mod parser;
