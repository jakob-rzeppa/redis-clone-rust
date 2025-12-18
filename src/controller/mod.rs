mod get;
mod set;
mod insert;
mod remove;

use crate::connection::Request;
use get::handle_get_request;
use insert::handle_insert_request;
use remove::handle_remove_request;
use set::handle_set_request;

// The '_ tells rust, that request has some lifetime ('a) that is used for the &[u8].
// This makes sure that the &[u8] is valid for the entirety of the function.
// '_ is a shortform for route_request<'a>(request: Request<'a>)
pub(crate) async fn route_request(request: Request<'_>) {
    match request {
        Request::Get { .. } => handle_get_request(request),
        Request::Set { .. } => handle_set_request(request),
        Request::Insert { .. } => handle_insert_request(request),
        Request::Remove { .. } => handle_remove_request(request),
    }
}