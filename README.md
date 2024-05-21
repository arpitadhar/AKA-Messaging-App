# RUST + React + Vite 
AKA MESSAGING APP 

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


Functionality: 
/login
/registration 
/forgot-password
/verify-code 
/changepassword 
/chat 
/userprofile 
/adminpanel (cannot delete/modiy users on front end but can view all users)


Bugs: 
1. When user is updated, it is only updated in the users table. There are helper functions that update the username in messages and conversation
but because the user update async function uses the rust match function, it can only successfully use one of these functions. This causes the conversation to disappear for the user who's username was changed and the user who is recieving ends up either having two conversations with the same person or a non functional conversation with the old username. 
2. Admin panel: the isAdmin variable isn't being changed depending on the recieving payload. It stays the default value which is false. 
3. Prone to refreshing which causes user to log out 

Other comments: 
1. For testing, there are no test cases for the async functions, instead there are test functions for the helper functions that make up these async functions. 







This template provides a minimal setup to get React working in Vite with HMR and some ESLint rules.

Currently, two official plugins are available:

- [@vitejs/plugin-react](https://github.com/vitejs/vite-plugin-react/blob/main/packages/plugin-react/README.md) uses [Babel](https://babeljs.io/) for Fast Refresh
- [@vitejs/plugin-react-swc](https://github.com/vitejs/vite-plugin-react-swc) uses [SWC](https://swc.rs/) for Fast Refresh
