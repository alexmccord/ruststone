use crate::{Redstone, RedstoneDust, RedstoneTorch};

trait RedstoneConnection {
    fn add_outgoing_edge(&mut self, outgoing: &Redstone);
    fn add_incoming_edge(&mut self, incoming: &Redstone);
}

pub trait RedstoneLinking {
    fn link(&self, there: &Redstone);
}

impl RedstoneLinking for Redstone {
    fn link(&self, there: &Redstone) {
        match self {
            Redstone::Torch(torch) => torch.borrow_mut().add_outgoing_edge(&there),
            Redstone::Dust(dust) => dust.borrow_mut().add_outgoing_edge(&there),
        }

        match there {
            Redstone::Torch(torch) => torch.borrow_mut().add_incoming_edge(self),
            Redstone::Dust(dust) => dust.borrow_mut().add_incoming_edge(self),
        }

        match (self, there) {
            (Redstone::Dust(dust_here), Redstone::Dust(dust_there)) => {
                dust_here.borrow_mut().add_incoming_edge(&there);
                dust_there.borrow_mut().add_outgoing_edge(self);
            }
            _ => (),
        }
    }
}

impl RedstoneConnection for RedstoneTorch {
    fn add_outgoing_edge(&mut self, outgoing: &Redstone) {
        self.outgoing.push(outgoing.clone());
    }

    fn add_incoming_edge(&mut self, incoming: &Redstone) {
        self.incoming = Some(incoming.clone());
    }
}

impl RedstoneConnection for RedstoneDust {
    fn add_outgoing_edge(&mut self, outgoing: &Redstone) {
        self.outgoing.push(outgoing.clone());
    }

    fn add_incoming_edge(&mut self, incoming: &Redstone) {
        self.incoming.push(incoming.clone());
    }
}
