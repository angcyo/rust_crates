///
/// @author <a href="mailto: angcyo@126.com">angcyo</a> \
/// @date 2025/06/07
///

/// 使用 `axum` 库封装的一些网络服务相关方法

#[cfg(test)]
mod tests {
    use axum::extract::Request;
    use axum::response::Redirect;
    use axum::routing::get;
    use axum::Router;

    /// 启动一个Web服务端
    #[tokio::test]
    async fn test_web_server() {
        // build our application with a route
        let app = Router::new()
            // `GET /` goes to `root`
            .route("/", get(root))
            .route("/redirect", get(redirect_handler));

        // run our app with hyper, listening globally on port 28080
        let listener = tokio::net::TcpListener::bind("0.0.0.0:28080")
            .await
            .unwrap();
        println!("Server listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, app).await.unwrap();
        println!("...end");
    }

    // 根路由处理器
    async fn root(req: Request) -> String {
        format!("Hello From Axum by rust.\n{req:?}")
    }

    // 重定向处理器
    async fn redirect_handler() -> Redirect {
        // 重定向到根 URL
        Redirect::temporary("https://www.baidu.com?angcyo")
    }
}
