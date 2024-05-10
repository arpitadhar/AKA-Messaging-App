import { IoSearch } from "react-icons/io5";
import { useEffect, useState } from "react";

const SearchInput = () =>{
    useEffect(() => {
        const handleClick = () => {
         const senderName = sessionStorage.getItem("username");
         const receiverName = document.getElementById("searchInput").value;
 
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
             document.getElementById("searchInput").value = "";
             // Refresh conversation list after creating a new conversation
             refreshConversationList(); // Added refresh
         })
         .catch(error => {
             console.error("Error creating conversation:", error);
             alert("Error creating conversation: " + error.message);
             document.getElementById("searchInput").value = "";
         });
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
        <form className='flex gap-2' id = "search">
            <input type = 'text' placeholder ='..Search..' className = 'input input-bordered rounded-full' id ="searchInput"/>
            <button type='submit' className='btn btn-circle bg-transparent text-white' id = "enter">
            <IoSearch className="searchBar"/>
            </button>

        </form>
    )
}
export default SearchInput