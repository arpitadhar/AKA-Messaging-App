import * as React from "react";
import { useEffect, useState } from "react";
import { Link, Navigate, useNavigate } from "react-router-dom";
import "./RegistrationPage.css";

export default function RegistrationPage(){
    useEffect(() => {

        const handleClick = () => {
            // This line sets up an event listener for the click event on the "Post" button.
            // When the button is clicked, the function provided as the second argument will run.
            
            // Step 2: Get the comment from the textarea
            const newUserName = document.getElementById("user").value;
            const newFirstName = document.getElementById("first_name").value; 
            const newLastName = document.getElementById("last_name").value; 
            const newPassword = document.getElementById("password").value;
            const confirmPassword = document.getElementById("confirm_password").value; 
            const newEmail = document.getElementById("email").value; 
            const defaultToken = "no_token"; 

            if (newPassword == '' || confirmPassword == ''){
                alert("Please enter a password"); 
                return false; 
            }
            
            else if (newPassword != confirmPassword){
               alert("Passwords do not match"); 
               return false; 
            }
            else if(newPassword.length < 12 || newPassword.length > 25){
                alert("Password too short or too long, must not be less than 12 characters and must be less than 25"); 
                return false; 
            }
            else{
                alert("Account created!"); 
            }
        
            
            
        
            fetch("http://localhost:3000/create-users", {
                method: "POST",
                body: JSON.stringify({
                username: newUserName,
                first_name: newFirstName, 
                last_name: newLastName, 
                password: newPassword,
                email: newEmail,
                token: defaultToken, 

                }),
                headers: {
                    "Content-type": "application/json; charset=UTF-8"
                }
            })
            .then((response) => response.json())
            .then((json) => console.log(json));
        
            document.getElementById("user").value = "";
            document.getElementById("first_name").value=""; 
            document.getElementById("last_name").value=""; 
            document.getElementById("password").value = "";
            document.getElementById("email").value = "";
            document.getElementById("confirm_password").value = ""; 
            
        };
        const enterButton = document.getElementById("enter");
            if (enterButton) {
                enterButton.addEventListener("click", handleClick);
            }
    
            return () => {
                if (enterButton) {
                    enterButton.removeEventListener("click", handleClick);
                }
            }
        }, []); 
    return(
        <body id = "registration_body">
        <div className = "container" id = "registration_container" style={{ fontFamily: 'Rubik, sans-serif' }}>
        <form action="">
        <div className = "header" id="reg-header">
            <div className = "text">Register</div>
            <div className = "underline"></div>
        </div>
        <div className = "inputs">
            <div className = "input">
                <input id="user" type = "text" placeholder='Username' required></input>
            </div> 
            <div className = "input">
                <input id="first_name" type = "text" placeholder='First Name' required></input>
            </div> 
            <div className = "input">
                <input id="last_name" type = "text" placeholder='Last Name' required></input>
            </div> 
            <div className = "input">
                <input id="email" type = "text" placeholder='Email' required></input>
            </div> 
            <div className = "input">
                <input id="password" type = "password" placeholder='Password' required></input>
            </div> 
            <div className = "input">
                <input id="confirm_password" type = "password" placeholder='Confirm Password' required></input>
            </div> 
        </div>
       <br></br>
        <button id = "enter">Submit</button>
        <div className="Register">
            <p>Already have an account? <a href = "/Login">Login</a></p>
        </div>
        </form>
      </div>
      </body>
    )
}


/*
export default function RegistrationPage(props) {
    console.log(props.user);
    const navigate = useNavigate();
    if(props.user) {
        navigate("/");
    }

    return (
        <div className="registration-page">
          <RegistrationForm user={props.user} setUser={props.setUser} />
        </div>
    );
}

export function RegistrationForm(props) {
    // const [isValidating, setIsValidating] = useState(false);
    // const [email, setEmail] = useState("");
    // const [username, setUsername] = useState("");
    // const [firstName, setFirstName] = useState("");
    // const [lastName, setLastName] = useState("");
    // const [password, setPassword] = useState("");
    // const [confirmPassword, setConfirmPassword] = useState("");
    // const [error, setError] = useState(false);
    // //const [values, setValues] = useState({
    // //  email: "",
    // //  username: "",
    // //  firstName: "",
    // //  lastName: "",
    // //  password: "",
    // //  confirmPassword: "",
    // //});
  
    // const handleEmailChange = (event) => {
    //     setEmail(event.target.value);
    // }

    // const handleUsernameChange = (event) => {
    //     setUsername(event.target.value);
    // }

    // const handleFirstNameChange = (event) => {
    //     setFirstName(event.target.value);
    // }

    // const handleLastNameChange = (event) => {
    //     setLastName(event.target.value);
    // }

    // const handlePasswordChange = (event) => {
    //     setPassword(event.target.value);
    // }

    // const handleConfirmPasswordChange = (event) => {
    //     setConfirmPassword(event.target.value);
    // }

    // const handleSubmit = (event) => {
    //     event.preventDefault();
    //     // TODO: Implement authentication API call here
    
    //     setError(false);
    //     setIsValidating(true);
        
    //     document.getElementById('invalid-0').classList.add("d-none");
    //     document.getElementById('invalid-1').classList.add("d-none");
    //     document.getElementById('invalid-2').classList.add("d-none");
    //     document.getElementById('invalid-3').classList.add("d-none");
    //     document.getElementById('invalid-4').classList.add("d-none");
    //     document.getElementById('invalid-5').classList.add("d-none");

    //     //const form = document.getElementsByClassName("needs-validation");
    //     const firstNameBox = document.getElementById('firstNameBox');
    //     const lastNameBox = document.getElementById('lastNameBox');
    //     const usernameBox = document.getElementById('usernameBox');
    //     const emailBox = document.getElementById('emailBox');
    //     const passwordBox = document.getElementById('passwordBox');

    //     const validRegex = /^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$/;

    //     let i = 0;

    //     if(firstName === "") {
    //         console.log("one");
    //         //setError(true);
    //         i++;
    //         firstNameBox.classList.add('border-danger');
    //         document.getElementById('invalid-0').classList.remove("d-none");
    //     }
    //     if(lastName === "") {
    //         console.log("two");
    //         //setError(true);
    //         i++;
    //         lastNameBox.classList.add('border-danger');
    //         document.getElementById('invalid-1').classList.remove("d-none");
    //     }
    //     if(username === "") {
    //         console.log("three");
    //         //setError(true);
    //         i++;
    //         usernameBox.classList.add('border-danger');
    //         document.getElementById('invalid-2').classList.remove("d-none");
    //     }
    //     if(!(email.match(validRegex))) {
    //         console.log("four");
    //         //setError(true);
    //         i++;
    //         emailBox.classList.add('border-danger');
    //         document.getElementById('invalid-3').classList.remove("d-none");
    //     }
    //     if(password === "") {
    //         console.log("five");
    //         //setError(true);
    //         i++;
    //         passwordBox.classList.add('border-danger');
    //         document.getElementById('invalid-4').classList.remove("d-none");
    //         console.log(error);
    //     }
    //     if(password != confirmPassword) {
    //         console.log("six");
    //         //setError(true);
    //         i++;
    //         passwordBox.classList.add('border-danger');
    //         document.getElementById('invalid-5').classList.remove("d-none");
    //     }

    //     console.log(i);
    //     if(i > 0) {
    //         console.log("perfect");
    //         props.setUser(false);
    //         setIsValidating(false);
    //         return;
    //     }

    //     fetch("http://localhost:3000/create-users", {
    //         method: "POST",
    //         body: JSON.stringify({
    //         username: usernameBox,
    //         first_name: firstNameBox, 
    //         last_name: lastNameBox, 
    //         password: passwordBox,
    //         email: emailBox,
    //         }),
    //         headers: {
    //             "Content-type": "application/json; charset=UTF-8"
    //         }
    //     })
    //     .then((response) => response.json())
    //     .then((json) => console.log(json));
    //     props.setUser(true);
    //     return;
    // }
    
    return(
        <div className = "container">
        <form action="">
        <div className = "header">
            <div className = "text">Register</div>
            <div className = "underline"></div>
        </div>
        <div className = "inputs">
            <div className = "input">
                <input id="user" type = "text" placeholder='Username' required></input>
            </div> 
            <div className = "input">
                <input id="firstNameBox" type = "text" placeholder='First Name' required></input>
            </div> 
            <div className = "input">
                <input id="lastNameBox" type = "text" placeholder='Last Name' required></input>
            </div> 
            <div className = "input">
                <input id="emailBox" type = "text" placeholder='Email' required></input>
            </div> 
            <div className = "input">
                <input id="passwordBox" type = "password" placeholder='Password' required></input>
            </div> 
            <div className = "input">
                <input id="confirm_password" type = "password" placeholder='Confirm Password' required></input>
            </div> 
        </div>
       <br></br>
        <button id = "enter">Submit</button>
        <div className="Register">
            <p>Already have an account? <a href = "/login">Login</a></p>
        </div>
        </form>
      </div>
    //     <div id="RegistrationPage">
    //         <h1 class="display-1 text-white">Registration </h1>
    //         <form class="needs-validation" onSubmit={handleSubmit} novalidate>
    //             <div class="form-group row d-flex justify-content-center text-white">
    //                 <div class="col-md-3 mb-3">
    //                     <label for="firstNameBox">First name</label>
    //                     <input type="text" class="form-control" id="firstNameBox" placeholder="First name" value={firstName} onChange={handleFirstNameChange}/>
    //                     <div id="invalid-0" class="text-danger d-none">
    //                         Please enter your first name
    //                     </div>
    //                 </div>
    //                 <div class="col-md-3 mb-3">
    //                     <label for="lastNameBox">Last name</label>
    //                     <input type="text" class="form-control" id="lastNameBox" placeholder="Last name" value={lastName} onChange={handleLastNameChange}/>
    //                     <div id="invalid-1" class="text-danger d-none">
    //                         Please enter your last name
    //                     </div>
    //                 </div>
    //                 <div class="col-md-4 mb-3">
    //                     <label for="usernameBox">Username</label>
    //                     <div class="input-group">
    //                         <div class="input-group-prepend">
    //                             <span class="input-group-text" id="inputGroupPrepend">@</span>
    //                         </div>
    //                         <input type="text" class="form-control" id="usernameBox" placeholder="Username" aria-describedby="inputGroupPrepend" value={username} onChange={handleUsernameChange}/>
    //                         <div id="invalid-2" class="text-danger d-none">
    //                             Please choose a username.
    //                         </div>
    //                     </div>
    //                 </div>
    //             </div>
    //             <div class="form-group row d-flex justify-content-center text-white">
    //                 <div class="col-md-6 mb-3">
    //                     <label for="emailBox">Email</label>
    //                     <input type="text" class="form-control" id="emailBox" placeholder="Email Address" value={email} onChange={handleEmailChange}/>
    //                     <div id="invalid-3" class="text-danger d-none">
    //                         Please provide a valid email.
    //                     </div>
    //                 </div>
    //             </div>
    //             <div class="form-group row d-flex justify-content-center text-white">
    //                 <div class="col-md-3 mb-3">
    //                     <label for="passwordBox">Password</label>
    //                     <input type="password" class="form-control" id="passwordBox" placeholder="Password" value={password} onChange={handlePasswordChange}/>
    //                     <div id="invalid-4" class="text-danger d-none">
    //                         Please enter a password.
    //                     </div>
    //                 </div>
    //                 <div class="col-md-3 mb-3">
    //                     <label for="confirmPasswordBox">Confirm Password</label>
    //                     <input type="password" class="form-control" id="confirmPasswordBox" placeholder="Confirm Password" value={confirmPassword} onChange={handleConfirmPasswordChange}/>
    //                     <div id="invalid-5" class="text-danger d-none">
    //                         Passwords do not match.
    //                     </div>
    //                 </div>
    //             </div>
    //         <button class="btn btn-success" type="submit">Submit form</button>
    //     </form>
    // </div>
    )
}  
window.onload=function(){
document.getElementById("enter").addEventListener("click", function () {
    // This line sets up an event listener for the click event on the "Post" button.
    // When the button is clicked, the function provided as the second argument will run.
    
    // Step 2: Get the comment from the textarea
    const newUserName = document.getElementById("usernameBox").value;
    const newFirstName = document.getElementById("firstNameBox").value; 
    const newLastName = document.getElementById("lastNameBox").value; 
    const newPassword = document.getElementById("passwordBox").value;
    const confirmPassword = document.getElementById("confirm_password").value; 
    const newEmail = document.getElementById("emailBox").value; 
    
    if (newPassword == '' || confirmPassword == ''){
        alert("Please enter a password"); 
        return false; 
    }
    
    else if (newPassword != confirmPassword){
       alert("Passwords do not match"); 
       return false; 
    }
    else if(newPassword.length < 12 || newPassword.length > 25){
        alert("Password too short or too long, must not be less than 12 characters and must be less than 25"); 
        return false; 
    }
    else{
        alert("Account created!"); 
    }

    
    

    fetch("http://localhost:3000/create-users", {
        method: "POST",
        body: JSON.stringify({
        username: newUserName,
        first_name: newFirstName, 
        last_name: newLastName, 
        password: newPassword,
        email: newEmail,
        }),
        headers: {
            "Content-type": "application/json; charset=UTF-8"
        }
    })
    .then((response) => response.json())
    .then((json) => console.log(json));

    document.getElementById("user").value = "";
    document.getElementById("first_name").value=""; 
    document.getElementById("last_name").value=""; 
    document.getElementById("password").value = "";
    document.getElementById("email").value = "";
    document.getElementById("confirm_password").value = ""; 
    
});
}
*/