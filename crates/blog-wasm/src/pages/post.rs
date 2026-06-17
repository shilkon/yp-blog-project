use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;

use crate::api::client;
use blog_client::BlogClientTransport;

#[component]
pub fn PostPage() -> impl IntoView {
    let params = use_params_map();

    let post = LocalResource::new(move || async move {
        let id = params.get().get("id")?.to_string();

        let uuid = Uuid::parse_str(&id).ok()?;

        client().get_post(uuid).await.ok()
    });

    view! {
        <div>
            <Suspense fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                {move || {
                    post.get().map(|post| {
                        match post {
                            Some(post) => {
                                view! {
                                    <article>
                                        <h1>{post.title}</h1>

                                        <p>{post.content}</p>
                                    </article>
                                }.into_any()
                            }

                            None => {
                                view! {
                                    <p>"Post not found"</p>
                                }.into_any()
                            }
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}