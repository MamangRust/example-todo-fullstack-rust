# Todo App with Axum, SQLite, Tauri (JS v2), and IPC

This project is a full-stack Todo application built with the following technologies:

- **Axum**: A web framework for building robust APIs in Rust.
- **SQLite**: A lightweight, file-based database for storing todos.
- **Tauri JS v2**: A framework for building cross-platform desktop applications with modern web technologies.
- **IPC (Inter-Process Communication)**: Used for connecting the frontend to the backend in the Tauri application.
- **Reqwest**: For making HTTP requests from the Tauri backend to the Axum API.

## Features

- **CRUD Operations**: Create, Read, Update, and Delete todos.
- **Rust Backend**:
  - Axum API for handling HTTP requests.
  - SQLite for persistent data storage.
- **Frontend**:
  - Built with TypeScript/JavaScript.
  - Uses Tauri's IPC for communication with the backend.

## Setup Instructions

### Prerequisites

1. **Rust**: Install [Rust](https://www.rust-lang.org/tools/install) if not already installed.
2. **Node.js**: Install [Node.js](https://nodejs.org/) and npm.
3. **Tauri Prerequisites**: Follow the [Tauri prerequisites](https://tauri.app/v2/guides/getting-started/prerequisites) for your platform.

### Backend Setup

1. Clone this repository:
   ```bash
   git clone https://github.com/MamangRust/example-todo-fullstack-rust.git
   cd example-todo-fullstack-rust/example-todo-backend
   ```

2. Create a `.env` file:
   ```bash
   DATABASE_URL=sqlite://sqlite.db
   ```

3. Run the database migrations:
   ```bash
   cargo install sqlx-cli
   sqlx migrate run
   ```

4. Start the backend server:
   ```bash
   cargo run
   ```

The Axum API will be running on `http://127.0.0.1:3000`.

### Frontend Setup

1. Navigate to the frontend directory:
   ```bash
   cd ../example-todo-frontend
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Start the Tauri development server:
   ```bash
   npm run tauri dev
   ```

The Tauri application will launch with the Todo app.

## Backend API Endpoints

| Method | Endpoint         | Description              |
|--------|------------------|--------------------------|
| GET    | `/todos`         | Fetch all todos         |
| POST   | `/todos`         | Create a new todo       |
| GET    | `/todos/:id`     | Fetch a todo by ID      |
| PUT    | `/todos/:id`     | Update a todo by ID     |
| DELETE | `/todos/:id`     | Delete a todo by ID     |

### Example Payload for Creating a Todo

```json
{
  "title": "My Todo",
  "description": "This is a sample todo."
}
```

## Tauri IPC Setup

The frontend communicates with the backend using Tauri's IPC mechanism. The backend invokes the Axum API using the `reqwest` library.

### IPC Commands

#### `fetch_todos`
Fetch all todos from the Axum API.

#### `create_todo`
Create a new todo. Expects a payload with `title` and `description`.

#### `update_todo`
Update an existing todo by ID. Expects `id`, `title`, and `description` as arguments.

#### `delete_todo`
Delete a todo by ID.

## Folder Structure

```
project-root
├── example-todo-backend
│   ├── src
│   │   ├── main.rs        # Entry point of the backend server
│   │   ├── database.rs    # Database connection and migration setup
│   │   ├── crud.rs        # CRUD logic for todos
│   │   ├── models.rs      # Todo model definition
│   │   └── handlers.rs    # Axum route handlers
│   ├── migrations         # SQL migrations folder
│   └── Cargo.toml         # Rust project configuration
├── example-todo-frontend
│   ├── src
│   │   ├── App.tsx        # React component for the app
│   │   ├── api            # API interaction logic with IPC
│   │   │   └── todo.ts    # Todo API functions
│   │   └── index.tsx      # React entry point
│   ├── public             # Static assets
│   └── package.json       # Frontend dependencies
├── README.md              # Project documentation
└── .env                   # Environment variables
```

## Acknowledgments

Special thanks to the Axum, SQLite, and Tauri communities for providing excellent tools to make this project possible.

## License

This project is licensed under the MIT License. See the LICENSE file for details.

