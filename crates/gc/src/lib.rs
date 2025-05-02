#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![expect(clippy::cast_ptr_alignment)]

mod gc;
mod heap;

pub use self::gc::{Gc, Trace, Tracer};
