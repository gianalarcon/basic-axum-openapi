use axum::{
  extract::Path,
  http::StatusCode,
  response::IntoResponse,
  routing::{delete, get},
  Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct Category {
  pub id: usize,
  pub name: String,
  pub url: String,
  pub icon: String,
}

impl Category {
  pub fn new() -> Self {
    Default::default()
  }
}

#[derive(OpenApi)]
#[openapi(
  paths(get_categories, create_category, delete_category),
  components(schemas(Category)),
  tags(
    (name = "Sample Project",
    description = "This is a sample AXUM swagger integration"
  )
))]
struct ApiDoc;

#[tokio::main]
async fn main() {
  println!("Starting server at port 8081");
  let app = Router::new()
    .route("/category", get(get_categories).post(create_category))
    .route("/category/:id", delete(delete_category))
    .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

  axum::Server::bind(&"127.0.0.1:8081".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

/// Sample API Endpoint Summary Message to get categories
#[utoipa::path(get, path="/category",
		responses((status = 200, body = [Category]),(status = 404)))]

async fn get_categories() -> Json<Vec<Category>> {
  Json(vec![
    Category {
      id: 1,
      name: "Category 1".to_string(),
      url: "video.url".to_string(),
      icon: "fontawesome video icon".to_string(),
    },
    Category {
      id: 2,
      name: "Category 2".to_string(),
      url: "video.url".to_string(),
      icon: "fontawesome video icon".to_string(),
    },
  ])
}

/// Saving new category message
/// ///
/// Any other message that we want
#[utoipa::path(post, path="/category", request_body = Category,
		responses((status = 200, description = "Creating a new category",body = Category),
		(status = 404, description = "Not found error")))]
async fn create_category(Json(category): Json<Category>) -> Json<Category> {
  let modified_category = Category { id: 3, ..category };
  Json(modified_category)
}

#[utoipa::path(delete, path="/category/{id}", 
params(("id" = usize, Path, description = "Category ID to delete")),
		responses((status = 200, description = "Deleting a category", body = bool ),
		(status = 404, description = "Not found error")))]

async fn delete_category(Path(id): Path<usize>) -> impl IntoResponse {
  if id == 8081 {
    (StatusCode::OK, Json(true)).into_response()
  } else {
    (StatusCode::NOT_FOUND, Json(false)).into_response()
  }
}
