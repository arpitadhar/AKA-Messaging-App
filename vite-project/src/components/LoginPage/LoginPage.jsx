import * as React from "react";
import { Link, useNavigate } from "react-router-dom";
import { useEffect, useRef, useState } from "react";
import { createRoot } from 'react-dom/client';
import "./LoginPage.css";

export default function LoginPage({isLoggedIn, setIsLoggedIn}){
 // const {setIsLoggedIn} = useLogin()
 
  //console.log(props.user);
  const navigate = useNavigate();

  // if(props.user) {
  //     navigate("/");
  // }
  useEffect(() => {
    const handleClick = (event) => {
        event.preventDefault(); // Prevent default form submission

        // Get email and password inputs
        const email = document.getElementById("email").value;
        const password = document.getElementById("password").value;

        // Make login API call
        fetch("http://localhost:3000/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ email, password })
        })
        .then(response => {
            if (!response.ok) {
                throw new Error("Network response was not ok");
            }
            return response.json();
        })
        .then(data => {
          console.log(data);
          sessionStorage.setItem("email", data); // Assuming email is returned from the server
          console.log(sessionStorage.getItem("email")); 
          setIsLoggedIn(true); // Update isLoggedIn state
          navigate("/chat"); // Redirect to chat page
        })
        .catch(error => {
            console.error("Error during login:", error);
            // Display error message as an alert
            alert("Error during login: " + error.message);
        });
        console.log(isLoggedIn); 
    };

    const enterButton = document.getElementById("Login");
    enterButton.addEventListener("click", handleClick);

    return () => {
        enterButton.removeEventListener("click", handleClick);
    };
  
   }, [navigate, setIsLoggedIn, isLoggedIn]); 
  
  return(
    <body id="registration_body">
    <div className = "container" id = "registration_container" style={{ fontFamily: 'Rubik, sans-serif' }}>
      <form action="">
      <div className = "header" id="reg-header">
          <div className = "text">Login</div>
          <div className = "underline"></div>
      </div>
      <div className = "inputs">
          <div className = "input">
              <input id = "email" type = "email" placeholder='Email' required></input>
          </div> 
          <div className = "input">
              <input id = "password" type = "password" placeholder='Password' required></input>
          </div> 
      </div>
      <div className = "forgotPassword">
          <a href="/forgot">Forgot Password?</a>
      </div>
      <button id ="Login">Login</button>
      <div className="Register">
          <p><a href = "/register">Register</a></p>
      </div>
      </form>
    </div>
    </body>
  )
}

