pub trait Generator {
    fn generate(&self);
}

pub struct HTMLGenerator {
}

pub struct CopyGenerator {
}

pub trait Collector {
}

pub struct MDCollector {
}

pub struct DirCollector {
}

pub struct Watcher {
}

pub struct Router {
}
