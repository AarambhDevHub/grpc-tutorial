use std::collections::HashMap;

use tonic::transport::Channel;
use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};

// Include your generated protobuf code
mod todo {
    tonic::include_proto!("todo");
}
use todo::todo_service_client::TodoServiceClient;
use todo::{Todo, TodoList, CreateTodoRequest, UpdateTodoRequest, DeleteTodoRequest};

#[derive(Debug, Clone)]
struct AppState {
    todo_client: TodoServiceClient<Channel>,
}

#[tokio::main]
async fn main() {
    // Connect to gRPC server
    let channel = Channel::from_static("http://[::1]:50051")
        .connect()
        .await
        .unwrap();
    
    let todo_client = TodoServiceClient::new(channel);
    
    let state = AppState { todo_client };
    let state_filter = warp::any().map(move || state.clone());
    
    // Routes
    let index = warp::path::end()
        .and(warp::get())
        .map(|| {
            warp::reply::html(include_str!("index.html"))
        });
    
    let get_todos = warp::path!("todos")
        .and(warp::get())
        .and(state_filter.clone())
        .and_then(get_todos_handler);
    
    let create_todo = warp::path!("todos")
        .and(warp::post())
        .and(warp::body::form())
        .and(state_filter.clone())
        .and_then(create_todo_handler);
    
    let update_todo = warp::path!("todos" / i64)
        .and(warp::put())
        .and(warp::body::form())
        .and(state_filter.clone())
        .and_then(update_todo_handler);
    
    let delete_todo = warp::path!("todos" / i64)
        .and(warp::delete())
        .and(state_filter.clone())
        .and_then(delete_todo_handler);
    
    let routes = index
        .or(get_todos)
        .or(create_todo)
        .or(update_todo)
        .or(delete_todo)
        .with(warp::cors().allow_any_origin());
    
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// Handler for getting todos
async fn get_todos_handler(state: AppState) -> Result<impl Reply, Rejection> {
    let mut client = state.todo_client;
    let request = tonic::Request::new(todo::Empty {});
    let response = client.get_todos(request).await.map_err(|e| {
        eprintln!("Error calling get_todos: {:?}", e);
        warp::reject::not_found()
    })?;
    
    let todos = response.into_inner().todos;
    
    Ok(warp::reply::html(render_todo_list(&todos)))
}

// Handler for creating a todo
async fn create_todo_handler(
    form: CreateTodoForm,
    state: AppState,
) -> Result<impl Reply, Rejection> {
    let mut client = state.todo_client;
    let request = tonic::Request::new(CreateTodoRequest {
        title: form.title,
    });
    
    let response = client.create_todo(request).await.map_err(|e| {
        eprintln!("Error calling create_todo: {:?}", e);
        warp::reject::not_found()
    })?;
    
    let todo = response.into_inner();
    Ok(warp::reply::html(render_todo_item(&todo)))
}

// Handler for updating a todo
async fn update_todo_handler(
    id: i64,
    form: UpdateTodoForm,
    state: AppState,
) -> Result<impl Reply, Rejection> {
    let mut client = state.todo_client;

    let title_key = format!("title-{}", id);
    let title = form.extra.get(&title_key)
        .cloned()
        .unwrap_or_default();

    let completed = match form.completed.to_lowercase().as_str() {
        "true" | "on" => true,
        _ => false,
    };


    let request = tonic::Request::new(UpdateTodoRequest {
        id: Some(id),
        title,
        completed,
    });
    
    let response = client.update_todo(request).await.map_err(|e| {
        eprintln!("Error calling update_todo: {:?}", e);
        warp::reject::not_found()
    })?;
    
    let todo = response.into_inner();
    Ok(warp::reply::html(render_todo_item(&todo)))
}

// Handler for deleting a todo
async fn delete_todo_handler(
    id: i64,
    state: AppState,
) -> Result<impl Reply, Rejection> {
    let mut client = state.todo_client;
    let request = tonic::Request::new(DeleteTodoRequest {
        id: Some(id),
    });
    
    client.delete_todo(request).await.map_err(|e| {
        eprintln!("Error calling delete_todo: {:?}", e);
        warp::reject::not_found()
    })?;
    
    Ok(warp::reply::html(""))
}

// Form structures
#[derive(Deserialize)]
struct CreateTodoForm {
    title: String,
}

#[derive(Debug,Deserialize)]
struct UpdateTodoForm {
    #[serde(flatten)]   // This will capture all other fields
    extra: HashMap<String, String>,
    #[serde(default)]
    completed: String,
}

// HTML rendering functions
fn render_todo_list(todos: &[Todo]) -> String {
    let mut html = String::new();
    for todo in todos {
        html.push_str(&render_todo_item(todo));
    }
    html
}

fn render_todo_item(todo: &Todo) -> String {
    format!(r#"
    <div class="todo-item" id="todo-{}" hx-target="this" hx-swap="outerHTML">
        <input type="checkbox" 
               hx-put="/todos/{}"
               hx-include="[name='title-{}']"
               name="completed"
               {} />
        <span class="{}">{}</span>
        <input type="hidden" name="title-{}" value="{}" />
        <button hx-delete="/todos/{}">Delete</button>
    </div>
    "#,
    todo.id.unwrap_or(0),
    todo.id.unwrap_or(0),
    todo.id.unwrap_or(0),  // For hx-include
    if todo.completed { "checked" } else { "" },
    if todo.completed { "completed" } else { "" },
    todo.title,
    todo.id.unwrap_or(0),  // For hidden input name
    todo.title,            // For hidden input value
    todo.id.unwrap_or(0)   // For delete button
    )
}
