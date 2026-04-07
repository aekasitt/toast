/* ~~/src/main.rs */

// standard crates
use std::env::current_dir;
use std::sync::Arc;

// third-party crates
use async_trait::async_trait;
use axum::extract::{FromRequestParts, Json, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{Router, delete, get};
use http::request::Parts;
use serde::{Deserialize, Serialize};
use serde_json::json;
use toasty::{Db, Model};
use toasty_driver_sqlite::Sqlite;
use tokio::net::TcpListener;
use uuid::Uuid;

#[derive(Clone, Model, Serialize)]
struct Todo {
  completed: bool,
  #[key]
  #[auto(uuid(v4))]
  slug: Uuid,
  title: String,
}

#[derive(Deserialize)]
struct CreateTodo {
  title: String,
}

enum TodoError {
  Internal,
  NotFound(toasty::Error),
}
impl From<toasty::Error> for TodoError {
  fn from(e: toasty::Error) -> Self {
    if e.is_record_not_found() {
      TodoError::NotFound(e)
    } else {
      TodoError::Internal
    }
  }
}
impl IntoResponse for TodoError {
  fn into_response(self) -> Response {
    match self {
      TodoError::NotFound(error) => (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": format!("{error}") })),
      )
        .into_response(),
      TodoError::Internal => (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": "Oops, we did it again" })),
      )
        .into_response(),
    }
  }
}

#[async_trait]
trait TodoServiceSchema: Send + Sync {
  async fn create(&self, db: &mut Db, input: CreateTodo) -> Result<Todo, TodoError>;
  async fn complete(&self, db: &mut Db, slug: Uuid) -> Result<Todo, TodoError>;
  async fn delete(&self, db: &mut Db, slug: Uuid) -> Result<(), TodoError>;
  async fn get(&self, db: &mut Db, slug: Uuid) -> Result<Todo, TodoError>;
  async fn list(&self, db: &mut Db) -> Result<Vec<Todo>, TodoError>;
}

struct TodoService;
#[async_trait]
impl TodoServiceSchema for TodoService {
  async fn create(&self, db: &mut Db, input: CreateTodo) -> Result<Todo, TodoError> {
    let todo = toasty::create!(Todo {
      completed: false,
      title: input.title
    })
    .exec(db)
    .await?;
    Ok(todo)
  }
  async fn complete(&self, db: &mut Db, slug: Uuid) -> Result<Todo, TodoError> {
    let mut todo = Todo::get_by_slug(db, &slug).await?;
    Todo::update(&mut todo).completed(true).exec(db).await?;
    Ok(todo)
  }
  async fn delete(&self, db: &mut Db, slug: Uuid) -> Result<(), TodoError> {
    let todo = Todo::get_by_slug(db, &slug).await?;
    todo.delete().exec(db).await?;
    Ok(())
  }
  async fn get(&self, db: &mut Db, slug: Uuid) -> Result<Todo, TodoError> {
    let todo = Todo::get_by_slug(db, &slug).await?;
    Ok(todo)
  }
  async fn list(&self, db: &mut Db) -> Result<Vec<Todo>, TodoError> {
    Ok(Todo::all().exec(db).await?)
  }
}

async fn complete_todo(
  State(context): State<Context>,
  Database(mut db): Database,
  Path(slug): Path<Uuid>,
) -> Result<Json<Todo>, TodoError> {
  context.service.complete(&mut db, slug).await.map(Json)
}
async fn delete_todo(
  State(context): State<Context>,
  Database(mut db): Database,
  Path(slug): Path<Uuid>,
) -> Result<(StatusCode, Json<()>), TodoError> {
  context.service.delete(&mut db, slug).await?;
  Ok((StatusCode::NO_CONTENT, Json(())))
}
async fn get_todo(
  State(context): State<Context>,
  Database(mut db): Database,
  Path(slug): Path<Uuid>,
) -> Result<Json<Todo>, TodoError> {
  context.service.get(&mut db, slug).await.map(Json)
}
async fn create_todo(
  State(context): State<Context>,
  Database(mut db): Database,
  Json(input): Json<CreateTodo>,
) -> Result<(StatusCode, Json<Todo>), TodoError> {
  let todo = context.service.create(&mut db, input).await?;
  Ok((StatusCode::CREATED, Json(todo)))
}
async fn list_todos(
  State(context): State<Context>,
  Database(mut db): Database,
) -> Result<Json<Vec<Todo>>, TodoError> {
  context.service.list(&mut db).await.map(Json)
}

// Dependency injection
pub struct Database(pub Db);
impl FromRequestParts<Context> for Database {
  type Rejection = TodoError;
  fn from_request_parts(
    _: &mut Parts,
    context: &Context,
  ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
    async { Ok(Database(context.database.clone())) }
  }
}
#[derive(Clone)]
struct Context {
  database: Db,
  service: Arc<dyn TodoServiceSchema>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let database_path = current_dir()?.join("todos.db");
  let database_url = format!("sqlite://{}", database_path.display());
  let driver = Sqlite::new(database_url)?;
  let database = Db::builder()
    .models(toasty::models!(Todo))
    .build(driver)
    .await?;
  match database.push_schema().await {
    Ok(_) => println!("✅ Migrated models into database tables."),
    Err(_) => println!("⚠️ Model tables already existed."),
  };
  let service: Arc<dyn TodoServiceSchema> = Arc::new(TodoService {});
  let context = Context { database, service };
  let app = Router::new()
    .route("/todos", get(list_todos).post(create_todo))
    .route(
      "/todos/{slug}",
      delete(delete_todo).get(get_todo).put(complete_todo),
    )
    .with_state(context);
  let listener = TcpListener::bind("127.0.0.1:3000").await?;
  println!("✅ Listening on http://localhost:3000");
  axum::serve(listener, app).await?;
  Ok(())
}
