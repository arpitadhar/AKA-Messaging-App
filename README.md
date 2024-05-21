# RUST + React + Vite 
AKA MESSAGING APP 

This build is the refactored version of the main branch, but does not have all of the same functionality. However it implements websockets using socketio.

Build Instructions
1. Download latest project from the main branch 
2. cd into rust 
3. cargo install diesel_cli
4. Make sure to have PSQL and create a database there using the CREATE DATABASE aka_project command
5. echo DATABASE_URL=postgres://username:password@localhost/aka_project > .env
6. diesel migrations run 
7. diesel migrations redo
8. cargo run or cargo test to run the tests 
9. in a new terminal cd into vite project 
10. npm install vite 
11. npm install react-icons 
12. npm run dev 


Sources Used: 
1. https://www.youtube.com/watch?v=HwCqsOis894&t=8469s&ab_channel=AsaProgrammer (for creating hooks and chat components in react)
2. https://dev.to/krowemoh/a-web-app-in-rust-07-logging-a-user-in-1e0a
3. https://dev.to/krowemoh/a-web-app-in-rust-06-registering-a-user-57po
4. https://medium.com/@stheodorejohn/handling-browser-refresh-in-react-best-practices-and-pitfalls-5d4451d579ff#:~:text=To%20stop%20the%20refresh%20process,confirmation%20message%20to%20the%20user.
5. https://doc.rust-lang.org/book
6. https://diesel.rs/guides
7. https://www.youtube.com/watch?v=HEhhWL1oUTM (for creating the socketio functionality)
8. https://www.youtube.com/watch?v=XZtlD_m59sM (for the refactoring of all the rust code)


Functionality: 
/login
/registration 
/chat 
/userprofile 
/adminpanel (cannot delete/modiy users on front end but can view all users)


Bugs: 
1. When user is updated, it is only updated in the users table. There are helper functions that update the username in messages and conversation
but because the user update async function uses the rust match function, it can only successfully use one of these functions. This causes the conversation to disappear for the user who's username was changed and the user who is recieving ends up either having two conversations with the same person or a non functional conversation with the old username. 
2. Admin panel: the isAdmin variable isn't being changed depending on the recieving payload. It stays the default value which is false. 
3. Prone to refreshing which causes user to log out
4. Currently the server and client are able to handle the "connect", "join", and "message" functionalities provided by socketio. However, while the server is able to emit to the client that a message was received, the client is not able to update it's messages. Whenever this was tried, react would return an error about the misuse of hooks.
5. While testing, the socket would sometimes disconnect and only reconnect when performing ctrl+s in the App.jsx, as well as in useSendMessage.js. 

Other comments: 
1. For testing, there are no test cases for the async functions, instead there are test functions for the helper functions that make up these async functions. 
2. To use quick-dev.rs that is in the tests directory, open a new terminal and run this command from the rust directory: cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture" (doing this multiple times will return an error since the database is expecting unique emails and usernames.If it runs but nothing is printed in the terminal, changing anything in the quick_dev.rs file will cause it to run and print the results to the terminal.






This template provides a minimal setup to get React working in Vite with HMR and some ESLint rules.

Currently, two official plugins are available:

- [@vitejs/plugin-react](https://github.com/vitejs/vite-plugin-react/blob/main/packages/plugin-react/README.md) uses [Babel](https://babeljs.io/) for Fast Refresh
- [@vitejs/plugin-react-swc](https://github.com/vitejs/vite-plugin-react-swc) uses [SWC](https://swc.rs/) for Fast Refresh
