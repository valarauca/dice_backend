
use std::slice::Iter;

#[derive(Default,Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct ReadTracking {
    // who reads from us
    sinks: Vec<u64>,
    // who we read from
    sources: Vec<u64>,
}
impl AsRef<ReadTracking> for ReadTracking {
    #[inline(always)]
    fn as_ref(&self) -> &ReadTracking {
        self
    }
}
impl AsMut<ReadTracking> for ReadTracking {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut ReadTracking {
        self
    }
}

/// ReadTrackingOp lets us be aware of how many expressions consume
/// our output
pub trait ReadTrackingOp: AsMut<ReadTracking> + AsRef<ReadTracking> {

    /// returns a copy of the read tracking structure
    fn get_copy(&self) -> ReadTracking {
        self.as_ref().clone()
    }

    /// append a sink (and expression which reads 'this' expression)
    fn add_sink(&mut self, sink: u64) {
        let already = self.as_mut().sinks.iter().map(|x| *x == sink).fold(false, |a,b| a | b);
        if !already {
            self.as_mut().sinks.push(sink);
        }
    }

    /// append a source (an expression which 'this' expression reads)
    fn add_source(&mut self, source: u64) {
        let already = self.as_mut().sources.iter().map(|x| *x == source).fold(false, |a,b| a | b);
        if !already {
            self.as_mut().sources.push(source);
        }
    }

    /// return the number of expressions which read this expression
    fn get_num_sinks(&self) -> usize {
        self.as_ref().sinks.len()
    }

    /// return the number of expressions which this expression reads
    fn get_num_sources(&self) -> usize {
        self.as_ref().sources.len()
    }
}
