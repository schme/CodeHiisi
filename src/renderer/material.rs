use std::sync::atomic::{AtomicUsize, Ordering};
use std::collections::HashMap;

static MATERIAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
