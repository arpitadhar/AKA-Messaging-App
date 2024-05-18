import * as React from "react";
import { Link, useNavigate } from "react-router-dom";
import { useEffect, useRef, useState } from "react";
import { createRoot } from 'react-dom/client';


export default function newPassword(props){
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
            .then((response) => response.json())
            .then((json) => console.log(json))
            
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
