#![allow(dead_code)]

use blog_client::*;
use wasm_bindgen::prelude::*;
use serde_json;

static SERVER_URL: &'static str = "http://localhost:8080/api";

#[wasm_bindgen]
struct BlogApp {
    client: Transport
}

#[wasm_bindgen]
impl BlogApp {
    #[wasm_bindgen]
    pub fn new() -> Self {
        Self { client: Transport::Http(HttpClient::new(SERVER_URL.to_string()).unwrap()) }
    }

    #[wasm_bindgen]
    pub async fn register(&mut self, username: String, email: String, password: String) -> Result<JsValue, JsValue> {
        match self.client.register(username, email, password).await {
            Ok(auth_response) => {
                self.save_token(&auth_response.token);
                self.save_user(&auth_response.user);
                
                web_sys::console::log_1(&format!("Registered user: {:?}", auth_response.user).into());
                web_sys::console::log_1(&format!("Token: {}", auth_response.token).into());

                serde_wasm_bindgen::to_value(&auth_response)
                    .map_err(|e| JsValue::from_str(&e.to_string()))
            },
            Err(e) => {
                let error_msg = format!("Registration error: {e}");
                web_sys::console::error_1(&error_msg.clone().into());
                
                Err(JsValue::from_str(&error_msg))
            }
        }
    }

    #[wasm_bindgen]
    pub async fn login(&mut self, username: String, password: String) -> Result<JsValue, JsValue> {
        match self.client.login(username, password).await {
            Ok(auth_response) => {
                self.save_token(&auth_response.token);
                self.save_user(&auth_response.user);

                web_sys::console::log_1(&format!("Logged in user: {:?}", auth_response.user).into());
                web_sys::console::log_1(&format!("Token: {}", auth_response.token).into());

                serde_wasm_bindgen::to_value(&auth_response)
                    .map_err(|e| JsValue::from_str(&e.to_string()))
            },
            Err(e) => {
                let error_msg = format!("Login error: {e}");
                web_sys::console::error_1(&error_msg.clone().into());

                Err(JsValue::from_str(&error_msg))
            }
        }
    }

    #[wasm_bindgen]
    pub fn logout(&self) -> Result<(), JsValue> {
        let storage = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap();

        storage.remove_item("token")
            .map_err(|e| JsValue::from(e))?;

        storage.remove_item("user")
            .map_err(|e| JsValue::from(e))?;

        Ok(())
    }

    #[wasm_bindgen]
    pub fn current_user(&self) -> Result<JsValue, JsValue> {
        match self.load_user() {
            Some(user) => serde_wasm_bindgen::to_value(&user)
                .map_err(|e| JsValue::from_str(&e.to_string())),
            None => Ok(JsValue::NULL),
        }
    }

    #[wasm_bindgen]
    pub async fn get_post(&mut self, id: String) -> Result<JsValue, JsValue> {
        let post_id = uuid::Uuid::parse_str(&id).map_err(|e| JsValue::from_str(&e.to_string()))?;

        match self.client.get_post(post_id).await {
            Ok(post) => serde_wasm_bindgen::to_value(&post)
                .map_err(|e| JsValue::from_str(&e.to_string())),
            Err(e) => {
                let error_msg = format!("Get post error: {e}");
                web_sys::console::error_1(&error_msg.clone().into());

                Err(JsValue::from_str(&error_msg))
            }
        }
    }

    #[wasm_bindgen]
    pub async fn list_posts(&mut self, limit: i64, offset: i64) -> Result<JsValue, JsValue> {
        match self.client.list_posts(limit, offset).await {
            Ok(posts_response) => serde_wasm_bindgen::to_value(&posts_response)
                .map_err(|e| JsValue::from_str(&e.to_string())),
            Err(e) => {
                let error_msg = format!("List posts error: {e}");
                web_sys::console::error_1(&error_msg.clone().into());

                Err(JsValue::from_str(&error_msg))
            }
        }
    }

    #[wasm_bindgen]
    pub async fn create_post(&mut self, title: String, content: String) -> Result<JsValue, JsValue> {
        match self.client.create_post(title, content).await {
            Ok(post) => serde_wasm_bindgen::to_value(&post)
                .map_err(|e| JsValue::from_str(&e.to_string())),
            Err(e) => {
                let error_msg = format!("Create post error: {e}");
                web_sys::console::error_1(&error_msg.clone().into());

                Err(JsValue::from_str(&error_msg))
            }
        }
    }

    #[wasm_bindgen]
    pub async fn update_post(&mut self, id: String, title: String, content: String) -> Result<JsValue, JsValue> {
        let post_id = uuid::Uuid::parse_str(&id).map_err(|e| JsValue::from_str(&e.to_string()))?;

        match self.client.update_post(post_id, title, content).await {
            Ok(post) => serde_wasm_bindgen::to_value(&post)
                .map_err(|e| JsValue::from_str(&e.to_string())),
            Err(e) => {
                let error_msg = format!("Update post error: {e}");
                web_sys::console::error_1(&error_msg.clone().into());

                Err(JsValue::from_str(&error_msg))
            }
        }
    }

    #[wasm_bindgen]
    pub async fn delete_post(&mut self, id: String) -> Result<(), JsValue> {
        let post_id = uuid::Uuid::parse_str(&id).map_err(|e| JsValue::from_str(&e.to_string()))?;

        match self.client.delete_post(post_id).await {
            Ok(_) => Ok(()),
            Err(e) => {
                let error_msg = format!("Delete post error: {e}");
                web_sys::console::error_1(&error_msg.clone().into());

                Err(JsValue::from_str(&error_msg))
            }
        }
    }

    #[wasm_bindgen]
    pub fn is_authenticated(&self) -> Result<JsValue, JsValue> {
        Ok(JsValue::from_bool(self.load_token().is_some()))
    }
}

impl BlogApp {
    fn save_token(&self, token: &str) {
        web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .set_item("token", token)
            .unwrap();
    }

    fn load_token(&self) -> Option<String> {
        web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .get_item("token")
            .unwrap()
    }

    fn save_user(&self, user: &blog_client::User) {
        let json = serde_json::to_string(user).unwrap();

        web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap()
            .set_item("user", &json)
            .unwrap();
    }

    fn load_user(&self) -> Option<User> {
        let storage = web_sys::window()
            .unwrap()
            .local_storage()
            .unwrap()
            .unwrap();

        let value = storage.get_item("user")
            .unwrap()?;

        serde_json::from_str(&value).ok()
    }
}
