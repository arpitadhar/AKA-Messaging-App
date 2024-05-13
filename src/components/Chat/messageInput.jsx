//import { BsSend } from "react-icons/bs"; 
import { useEffect, useState } from "react";
const MessageInput = () => {
  
  useEffect(() => {

    const handleClick = () => {
        // This line sets up an event listener for the click event on the "Post" button.
        // When the button is clicked, the function provided as the second argument will run.
        
        // Step 2: Get the comment from the textarea
        const messageBody = document.getElementById("message").value;

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
    <form className='inputMessage'>
        <div className = "messageInput">
                <input id="message" type = "message" placeholder='type a message!' required></input>
        </div> 
        {/* <div className = 'messageInput'>
            <input type = "text" placeholder = 'Send a message' id = "message"></input>
            <botton type = "submit"></botton>
            <BsSend className="sendButton"/> 
        </div> */}
    </form>
  )

}
export default MessageInput; 
