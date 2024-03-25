import * as React from "react";
import { useEffect, useState } from "react";
import { Link, Navigate, useNavigate } from "react-router-dom";
import "./RegistrationPage.css";


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
    const [isValidating, setIsValidating] = useState(false);
    const [email, setEmail] = useState("");
    const [username, setUsername] = useState("");
    const [firstName, setFirstName] = useState("");
    const [lastName, setLastName] = useState("");
    const [password, setPassword] = useState("");
    const [confirmPassword, setConfirmPassword] = useState("");
    const [error, setError] = useState(false);
    //const [values, setValues] = useState({
    //  email: "",
    //  username: "",
    //  firstName: "",
    //  lastName: "",
    //  password: "",
    //  confirmPassword: "",
    //});
  
    const handleEmailChange = (event) => {
        setEmail(event.target.value);
    }

    const handleUsernameChange = (event) => {
        setUsername(event.target.value);
    }

    const handleFirstNameChange = (event) => {
        setFirstName(event.target.value);
    }

    const handleLastNameChange = (event) => {
        setLastName(event.target.value);
    }

    const handlePasswordChange = (event) => {
        setPassword(event.target.value);
    }

    const handleConfirmPasswordChange = (event) => {
        setConfirmPassword(event.target.value);
    }

    const handleSubmit = (event) => {
        event.preventDefault();
        // TODO: Implement authentication API call here
    
        setError(false);
        setIsValidating(true);
        
        document.getElementById('invalid-0').classList.add("d-none");
        document.getElementById('invalid-1').classList.add("d-none");
        document.getElementById('invalid-2').classList.add("d-none");
        document.getElementById('invalid-3').classList.add("d-none");
        document.getElementById('invalid-4').classList.add("d-none");
        document.getElementById('invalid-5').classList.add("d-none");

        //const form = document.getElementsByClassName("needs-validation");
        const firstNameBox = document.getElementById('firstNameBox');
        const lastNameBox = document.getElementById('lastNameBox');
        const usernameBox = document.getElementById('usernameBox');
        const emailBox = document.getElementById('emailBox');
        const passwordBox = document.getElementById('passwordBox');

        const validRegex = /^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$/;

        let i = 0;

        if(firstName === "") {
            console.log("one");
            //setError(true);
            i++;
            firstNameBox.classList.add('border-danger');
            document.getElementById('invalid-0').classList.remove("d-none");
        }
        if(lastName === "") {
            console.log("two");
            //setError(true);
            i++;
            lastNameBox.classList.add('border-danger');
            document.getElementById('invalid-1').classList.remove("d-none");
        }
        if(username === "") {
            console.log("three");
            //setError(true);
            i++;
            usernameBox.classList.add('border-danger');
            document.getElementById('invalid-2').classList.remove("d-none");
        }
        if(!(email.match(validRegex))) {
            console.log("four");
            //setError(true);
            i++;
            emailBox.classList.add('border-danger');
            document.getElementById('invalid-3').classList.remove("d-none");
        }
        if(password === "") {
            console.log("five");
            //setError(true);
            i++;
            passwordBox.classList.add('border-danger');
            document.getElementById('invalid-4').classList.remove("d-none");
            console.log(error);
        }
        if(password != confirmPassword) {
            console.log("six");
            //setError(true);
            i++;
            passwordBox.classList.add('border-danger');
            document.getElementById('invalid-5').classList.remove("d-none");
        }

        console.log(i);
        if(i > 0) {
            console.log("perfect");
            props.setUser(false);
            setIsValidating(false);
            return;
        }

        
        props.setUser(true);
        return;
    }

    return(
        <div id="RegistrationPage">
            <h1 class="display-1 text-white">Registration </h1>
            <form class="needs-validation" onSubmit={handleSubmit} novalidate>
                <div class="form-group row d-flex justify-content-center text-white">
                    <div class="col-md-3 mb-3">
                        <label for="firstNameBox">First name</label>
                        <input type="text" class="form-control" id="firstNameBox" placeholder="First name" value={firstName} onChange={handleFirstNameChange}/>
                        <div id="invalid-0" class="text-danger d-none">
                            Please enter your first name
                        </div>
                    </div>
                    <div class="col-md-3 mb-3">
                        <label for="lastNameBox">Last name</label>
                        <input type="text" class="form-control" id="lastNameBox" placeholder="Last name" value={lastName} onChange={handleLastNameChange}/>
                        <div id="invalid-1" class="text-danger d-none">
                            Please enter your last name
                        </div>
                    </div>
                    <div class="col-md-4 mb-3">
                        <label for="usernameBox">Username</label>
                        <div class="input-group">
                            <div class="input-group-prepend">
                                <span class="input-group-text" id="inputGroupPrepend">@</span>
                            </div>
                            <input type="text" class="form-control" id="usernameBox" placeholder="Username" aria-describedby="inputGroupPrepend" value={username} onChange={handleUsernameChange}/>
                            <div id="invalid-2" class="text-danger d-none">
                                Please choose a username.
                            </div>
                        </div>
                    </div>
                </div>
                <div class="form-group row d-flex justify-content-center text-white">
                    <div class="col-md-6 mb-3">
                        <label for="emailBox">Email</label>
                        <input type="text" class="form-control" id="emailBox" placeholder="Email Address" value={email} onChange={handleEmailChange}/>
                        <div id="invalid-3" class="text-danger d-none">
                            Please provide a valid email.
                        </div>
                    </div>
                </div>
                <div class="form-group row d-flex justify-content-center text-white">
                    <div class="col-md-3 mb-3">
                        <label for="passwordBox">Password</label>
                        <input type="password" class="form-control" id="passwordBox" placeholder="Password" value={password} onChange={handlePasswordChange}/>
                        <div id="invalid-4" class="text-danger d-none">
                            Please enter a password.
                        </div>
                    </div>
                    <div class="col-md-3 mb-3">
                        <label for="confirmPasswordBox">Confirm Password</label>
                        <input type="password" class="form-control" id="confirmPasswordBox" placeholder="Confirm Password" value={confirmPassword} onChange={handleConfirmPasswordChange}/>
                        <div id="invalid-5" class="text-danger d-none">
                            Passwords do not match.
                        </div>
                    </div>
                </div>
            <button class="btn btn-success" type="submit">Submit form</button>
        </form>
    </div>
    )
}  