
#[cfg(test)]
mod tests {
    use crate::user::*;
    use actix_web::{test::{self, TestRequest}, App};
    use serde_json::json;

    #[actix_rt::test]
    async fn test_user() {
        crate::test::init();

        let request_body = json!({
            "email": "tore@cloudmaker.dev",
            "password": "test",
        });

        let mut app = test::init_service(App::new().configure(init_routes)).await;

        let resp = TestRequest::post().uri("/users").set_json(&request_body).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to create user");
        let user: User = test::read_body_json(resp).await;

        // let resp = TestRequest::post().uri("/users").set_json(&request_body).send_request(&mut app).await;
        // assert!(resp.status().is_client_error(), "Should not be possible to create user with same email twice");

        let resp = TestRequest::get().uri(&format!("/users/{}", user.id)).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to find user");

        let user: User = test::read_body_json(resp).await;
        assert_eq!(user.email, "tore@cloudmaker.dev", "Found wrong user");

        let request_body = json!({
            "email": "tore@cloudmaker.dev",
            "password": "new",
        });

        let resp = TestRequest::put().uri(&format!("/users/{}", user.id)).set_json(&request_body).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to update user");

        let user: User = test::read_body_json(resp).await;
        assert_eq!("new", user.password, "Failed to change password for user");

        let resp = TestRequest::delete().uri(&format!("/users/{}", user.id)).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to delete user");

        let resp = TestRequest::get().uri(&format!("/users/{}", user.id)).send_request(&mut app).await;
        assert!(resp.status().is_client_error(), "It should not be possible to find the user after deletion");
    }
}

