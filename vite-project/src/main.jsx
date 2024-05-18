import React from "react"
import ReactDOM from "react-dom"
import App from "./components/App/App"
import "./index.css"
import { createRoot } from 'react-dom/client';

createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
