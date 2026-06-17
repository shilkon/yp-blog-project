use leptos::prelude::*;
use leptos_router::components::A;

use crate::api::client;
use blog_client::BlogClientTransport;

#[component]
pub fn Posts() -> impl IntoView {
    let resp = LocalResource::new(|| async {
        client().list_posts(10, 0).await.ok()
    });

    view! {
        <div>
            <h1>"Posts"</h1>

            <Suspense fallback=move || {
                view! { <p>"Loading..."</p> }
            }>
                {move || {
                    resp.get().map(|resp| {
                        match resp {
                            Some(resp) => {
                                view! {
                                    <ul>
                                        {
                                            resp.posts
                                                .into_iter()
                                                .map(|post| {
                                                    view! {
                                                        <li>
                                                            <A href=format!("/posts/{}", post.id)>
                                                                {post.title}
                                                            </A>
                                                        </li>
                                                    }
                                                })
                                                .collect_view()
                                        }
                                    </ul>
                                }.into_any()
                            }

                            None => {
                                view! {
                                    <p>"Failed to load posts"</p>
                                }.into_any()
                            }
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}