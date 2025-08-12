# Real-Time Chat App (Rust & React)

## Project Description/Vision

A real-time chatting application built with modern web technologies. The back-end is built with Rust's performance and safety using the Actix Web framework with a PostgreSQL database. The front-end is a simple but responsive design built with React, Typescript and TailwindCSS.

## Current Status

**Under-Development**

This project is still in its early stages and isn't fully complete yet. See the roadmap below for what is working.

## Tech Stack

### **Backend**
 * Rust
 * Actix Web
 * PostgreSQL
 * Sqlx
 * jsonwebtocken
### **Frontend**
 * Typescript
 * React
 * TailwindCSS
 * Vite
### **DevOps**
 * Docker
 * Sqitch

## Roadmap

### Phase 1: (Backend) Core Authentication and Users (In Progress)
- [X] Register user endpoint 
- [X] User Login endpoint
- [ ] JWT generation and validation
- [ ] Refresh Token endpoint
- [ ] Middleware to restrict access
### Phase 2: (Backend) Core chat system 
- [ ] Get active rooms
- [ ] Create a room
- [ ] Get users within a Room
- [ ] Storing chat messages in PostgreSQL
- [ ] Get past messages within a room
#### Phase 3: (Backend) Websockets (Real-time) message
- [ ] Websocket message distribution
- [ ] Join messages within websockets
- [ ] Typing indicators
### Phase 4: (Frontend) Frontend UI
- [ ] Roadmap required components 
- [ ] Login and Registration Forms
- [ ] Active Chat Rooms Component
- [ ] Main chat view Component
- [ ] Input for sending messages
 ### Phase 5: (Frontend) Frontend Integration
- [ ] Connect the Login and Signup forms to the API
- [ ] Add API request for showing active rooms
- [ ] Integrate Websocket connection into main chat component

### Future features
- [ ] Private Messaging
- [ ] Private Chatrooms (Password Protected / Invite Only?)
- [ ] User Profiles and Avatars

## Getting Started

### Prerequisites
 * Rust
 * Node.js and npm
 * Docker with Docker Compose Plugin

### Clone the repository
```
git clone https://github.com/J-Snook/chat-app.git
```
### Backend Setup
```
cd chat-app/back-end
```
Download and install sqitch through Docker [Sqitch Download](https://sqitch.org/download/docker/)
Set up an ``.env`` by filling in ``.env.example`` with values
```
docker compose up -d
```
To launch the postgres database
```
./sqitch deploy
```
To create the tables
```
cargo run
```
Now the RESTful API should be working on ``127.0.0.1:8080``

## Frontend Setup
```
cd ../front-end
```
```
npm install
```
```
  npx vite
```

Thanks for reading. Any questions, queries or suggestions, feel free to reach out to me here or my email (reachout@jsnook.co.uk)
