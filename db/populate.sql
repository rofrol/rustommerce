SET CLIENT_ENCODING TO UTF8;
SET STANDARD_CONFORMING_STRINGS TO ON;
BEGIN;

DROP TABLE IF EXISTS products CASCADE;
DROP TABLE IF EXISTS notifications CASCADE;
DROP TABLE IF EXISTS user_information CASCADE;
DROP TABLE IF EXISTS data_sets_in_categories CASCADE;
DROP TABLE IF EXISTS comments CASCADE;
DROP TABLE IF EXISTS categories CASCADE;
DROP TABLE IF EXISTS data_sets CASCADE;

create table products (
  name varchar,
  description varchar,
  url varchar,
  main_image_url varchar,
  id serial primary key
);

create table user_information (
  "magicUrl" varchar,
  name varchar,
  surname varchar,
  "userId" serial primary key
);

create table notifications (
  context varchar,
  status bool,
  "userId" integer REFERENCES user_information ("userId"),
  id serial primary key
);

create table data_sets (
  name varchar,
  owner varchar,
  "ownerEmail" varchar,
  "releaseDate" varchar,
  "updatedDate" varchar,
  rating real,
  favourite bool,
  description varchar,
  usage varchar,
  "categoryId" integer,
  private bool,
  url varchar,
  id serial primary key
);

create table categories (
  title varchar,
  route varchar,
  count integer,
  "contentUrl" varchar,
  type varchar not null,
  "parentId" integer REFERENCES categories (id),
  id serial primary key
);

create table data_sets_in_categories (
  data_sets_id integer REFERENCES data_sets (id),
  categories_id integer REFERENCES categories (id),
  id serial primary key
);

create table comments (
  content varchar,
  "userName" varchar,
  "userPhotoUrl" varchar,
  date varchar,
  data_set_id integer REFERENCES data_sets (id),
  id serial primary key
);


insert into products values (
  'Product 1',
  'Description of Product 1',
  'product1',
  'product1.jpg'
);

insert into products (name, description, url, main_image_url, id) values (
  'Product 2',
  'Description of Product 2',
  'product2',
  'product2.jpg',
  DEFAULT
);


-- works too:
--insert into data_sets values ('Name of Data Set');


insert into user_information("magicUrl", name, surname) values ('images/user.jpg', 'James', 'Bond');
insert into notifications(context, status, "userId") values ('the dataset had been shared to you!', false, 1);

insert into data_sets values (
  'Name of Data Set',
  'John Smith',
  'john.smith@example.com',
  '2014-08-03',
  '2014-08-03',
  0.2,
  true,
  'Lorem ipsum dolor sit amet enim. Etiam ullamcorper. Suspendisse a pellentesque dui, non felis. Maecenas malesuada elit lectus felis, malesuada ultricies. Curabitur et ligula. Ut molestie a, ultricies porta urna. Vestibulum commodo volutpat a, convallis ac, laoreet enim. Phasellus fermentum in, dolor. Pellentesque facilisis. Nulla imperdiet sit amet magna. Vestibulum dapibus, mauris nec malesuada fames ac turpis velit, rhoncus eu, luctus et interdum adipiscing wisi. Aliquam erat ac ipsum. Integer aliquam purus. Quisque lorem tortor fringilla sed, vestibulum id, eleifend justo vel bibendum sapien massa ac turpis faucibus orci luctus non, consectetuer lobortis quis, varius in, purus. Integer ultrices posuere cubilia Curae, Nulla ipsum dolor lacus, suscipit adipiscing. Cum sociis natoque penatibus et ultrices volutpat. Nullam wisi ultricies a, gravida vitae, dapibus risus ante sodales lectus blandit eu, tempor diam pede cursus vitae, ultricies eu, faucibus quis, porttitor eros cursus lectus, pellentesque eget, bibendum a, gravida ullamcorper quam. Nullam viverra consectetuer. Quisque cursus et, porttitor risus. Aliquam sem. In hendrerit nulla quam nunc, accumsan congue. Lorem ipsum primis in nibh vel risus. Sed vel lectus. Ut sagittis, ipsum dolor quam.',
  'Usage',
  '2',
  true,
  'dataSets/name-of-data-set'
);

insert into data_sets values (
  'Name of Data Set 2',
  'Jane Smith',
  'john.smith@example.com',
  '2014-08-04',
  '2014-08-04',
  0.7,
  false,
  'In hendrerit nulla quam nunc, accumsan congue. Lorem ipsum primis in nibh vel risus. Sed vel lectus. Ut sagittis, ipsum dolor quam..Lorem ipsum dolor sit amet enim. Etiam ullamcorper. Suspendisse a pellentesque dui, non felis. Maecenas malesuada elit lectus felis, malesuada ultricies. Curabitur et ligula. Ut molestie a, ultricies porta urna. Vestibulum commodo volutpat a, convallis ac, laoreet enim. Phasellus fermentum in, dolor.',
  'Usage',
  '2',
  true,
  'dataSets/name-of-data-set-2'
);

insert into categories values ('Data Set Category 1', 'data-set-category-1', 8, 'dataSetsCategories/dataSets', 'dataSet');
insert into categories values ('Data Set SubCategory 1', 'data-set-subcategory-1', 4, 'dataSetsCategories/dataSetsSub', 'dataSet', 1);
insert into categories values ('Data Set SubCategory 2', 'data-set-subcategory-2', 4, 'dataSetsCategories/dataSetsSub-2', 'dataSet', 1);

insert into data_sets_in_categories values (1, 1);
insert into data_sets_in_categories values (2, 1);

insert into comments values (
  'Integer ultrices posuere cubilia Curae, Nulla ipsum dolor lacus, suscipit adipiscing. Cum sociis natoque penatibus et ultrices volutpat. Nullam wisi ultricies a, gravida vitae, dapibus risus ante sodales lectus blandit eu, tempor diam pede cursus vitae, ultricies eu, faucibus quis, porttitor eros cursus lectus, pellentesque eget, bibendum a, gravida ullamcorper quam. Nullam viverra consectetuer. Quisque cursus et, porttitor risus. Aliquam sem. In hendrerit nulla quam nunc, accumsan congue. Lorem ipsum primis in nibh vel risus. Sed vel lectus. Ut sagittis, ipsum dolor quam.',
  'John Smith',
  'images/user.jpg',
  '2017-01-24T15:00:39.280Z',
  1
);

insert into comments values (
  'Curabitur et ligula. Ut molestie a, ultricies porta urna. Vestibulum commodo volutpat a, convallis ac, laoreet enim. Phasellus fermentum in, dolor. Pellentesque facilisis. Nulla imperdiet sit amet magna. Vestibulum dapibus, mauris nec malesuada fames ac turpis velit, rhoncus eu, luctus et interdum adipiscing wisi. Aliquam erat ac ipsum. Integer aliquam purus.',
  'John Blackhawk',
  'images/user.jpg',
  '2017-01-24T13:08:41.231Z',
  1
);

select * from products;
select * from data_sets;
select * from user_information;
select * from user_information as u join notifications as n on u."userId" = n."userId";
COMMIT;
