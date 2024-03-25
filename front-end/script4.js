document.addEventListener("DOMContentLoaded", function() {
    const createConversationForm = document.getElementById("createConversationForm");
    const conversationsList = document.getElementById("conversationsList"); // Added reference to conversationsList

    createConversationForm.addEventListener("submit", function(event) {
        event.preventDefault(); 

        const senderName = sessionStorage.getItem("username");
        const receiverName = document.getElementById("receiverName").value;

        // Make create-conversation API call
        fetch("http://localhost:3000/create-conversation", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ username_sender: senderName, username_receiver: receiverName })
        })
        .then(response => {
            if (!response.ok) {
                throw new Error("Network response was not ok");
            }
            return response.json();
        })
        .then(data => {
            alert("Conversation created successfully!");
            document.getElementById("receiverName").value = "";
            // Refresh conversation list after creating a new conversation
            refreshConversationList(); // Added refresh
        })
        .catch(error => {
            console.error("Error creating conversation:", error);
            alert("Error creating conversation: " + error.message);
            document.getElementById("receiverName").value = "";
        });
    });

    // Function to fetch and display conversations
    function refreshConversationList() {
        fetch("http://localhost:3000/conversations", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(sessionStorage.getItem("username"))
        })
        .then(response => {
            if (!response.ok) {
                throw new Error("Network response was not ok");
            }
            return response.json();
        })
        .then((json) => {
            const responseJson = JSON.parse(json);
            console.log(responseJson);
            for (const conversations in responseJson) {                    
                const responseMessage = responseJson[parseInt(conversations)];
                
                const commentContainer = document.getElementById("conversationListContainer");

                const button = document.createElement("button");
                button.innerText = responseMessage.id;
                button.classList.add("conversationsList");
                button.addEventListener("click", function() {
                    // Clear the previous value if it exists
                    if (sessionStorage.getItem("selectedConversation")) {
                        sessionStorage.removeItem("selectedConversation");
                    }

                    sessionStorage.setItem("selectedConversation",this.innerText);
                    displayMessagesForSelectedConversation();
                });
                commentContainer.appendChild(button);
            }
        })   
        .catch(error => {
            console.error("Error fetching conversations:", error);
            alert("Error fetching conversations: " + error.message);
        });
    }
    
    // Initial call to fetch and display conversations when the page loads
    refreshConversationList();

    //display messages
    function displayMessagesForSelectedConversation() {
        document.getElementById("inner-display").innerHTML = "";
        fetch("http://localhost:3000/messages", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(sessionStorage.getItem("selectedConversation"))
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
    }

    //sending messages
    const sendMessageForm = document.getElementById("sendMessageForm");

    sendMessageForm.addEventListener("submit", function (event){
        event.preventDefault();

        const username = sessionStorage.getItem("username");
        const conversation = sessionStorage.getItem("selectedConversation");
        const messageBody = document.getElementById("message").value;

        fetch("http://localhost:3000/create-message" , {
            method: "Post",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({username: sessionStorage.getItem("username"), message: messageBody, conversation_id: sessionStorage.getItem("selectedConversation")})
        })
        .then((response) => {
            if(!response.ok) {
                throw new Error("Network response was not okay")
        }
            return response.json();
        })
        .then((json) => {
            document.getElementById("message").value = "";
            displayMessagesForSelectedConversation();
        })
        .catch(error => {
            console.error("Error sending message:", error);
        })
    });
});
