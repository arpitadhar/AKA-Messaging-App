window.onload=function(){
    // Step 1: Add an event listener to the "Post" button
    document.getElementById("submitPost").addEventListener("click", function () {
        // This line sets up an event listener for the click event on the "Post" button.
        // When the button is clicked, the function provided as the second argument will run.

        // Step 2: Get the comment from the textarea
        const newUserName = document.getElementById("display-name").value;
        const newComment = document.getElementById("message").value;


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
        .then((response) => response.json())
        .then((json) => console.log(json));

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
        .then((response) => response.json())
        .then((json) => console.log(json));

        // Step 3: Create a new paragraph element for the comment
        const commentContainer = document.getElementById("inner-display");
        const commentElement = document.createElement("p");
        commentElement.innerText = newComment;

        // Step 4: Append the comment to the comment container
        commentContainer.appendChild(commentElement);

        // Clear the textarea after posting the comment
        document.getElementById("message").value = "";
    });
}
