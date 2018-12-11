// http://stackoverflow.com/questions/25877285/how-to-disable-unused-code-warnings-in-rust
// https://users.rust-lang.org/t/turning-off-compiler-warning-messages/4975/2
#![allow(non_snake_case)]

use postgres::{Connection, TlsMode};

use std::env;

use actix_web::{FromRequest, HttpRequest, HttpResponse};
use futures::future::{result, FutureResult};

use serde_derive::{Deserialize, Serialize};

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

fn connection() -> Connection {
    Connection::connect(
        "postgres://".to_owned()
            + &env::var("PGUSER").unwrap()
            + "@localhost/"
            + &env::var("PGDATABASE").unwrap(),
        TlsMode::None,
    )
    .unwrap()
}

pub fn user_information(_req: &HttpRequest) -> FutureResult<HttpResponse, actix_web::error::Error> {
    let conn = connection();
    let user_id = 1;
    let rows = &conn.query(
    r#"SELECT "userId", name, surname, "magicUrl" FROM user_information where "userId" = $1"#,
                           &[&user_id])
                    .expect("Query failed");
    let row = rows.into_iter().next().expect("No next row");

    let mut notifications = Vec::new();
    for row in &conn.query(
        r#"select context, status from user_information as u join notifications as n on n."userId"
         = u."userId" and u."userId" = $1"#,
                           &[&user_id])
                    .expect("Query failed") {
        let notification = Notification {
            context: row.get(0),
            status: row.get(1),
        };
        notifications.push(notification);
    }

    let s = UserInformation {
        userId: row.get(0),
        name: row.get(1),
        surname: row.get(2),
        magicUrl: row.get(3),
        notifications,
    };
    result(Ok(HttpResponse::Ok().json(s)))
}
#[derive(Serialize, Deserialize)]
struct DataSet {
    id: i32,
    name: String,
}

pub fn data_sets(_req: &HttpRequest) -> FutureResult<HttpResponse, actix_web::error::Error> {
    let conn = connection();
    let mut data_sets = Vec::new();

    for row in &conn
        .query("SELECT id, name FROM data_sets", &[])
        .expect("Query failed")
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
    result(Ok(HttpResponse::Ok().json(data_sets)))
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
pub fn data_set(req: &HttpRequest) -> FutureResult<HttpResponse, actix_web::error::Error> {
    let conn = connection();

    let url = actix_web::Path::<String>::extract(req).expect("Path extract failed");

    let url2 = "dataSets/".to_owned() + &url;
    let rows = &conn
        .query("SELECT id, name FROM data_sets where url = $1", &[&url2])
        .expect("Query failed");
    let row = rows.into_iter().next().expect("No next row");
    let data_set_id: i32 = row.get(0);

    let mut comments = Vec::new();
    for row in &conn
        .query(
            r#"select c.id, content, "userName", "userPhotoUrl", date from comments as c join data_sets as d on c.data_set_id = d.id and d.id = $1"#,
            &[&data_set_id],
        )
        .expect("Query failed")
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
    result(Ok(HttpResponse::Ok().json(s)))
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
pub fn data_set_category(req: &HttpRequest) -> FutureResult<HttpResponse, actix_web::error::Error> {
    let conn = connection();
    let url = actix_web::Path::<String>::extract(req).expect("Path extract failed");
    let url2 = "dataSetsCategories/".to_owned() + &url;
    let rows = &conn
        .query(
            r#"select id from categories where "contentUrl" = $1"#,
            &[&url2],
        )
        .unwrap();
    let row = rows.into_iter().next().unwrap();
    let category_id: i32 = row.get(0);

    let dataSetShortRows = &conn.query(r#"select d.id, name, SUBSTRING(description,0,100) as description, owner, "releaseDate",
        rating, favourite, url from data_sets d join data_sets_in_categories di on d.id = di.data_sets_id
        and di.categories_id = $1"#,
                    &[&category_id])
             .unwrap();
    let dataSetShortRow = dataSetShortRows.into_iter().next().unwrap();

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

    result(Ok(HttpResponse::Ok().json(s)))
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

pub fn data_sets_categories(
    _req: &HttpRequest,
) -> FutureResult<HttpResponse, actix_web::error::Error> {
    let conn = connection();
    let mut categories = Vec::new();
    for row in &conn
        .query(
            r#"SELECT id, title, route, count, "contentUrl" FROM categories
             where type = 'dataSet' and "parentId" is null"#,
            &[],
        )
        .unwrap()
    {
        let row_id: i32 = row.get(0);

        let mut subcategories = Vec::new();
        for subcategoryRow in &conn
            .query(
                r#"SELECT id, title, route, count, "contentUrl" FROM categories
                 where type = 'dataSet' and "parentId" = $1"#,
                &[&row_id],
            )
            .unwrap()
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
    result(Ok(HttpResponse::Ok().json(categories)))
}

/*
use std::fs::File;
use std::io::{BufWriter, Write};

#[post("/dataSets/new", data = "<upload>")]
fn data_sets_new(upload: DataSetMultipart) -> CORS<String> {
    let f = File::create("plik").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(&*upload.metadata)
        .expect("Unable to write data");
    CORS::any(format!("I read this: {:?}", upload))
}

#[derive(Debug)]
struct DataSetMultipart {
    name: String,
    description: String,
    categoryId: i32,
    metadata: Vec<u8>,
}

use std::io::{Cursor, Read};

impl FromData for DataSetMultipart {
    type Error = ();

    fn from_data(request: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        // All of these errors should be reported
        let ct = request
            .headers()
            .get_one("Content-Type")
            .expect("no content-type");
        let idx = ct.find("boundary=").expect("no boundary");
        let boundary = &ct[(idx + "boundary=".len())..];

        let mut d = Vec::new();
        data.stream_to(&mut d).expect("Unable to read");

        let mut mp = Multipart::with_body(Cursor::new(d), boundary);

        // Custom implementation parts

        let mut name = None;
        let mut description = None;
        let mut categoryId = None;
        let mut metadata = None;

        mp.foreach_entry(|mut entry| match entry.name.as_str() {
            "name" => {
                let t = entry.data.as_text().expect("not text");
                name = Some(t.into());
            }
            "description" => {
                let t = entry.data.as_text().expect("not text");
                description = Some(t.into());
            }
            "categoryId" => {
                let t = entry.data.as_text().expect("not text");
                let n = t.parse().expect("not number");
                categoryId = Some(n);
            }
            "metadata" => {
                let mut d = Vec::new();
                let f = entry.data.as_file().expect("not file");
                f.read_to_end(&mut d).expect("can't read");
                metadata = Some(d);
            }
            other => panic!("No known key {}", other),
        })
        .expect("Unable to iterate");

        let v = DataSetMultipart {
            name: name.expect("name not set"),
            description: description.expect("description not set"),
            categoryId: categoryId.expect("categoryId not set"),
            metadata: metadata.expect("file not set"),
        };

        // End custom

        Outcome::Success(v)
    }
}
*/
