// http://stackoverflow.com/questions/25877285/how-to-disable-unused-code-warnings-in-rust
// https://users.rust-lang.org/t/turning-off-compiler-warning-messages/4975/2
#![allow(non_snake_case)]

mod errors {
    use actix_web::{HttpResponse, ResponseError};
    use deadpool_postgres::PoolError;
    use derive_more::{Display, From};
    use tokio_postgres::error::Error as PGError;

    #[derive(Display, From, Debug)]
    pub enum MyError {
        NotFound,
        NonError,
        PGError(PGError),
        PoolError(PoolError),
    }
    impl std::error::Error for MyError {}

    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => HttpResponse::NotFound().finish(),
                MyError::PoolError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

use actix_web::{web, Error as ActixError, HttpResponse};

use serde_derive::{Deserialize, Serialize};

use deadpool_postgres::{Client, Pool};

#[derive(Serialize, Deserialize)]
struct UserInformation {
    userId: i32,
    name: String,
    surname: String,
    magicUrl: String,
    notifications: Vec<Notification>,
}

#[derive(Serialize, Deserialize)]
struct Notification {
    context: String,
    status: bool,
}

async fn get_user_information(client: &Client) -> Result<UserInformation, errors::MyError> {
    let user_id = 1;
    let rows = client.query(
    r#"SELECT "userId", name, surname, "magicUrl" FROM user_information where "userId" = $1"#,
                           &[&user_id]).await?;
    let row = rows.into_iter().next().expect("No next row");

    let mut notifications = Vec::new();
    for row in client.query(
        r#"select context, status from user_information as u join notifications as n on n."userId"
         = u."userId" and u."userId" = $1"#,
                           &[&user_id])
                    .await
                    .expect("Query failed") {
        let notification = Notification {
            context: row.get(0),
            status: row.get(1),
        };
        notifications.push(notification);
    }

    Ok(UserInformation {
        userId: row.get(0),
        name: row.get(1),
        surname: row.get(2),
        magicUrl: row.get(3),
        notifications,
    })
}

pub async fn user_information(db_pool: web::Data<Pool>) -> Result<HttpResponse, ActixError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let user_information = get_user_information(&client).await?;
    Ok(HttpResponse::Ok().json(user_information))
}

#[derive(Serialize, Deserialize)]
struct DataSet {
    id: i32,
    name: String,
}

pub async fn data_sets(db_pool: web::Data<Pool>) -> Result<HttpResponse, ActixError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let mut data_sets = Vec::new();

    for row in client
        .query("SELECT id, name FROM data_sets", &[])
        .await
        .map_err(errors::MyError::PGError)?
    {
        let data_set = DataSet {
            id: row.get(0),
            name: row.get(1),
        };
        // note: move occurs because `data_set` has type `DataSet`,
        // which does not implement the `Copy` trait
        println!("Found DataSet {}", &data_set.name);
        data_sets.push(data_set);
    }
    Ok(HttpResponse::Ok().json(data_sets))
}

#[derive(Serialize, Deserialize)]
struct DataSetWithComments {
    id: i32,
    name: String,
    comments: Vec<Comment>,
}

#[derive(Serialize, Deserialize)]
struct Comment {
    id: i32,
    content: String,
    userName: String,
    userPhotoUrl: String,
    date: String,
}

// test url: /dataSets/name-of-data-set
pub async fn data_set(
    url: web::Path<String>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, ActixError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;

    let url2 = "dataSets/".to_owned() + &url;
    let rows = client
        .query("SELECT id, name FROM data_sets where url = $1", &[&url2])
        .await
        .map_err(errors::MyError::PGError)?;
    let row = rows.into_iter().next().expect("No next row");
    let data_set_id: i32 = row.get(0);

    let mut comments = Vec::new();
    for row in client
        .query(
            r#"select c.id, content, "userName", "userPhotoUrl", date from comments as c join data_sets as d on c.data_set_id = d.id and d.id = $1"#,
            &[&data_set_id],
        )
        .await
        .map_err(errors::MyError::PGError)?
    {
        let comment = Comment {
            id: row.get(0),
            content: row.get(1),
            userName: row.get(2),
            userPhotoUrl: row.get(3),
            date: row.get(4),
        };
        comments.push(comment);
    }

    let s = DataSetWithComments {
        id: row.get(0),
        name: row.get(1),
        comments,
    };
    Ok(HttpResponse::Ok().json(s))
}

#[derive(Serialize, Deserialize)]
struct DataSetShort {
    id: i32,
    name: String,
    description: String,
    owner: String,
    releaseDate: String,
    rating: f32,
    favourite: bool,
    url: String,
}

// test url: /dataSetsCategories/dataSets
pub async fn data_set_category(
    url: web::Path<String>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, ActixError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let url2 = "dataSetsCategories/".to_owned() + &url;
    let rows = client
        .query(
            r#"select id from categories where "contentUrl" = $1"#,
            &[&url2],
        )
        .await
        .map_err(errors::MyError::PGError)?;
    // https://www.reddit.com/r/rust/comments/bod7eq/how_to_use_with_option_and_result/enesbnp/?utm_source=reddit&utm_medium=web2x&context=3
    // https://github.com/rust-lang/rust/issues/46871
    let row = rows.into_iter().next().ok_or(errors::MyError::NonError)?;
    let category_id: i32 = row.get(0);

    let dataSetShortRows = client.query(r#"select d.id, name, SUBSTRING(description,0,100) as description, owner, "releaseDate",
        rating, favourite, url from data_sets d join data_sets_in_categories di on d.id = di.data_sets_id
        and di.categories_id = $1"#,
                    &[&category_id])
        .await
        .map_err(errors::MyError::PGError)?;
    let dataSetShortRow = dataSetShortRows
        .into_iter()
        .next()
        .ok_or(errors::MyError::NonError)?;

    let s = DataSetShort {
        id: dataSetShortRow.get(0),
        name: dataSetShortRow.get(1),
        description: dataSetShortRow.get(2),
        owner: dataSetShortRow.get(3),
        releaseDate: dataSetShortRow.get(4),
        rating: dataSetShortRow.get(5),
        favourite: dataSetShortRow.get(6),
        url: dataSetShortRow.get(7),
    };

    Ok(HttpResponse::Ok().json(s))
}

#[derive(Serialize, Deserialize)]
struct Category {
    id: i32,
    title: String,
    route: String,
    count: i32,
    contentUrl: String,
    subcategories: Vec<Subcategory>,
}

#[derive(Serialize, Deserialize)]
struct Subcategory {
    id: i32,
    title: String,
    route: String,
    count: i32,
    contentUrl: String,
}

pub async fn data_sets_categories(db_pool: web::Data<Pool>) -> Result<HttpResponse, ActixError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let mut categories = Vec::new();
    for row in client
        .query(
            r#"SELECT id, title, route, count, "contentUrl" FROM categories
             where type = 'dataSet' and "parentId" is null"#,
            &[],
        )
        .await
        .map_err(errors::MyError::PGError)?
    {
        let row_id: i32 = row.get(0);

        let mut subcategories = Vec::new();
        for subcategoryRow in client
            .query(
                r#"SELECT id, title, route, count, "contentUrl" FROM categories
                 where type = 'dataSet' and "parentId" = $1"#,
                &[&row_id],
            )
            .await
            .map_err(errors::MyError::PGError)?
        {
            let subcategory = Subcategory {
                id: subcategoryRow.get(0),
                title: subcategoryRow.get(1),
                route: subcategoryRow.get(2),
                count: subcategoryRow.get(3),
                contentUrl: subcategoryRow.get(4),
            };

            subcategories.push(subcategory);
        }

        let category = Category {
            id: row.get(0),
            title: row.get(1),
            route: row.get(2),
            count: row.get(3),
            contentUrl: row.get(4),
            subcategories: subcategories,
        };
        println!("Found Category {}", &category.title);
        categories.push(category);
    }
    Ok(HttpResponse::Ok().json(categories))
}
