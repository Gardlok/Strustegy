//! Allocation-free route recognition over proven path segments.

use super::types::{Method, ProvenRequest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route<'input> {
    Projects,
    Project { name: &'input str },
    Health,
    Unknown { path: &'input str },
}

pub fn recognize(request: ProvenRequest<'_>) -> Route<'_> {
    let mut segments = request.segments().iter();
    let first = segments.next();
    let second = segments.next();
    let extra = segments.next();

    match (request.method(), first, second, extra) {
        (Method::Get, Some("projects"), None, None) => Route::Projects,
        (Method::Get, Some("projects"), Some(name), None) => Route::Project { name },
        (Method::Get, Some("health"), None, None) => Route::Health,
        _ => Route::Unknown {
            path: request.path().as_str(),
        },
    }
}

pub fn dispatch(route: Route<'_>) {
    match route {
        Route::Projects => println!("dispatch: list projects"),
        Route::Project { name } => println!("dispatch: load project {name:?}"),
        Route::Health => println!("dispatch: report service health"),
        Route::Unknown { path } => println!("dispatch: no route for {path:?}"),
    }
}
