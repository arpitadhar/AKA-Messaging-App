import * as React from "react";
import { Link, useNavigate } from "react-router-dom";
import { useEffect, useRef, useState } from "react";
import { createRoot } from 'react-dom/client';


export default function VerifyCode(props){
  console.log(props.user);
  const navigate = useNavigate();
  if(props.user) {
      navigate("/");
  }
  useEffect(() => {
    const handleClick = (event) => {
        event.preventDefault(); // Prevent default form submission
        const newEmail = document.getElementById("email").value; 
        const newToken = document.getElementById("code").value;
            
            fetch("http://localhost:3000/verify-token", {
                method: "POST",
                body: JSON.stringify({
                email: newEmail,
                token: newToken,
                }),
                headers: {
                    "Content-type": "application/json; charset=UTF-8"
                }
            })
            .then(response => {
                if (response.ok) {
                    alert("Correct code");
                    return response.json(); 
                } else {
                    throw new Error("Failed to send reset password email");
                }
            })
            .then(json => {
                console.log(json);
                window.location.href = "/chat";
            })
            .catch(error => {
                console.error("Error:", error);
            });
            document.getElementById("email").value = "";
            document.getElementById("code").value = "";
            
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
          <div className = "text">Enter Code</div>
          <div className = "underline"></div>
      </div>
      <div className = "inputs">
          <div className = "input">
              <input id = "email" type = "email" placeholder='email' required></input>
          </div> 
          <div className = "input">
              <input id = "code" type = "token" placeholder='code' required></input>
          </div> 
      </div>
      <button id ="enter">Submit</button>
      </form>
    </div>
    </body>
  )
}
