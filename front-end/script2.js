window.onload=function(){

document.getElementById("enter").addEventListener("click", function () {
    // This line sets up an event listener for the click event on the "Post" button.
    // When the button is clicked, the function provided as the second argument will run.
    
    // Step 2: Get the comment from the textarea
    const newUserName = document.getElementById("user").value;
    const newFirstName = document.getElementById("first_name").value; 
    const newLastName = document.getElementById("last_name").value; 
    const newPassword = document.getElementById("password").value;
    const confirmPassword = document.getElementById("confirm_password").value; 
    const newEmail = document.getElementById("email").value; 
    
    if (newPassword == '' || confirmPassword == ''){
        alert("Please enter a password"); 
        return false; 
    }
    
    else if (newPassword != confirmPassword){
       alert("Passwords do not match"); 
       return false; 
    }
    else{
        alert("Account created!"); 
    }

    
    

    fetch("http://localhost:3000/create-user", {
        method: "POST",
        body: JSON.stringify({
        username: newUserName,
        first_name: newFirstName, 
        last_name: newLastName, 
        email: newEmail,
        password: newPassword,
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
    

    // // Step 3: Create a new paragraph element for the comment
    // const commentContainer = document.getElementById("inner-display");
    // const commentElement = document.createElement("p");
    // commentElement.innerText = new;

    // // Step 4: Append the comment to the comment container
    // commentContainer.appendChild(commentElement);

    // // Clear the textarea after posting the comment
    // document.getElementById("message").value = "";
});
}
