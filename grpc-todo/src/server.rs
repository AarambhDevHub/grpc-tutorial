use tonic::{transport::Server, Request, Response, Status};
use tower_http::cors::{Any, CorsLayer};
use crate::db::{self, create_todo, delete_todo, get_todos, init_db, update_todo};
use todo::todo_service_server::{TodoService, TodoServiceServer};
use todo::{CreateTodoRequest, UpdateTodoRequest, DeleteTodoRequest, TodoList, Empty};

mod todo {
    tonic::include_proto!("todo");
}

#[derive(Debug)]
pub struct MyTodoService {
    pool: sqlx::SqlitePool,
}

impl From<db::Todo> for todo::Todo {
    fn from(todo: db::Todo) -> Self {
        todo::Todo {
            id: todo.id,
            title: todo.title,
            completed: todo.completed,
        }
    }
}


#[tonic::async_trait]
impl TodoService for MyTodoService {
    async fn create_todo(
        &self,
        request: Request<CreateTodoRequest>,
    ) -> Result<Response<todo::Todo>, Status> {
        let req = request.into_inner();
        let todo = create_todo(&self.pool, &req.title).await;
        Ok(Response::new(todo.into()))
    }

    async fn get_todos(&self, _: Request<Empty>) -> Result<Response<TodoList>, Status> {
        let todos = get_todos(&self.pool).await;
        Ok(Response::new(TodoList { todos: todos.into_iter().map(|t| t.into()).collect() }))
    }

    async fn update_todo(
        &self,
        request: Request<UpdateTodoRequest>,
    ) -> Result<Response<todo::Todo>, Status> {
        let req = request.into_inner();
        let todo = update_todo(&self.pool, req.id, &req.title, req.completed).await;
        Ok(Response::new(todo.into()))
    }

    async fn delete_todo(
        &self,
        request: Request<DeleteTodoRequest>,
    ) -> Result<Response<Empty>, Status> {
        let req = request.into_inner();
        delete_todo(&self.pool, req.id).await;
        Ok(Response::new(Empty {}))
    }
}

pub async fn run_server() {
    let addr = "[::]:50051".parse().unwrap();
    let pool = init_db().await;
    let service = MyTodoService { pool };

    let cors = CorsLayer::new()
        .allow_origin(Any);    

    let grpc_web_service = tonic_web::enable(TodoServiceServer::new(service));
    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .add_service(grpc_web_service)
        .serve(addr)
        .await
        .unwrap();
}
