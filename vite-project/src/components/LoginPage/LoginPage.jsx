import * as React from "react";
import { Link, useNavigate } from "react-router-dom";
import { useEffect, useRef, useState } from "react";
import { createRoot } from 'react-dom/client';
import "./LoginPage.css";
import { BsChevronCompactLeft } from "react-icons/bs";
//import {useLogin} from '/Users/arpitadhar/Downloads/AKA-Messaging-App-new-react/vite-project/src/context/login-provider.jsx';
//import {loggedIn} from '../LoginHandler/LoginHandler'; 

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
        const loginRequest = fetch("http://localhost:3000/login", {
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
          localStorage.setItem("email", data); // Assuming email is returned from the server
          console.log(localStorage.getItem("email")); 
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

/*
export default function LoginPage(props) {
 console.log(props.user);
  const navigate = useNavigate();
  if(props.user) {
      navigate("/");
  }

  const [isValidating, setIsValidating] = useState(false);
  const [error, setError] = useState('')
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const handleEmailChange = (event) => {
    setEmail(event.target.value);
  }

  const handlePasswordChange = (event) => {
    setPassword(event.target.value);
  }

  const handleSubmit = (event) => {
    event.preventDefault();
    // TODO: Implement authentication API call here

    setError("");
    setIsValidating(true);

    const emailInputBox = document.getElementById('inputEmail');
    const passwordInputBox = document.getElementById('inputPassword');

    //clear red borders for previous erroneous inputs
    if(emailInputBox.classList.contains('border-danger')) {
      emailInputBox.classList.remove('border-danger');
    } 
    if(passwordInputBox.classList.contains('border-danger')) {
      passwordInputBox.classList.remove('border-danger');
    } 

    console.log(email, password);

    const validRegex = /^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$/;

    if (!(email.match(validRegex))) {
      setError("Please enter a valid email");
      console.log(error);
      setIsValidating(false);

      emailInputBox.classList.add('border-danger');

      return;
    }

    if(password === "") {
      setError("Please enter your password");
      console.log(error);
      setIsValidating(false);

      passwordInputBox.classList.add('border-danger');

      return;
    }

    if(false) {
      setError("Email or Password is incorrect");
      console.log(error);
      setIsValidating(false);

      emailInputBox.classList.add('border-danger');
      passwordInputBox.classList.add('border-danger');

      return;
    }
    props.setUser(true);
    return
  }

  return (
    <div className = "container">
        <form action="">
        <div className = "header">
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
            <a href="#">Forgot Password?</a>
        </div>
        <button id ="Login">Login</button>
        <div className="Register">
            <p>Don't have an account? Sign up{" "}<a href = "/register">here</a></p>
        </div>
        </form>
      </div>
    // <div id="loginPage">
    //   <h1 class="display-1 text-white">Welcome Back! </h1>
    //   <form id="login-form" onSubmit={handleSubmit}>
    //     <div id="row1" class="form-group row d-flex justify-content-center">
    //       <label for="inputEmail" class="col-sm-1 col-form-label-lg text-white">Email: </label>
    //       <div class="col-sm-3  ">
    //         <input type="text" class="form-control" id="inputEmail" aria-describedby="emailHelp" placeholder="Enter email" value={email} onChange={handleEmailChange} />
    //       </div>
    //     </div>
    //     <div class="form-group row d-flex justify-content-center text-white">
    //       <label for="inputPassword" class="col-sm-1 col-form-label-lg">Password:</label>
    //       <div class="col-sm-3 ">
    //         <input type="password" class="form-control" id="inputPassword" placeholder="Password" value={password} onChange={handlePasswordChange} />
    //       </div>
    //     </div>
    //     <h4 id="errorMessage" class="text-danger">{error ? error : ""}</h4>
    //     <button type="submit" class="btn btn-success">Login</button>
    //   </form>

    //   <div className="footer">
    //     <p class="text-white">
    //       Don't have an account? Sign up{" "}
    //       <Link className="auth-link" to="/register">here.</Link>
    //     </p>
    //   </div>
    // </div>
  )
}

{/* <div className="footer">
        <p class="text-white">
          Don't have an account? Sign up{" "}
          <Link className="auth-link" to="/register">here.</Link>
        </p>
      </div> */
    
