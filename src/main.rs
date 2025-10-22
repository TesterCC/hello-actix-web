use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use env_logger::Env; // 引入 env_logger

/*
ref: https://actix.rs/docs/getting-started/
curl http://127.0.0.1:9999/
curl http://127.0.0.1:9999/hey
curl http://127.0.0.1:9999/info/tester
curl -X POST http://127.0.0.1:9999/echo -d 'Hello, Test Actix Web Post Request!'
curl -X POST http://127.0.0.1:9999/echo -d "Hello, Test Actix Web Post Request\!"  Attention the testcase
curl -X POST http://127.0.0.1:9999/echo -d "Hello, Test Actix Web Post Request."
*/

#[get("/")]
async fn hello() -> impl Responder {
    println!("[D] Visit homepage"); // debug print
    HttpResponse::Ok().body("Hello, actix-web!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("[D] Received request body: {}", req_body); // debug print request body
    HttpResponse::Ok().body(req_body)
}

// ref https://knots.l0u0l.com/Rust/%E5%85%A5%E9%97%A8%E7%AC%94%E8%AE%B0/Actix%20Web.html
#[get("/info/{name}")]
async fn info(name: web::Path<String>) -> impl Responder {
    format!("Hello, here is user {} basic info!", &name)
}

// curl http://127.0.0.1:9999/info/tester

async fn manual_hello() -> impl Responder {
    println!("[D] Manual hello endpoint called"); // debug print
    HttpResponse::Ok().body("Hey there! It's working ...")
}

// 不使用路由宏的路由定义方法，手动注册 manual_hello
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 初始化 env_logger，设置日志级别为 debug
    // 在 main 函数中初始化 env_logger，并设置日志级别为 debug 或更低（如 trace），以查看详细的请求日志。
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    // env_logger 并不会自动记录 Actix Web 的请求和响应信息。
    // Actix Web 使用自己的日志中间件来记录请求和响应信息，因此你需要显式地启用 Actix Web 的 Logger 中间件。

    let mut _server_ip = "127.0.0.1";
    let mut _server_port: u16 = 9999;

    HttpServer::new(|| {
        App::new()
            // 启用 Logger 中间件, 这是 Actix Web 提供的一个默认日志中间件，用于记录请求和响应信息。
            // 默认格式为：%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T。
            // .wrap(Logger::default())
            // use custom log format
            .wrap(Logger::new("%a %{User-Agent}i %r %s %b")) // 自定义日志格式, it's better
            .service(hello)
            .service(echo)
            .service(info)   // 注意要注册，否则路由不会生效
            // different way to register route that do not use a routing macro，这个是手动注册方式
            .route("/hey", web::get().to(manual_hello))
    })
        // .bind(("127.0.0.1", 8080))?
        .bind((_server_ip, _server_port))?
        .run()
        .await
}
