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
        const newToken = document.getElementById("code").value;
        if(newToken === localStorage.getItem("token")){
            alert("correcto");
        }
        else{
            alert("wrong token"); 
        }
        
        
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
              <input id = "code" type = "token" placeholder='code' required></input>
          </div> 
      </div>
      <button id ="enter">Submit</button>
      </form>
    </div>
    </body>
  )
}
