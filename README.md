# 🚀 HTMX + Warp + gRPC Todo App

A simple To-Do App built with Rust using the following technologies:

- gRPC – Efficient backend communication
- HTMX – Reactive frontend without JavaScript frameworks
- Warp – Fast and flexible web framework
- SQLite – Lightweight database
- Cargo Workspace – Manage multiple Rust projects
- Cargo-Make – Automate and simplify running multiple services

## 📌Features
✅ Create, update, and delete tasks
✅ Real-time UI updates with HTMX
✅ Fast API with gRPC & Warp
✅ Organized with Cargo Workspaces
✅ Easy service management with Cargo-Make

## 🛠️ Setup & Installation
1️⃣ Install Rust & Dependencies

Ensure Rust is installed. If not, install it from [Rust's official website](https://www.rust-lang.org/).
Then, install Cargo-Make:
```
cargo install --force cargo-make
```

2️⃣ Clone the Repository
```
git clone https://github.com/yourusername/htmx-warp-grpc-todo.git
cd htmx-warp-grpc-todo
```

3️⃣ Setup Database
Ensure SQLite is installed, then apply migrations:
```
cargo run -p grpc-todo --bin migrate
```

4️⃣ Running the Project
Use Cargo-Make to start both backend and frontend:
```
cargo make run-both
```

This will:
- Start the gRPC backend
- Start the HTMX + Warp frontend

5️⃣ Open in Browser
Visit:
```
http://localhost:3030
```
## 📂 Project Structure
```
htmx-warp-grpc-todo/
│── grpc-todo/        # gRPC backend
│── htmx-warp/        # Warp + HTMX frontend
│── Makefile.toml     # Cargo-Make tasks
│── Cargo.toml        # Workspace configuration
│── README.md         # Documentation
```

##⚡ API Endpoints
| Endpoint      | Method | Description            |
|--------------|--------|------------------------|
| `/todos`     | GET    | Fetch all todos       |
| `/todos`     | POST   | Create a new todo     |
| `/todos/{id}` | PUT    | Update an existing todo |
| `/todos/{id}` | DELETE | Delete a todo        |

## 🎯 Contributing

Feel free to fork this project, submit issues, or suggest improvements!

## 📜 License
This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.
---

## Donations

If you find this project useful and would like to support its continued development, you can make a donation via [Buy Me a Coffee](https://buymeacoffee.com/aarambhdevhub).

Thank you for your support!
Also, join my Discord Community for discussions and project updates! [Here](https://discord.gg/cDAbFuAC)

Happy coding! 🎧🚀
