import * as React from "react";
import { Link, useNavigate } from "react-router-dom";
import { useEffect, useRef, useState } from "react";
import { createRoot } from 'react-dom/client';


export default function ForgotPassword(props){
  console.log(props.user);
  const navigate = useNavigate();
  if(props.user) {
      navigate("/");
  }
  useEffect(() => {
    const handleClick = (event) => {
        event.preventDefault(); // Prevent default form submission
        const newEmail = document.getElementById("email").value; 
            
            fetch("http://localhost:3000/forgot-password", {
                method: "POST",
                body: JSON.stringify({
                email: newEmail,
                }),
                headers: {
                    "Content-type": "application/json; charset=UTF-8"
                }
            })
            .then(response => {
                if (response.ok) {
                    alert("Reset password email sent! If we have an account with your email, you will receive a link to reset your password");
                    return response.json(); 
                } else {
                    throw new Error("Failed to send reset password email");
                }
            })
            .then(json => {
                console.log(json);
                window.location.href = "/verify";
            })
            .catch(error => {
                console.error("Error:", error);
            });
    
            document.getElementById("email").value = "";
            
    };

    const enterButton = document.getElementById("enter");
    enterButton.addEventListener("click", handleClick);

    return () => {
        enterButton.removeEventListener("click", handleClick);
    };
   }, []); 
  
  return(
    <body id="registration_body">
    <div className = "container" id = "registration_container" style={{ fontFamily: 'Rubik, sans-serif' }}>
      <form action="">
      <div className = "header" id="reg-header">
          <div className = "text">Forgot Password</div>
          <div className = "underline"></div>
      </div>
      <div className = "inputs">
          <div className = "input">
              <input id = "email" type = "email" placeholder='Email' required></input>
          </div> 
      </div>
      <button id ="enter">Submit</button>
      </form>
    </div>
    </body>
  )
}
