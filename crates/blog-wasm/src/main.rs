mod api;
mod pages;

use leptos::prelude::*;
use leptos_router::{
    components::*,
    path,
};

use pages::*;

fn main() {
    mount_to_body(|| {
        view! {
            <Router>
                <Routes fallback=|| {
                    view! {
                        <p>"Not Found"</p>
                    }
                }>

                    <Route
                        path=path!("/posts")
                        view=Posts
                    />

                    <Route
                        path=path!("/posts/:id")
                        view=PostPage
                    />

                </Routes>
            </Router>
        }
    });
}