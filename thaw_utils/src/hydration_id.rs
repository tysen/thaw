/// Returns a stable id that is identical between the SSR render and the
/// matching client hydration pass, and unique within a CSR-only session.
///
/// Use this anywhere thaw wires a generated id into the DOM (`id=`, `for=`,
/// `aria-labelledby=`, `aria-controls=`, `name=`, `data-thaw-id=`, etc.).
/// Do **not** use `uuid::Uuid::new_v4()` for that — the component body runs
/// independently on server and client and would produce divergent ids,
/// triggering unrecoverable hydration errors.
#[cfg(feature = "csr")]
pub fn next_stable_id() -> String {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    format!("thaw-{}", COUNTER.fetch_add(1, Ordering::Relaxed))
}

#[cfg(not(feature = "csr"))]
pub fn next_stable_id() -> String {
    use leptos::prelude::Owner;
    let id = Owner::current_shared_context()
        .map(|sc| sc.next_id().into_inner())
        .unwrap_or(0);
    format!("thaw-{id}")
}
