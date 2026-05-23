// This handles GraphQL queries AND mutations (both use POST)
.service(web::resource("/graphql").guard(guard::Post()).to(graphql))

// This is just the UI playground interface (uses GET to load the HTML page)
.service(web::resource("/playground").guard(guard::Get()).to(graphql_playground))

| Feature             | REST                   | GraphQL                   | gRPC                | WebSocket         |
| ------------------- | ---------------------- | ------------------------- | ------------------- | ----------------- |
| **Protocol**        | HTTP/1.1, HTTP/2       | HTTP/1.1, HTTP/2          | HTTP/2 only         | WebSocket         |
| **Transport**       | TCP                    | TCP                       | TCP                 | TCP               |
| **Data Format**     | JSON/XML               | JSON                      | Protobuf (binary)   | Text/Binary       |
| **Connection**      | Stateless              | Stateless                 | Stateless\*         | Stateful          |
| **Methods**         | GET, POST, PUT, DELETE | POST (mostly)             | Defined in .proto   | Bidirectional     |
| **Endpoints**       | Multiple               | Single (/graphql)         | Defined services    | Single connection |
| **Real-time**       | No (polling)           | Subscriptions (WebSocket) | Streaming           | Yes               |
| **Browser Support** | ✅ Native              | ✅ Native                 | ❌ (needs gRPC-Web) | ✅ Native         |
| **Caching**         | ✅ Easy                | ❌ Harder                 | ❌ Complex          | ❌ N/A            |
| **Schema**          | Optional (OpenAPI)     | Required (SDL)            | Required (.proto)   | None              |
| **Discovery**       | OpenAPI/Swagger        | Introspection             | Reflection          | None              |
| **Learning Curve**  | Easy                   | Medium                    | Hard                | Medium            |


//--------------------------------------------------------------------------------------------------------------------------------------


1️⃣ First: Define Your Data Model
struct Book { id, title, author }


This is just normal Rust domain model.

Then expose it to GraphQL:

#[Object]
impl Book { ... }


👉 Now Book becomes a GraphQL type.

//---------------------------

2️⃣ Define What Clients Can READ (Query Root)

GraphQL starts from a root type.

struct QueryRoot { storage }


Then:

#[Object]
impl QueryRoot {
    async fn book(...)
    async fn books(...)
}


Each function = GraphQL query field

So schema now supports:

query {
  books { id title }
}

//---------------------------

3️⃣ Define What Clients Can MODIFY (Mutation Root)
struct MutationRoot { storage }


Add functions:

add_book

update_book

delete_book

Each function = GraphQL mutation.

So schema supports:

mutation {
  addBook(title: "...", author: "...") { id }
}

//---------------------------

4️⃣ Build the Schema

GraphQL always needs:

Schema<Query, Mutation, Subscription>


You write:

Schema::build(QueryRoot, MutationRoot, EmptySubscription)


👉 This wires everything together.

//---------------------------

5️⃣ Create HTTP Layer

GraphQL itself is just execution engine.

You still need:

HTTP server (Actix)

Route /graphql

Route /playground

So:

.service("/graphql")
.service("/playground")

//---------------------------

6️⃣ Execution Flow at Runtime

When request comes:

POST /graphql


Flow:

Actix receives HTTP request

Handler extracts GraphQLRequest

schema.execute(query)

async-graphql:

Parses query

Validates schema

Calls appropriate resolver (QueryRoot/MutationRoot method)

Result returned as JSON

🔄 Full Logical Flow
Define Rust model
        ↓
Expose it with #[Object]
        ↓
Create QueryRoot
        ↓
Create MutationRoot
        ↓
Build Schema
        ↓
Attach to HTTP server
        ↓
Run

//---------------------------

🧩 What GraphQL Actually Is

GraphQL =

Type system (Book)

Root entry points (Query/Mutation)

Resolver functions (your methods)

Execution engine (async-graphql)

HTTP is just transport.

🎯 Big Concept Difference (vs REST)

REST:

GET /books
POST /books
PUT /books/1


GraphQL:

POST /graphql


Query decides everything.