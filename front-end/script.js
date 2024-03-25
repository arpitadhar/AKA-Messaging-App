window.onload=function(){
    //Event listener to the "sumbitPost" button
    document.getElementById("submitPost").addEventListener("click", function () {
        // This line sets up an event listener for the click event on the "Post" button.
        // When the button is clicked, the function provided as the second argument will run.

        //Get the message from the textarea
        const newUserName = document.getElementById("display-name").value;
        const newComment = document.getElementById("message").value;

        //POST request with username and message payload
        fetch("http://localhost:3000/create-message", {
            method: "POST",
            body: JSON.stringify({
            username: newUserName,
            message: newComment
            }),
            headers: {
                "Content-type": "application/json; charset=UTF-8"
            }
        })
        .then((response) => response.json());

        // Clear the textarea after posting the message
        document.getElementById("message").value = "";

        //I suspected that the following GET request queries the server before 
        //the messages table is updated from the previous POST request
        //so added delay of one second in an attempt to give server time to update
        //
        //it was futile
        setTimeout(function(){
        }, 1000);

        // Clear the display area after posting the message
        document.getElementById("inner-display").textContent = '';

        //GET request for all stored message
        fetch("http://localhost:3000/messages", {
            method: "GET",
        })
        .then((response) => response.json())
        .then((json) => {
            const responseJson = JSON.parse(json);
            
            //for each item in array append "username" + ":" + "message" to display element
            for (const post in responseJson) {
                
                const responseMessage = responseJson[parseInt(post)];

                const commentContainer = document.getElementById("inner-display");
                const commentElement = document.createElement("p");
                commentElement.innerText = responseMessage.user_id + ": " + responseMessage.message;

                commentContainer.appendChild(commentElement);
            }

        })

    });

    //Event listener to the "sumbitPost" button
    document.getElementById("login").addEventListener("click", function () {
        //Get the message from the textarea
        const email_input = document.getElementById("email").value;
        const password_input = document.getElementById("password").value;

        fetch("http://localhost:3000//login", {
            method: "POST",
            body: JSON.stringify({
            email: email_input,
            password: password_input,
            }),
            headers: {
                "Content-type": "application/json; charset=UTF-8"
            }
        })
        .then((response) => response.json())
        .then((json) => console.log(json));
    });
}
