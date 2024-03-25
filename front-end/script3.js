document.addEventListener("DOMContentLoaded", function() {
    const loginButton = document.getElementById("Login");

    loginButton.addEventListener("click", function(event) {
        event.preventDefault(); // Prevent default form submission

        // Get email and password inputs
        const email = document.getElementById("email").value;
        const password = document.getElementById("password").value;

        // Make login API call
        fetch("http://localhost:3000/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ email: email, password: password })
        })
        .then(response => {
            if (!response.ok) {
                throw new Error("Network response was not ok");
            }
            return response.json();
        })
        .then(data => {
            // Display response as an alert
            sessionStorage.setItem("username", data);
            alert("Login successful. Welcome " + sessionStorage.getItem("username"));
            if(sessionStorage.username) {
                window.location.href = "user_homepage.html";
            }
        })
        .catch(error => {
            console.error("Error during login:", error);
            // Display error message as an alert
            alert("Error during login: " + error.message);
        });
    });
});
