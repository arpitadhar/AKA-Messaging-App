import { IoSearch } from "react-icons/io5";
import { useEffect, useState } from "react";

const SearchInput = () =>{
    const [receiverName, setReceiverName] = useState('');

    useEffect(() => {
        const handleClick = () => {
            const senderUser = localStorage.getItem("email");
            const senderName = senderUser.replace(/^"(.*)"$/, '$1');
            console.log(receiverName); 
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
                setReceiverName(''); // Reset input value
                // Refresh conversation list after creating a new conversation
                //refreshConversationList(); // Added refresh
            })
            .catch(error => {
                console.error("Error creating conversation:", error);
                alert("Error creating conversation: " + error.message);
                setReceiverName(''); // Reset input value
            });
        }; 

        const handleSubmit = (event) => {
            event.preventDefault(); // Prevent form submission
            handleClick(); // Call handleClick function
        };

        const form = document.getElementById("search");
        if (form) {
            form.addEventListener("submit", handleSubmit);
        }

        return () => {
            if (form) {
                form.removeEventListener("submit", handleSubmit);
            }
        };
    }, [receiverName]); 

    return(
        <form className='flex gap-2' id="search">
            <input 
                type='text' 
                placeholder='..Search..' 
                className='input input-bordered rounded-full' 
                value={receiverName}
                onChange={(e) => setReceiverName(e.target.value)}
            />
            <button type='submit' className='btn btn-circle bg-transparent text-white'>
                <IoSearch className="searchBar" />
            </button>
        </form>
    )
}

export default SearchInput;


// import { IoSearch } from "react-icons/io5";
// import { useEffect, useState } from "react";

// const SearchInput = () =>{
//     useEffect(() => {
//         const handleClick = () => {
//          const senderName = sessionStorage.getItem("email");
//          const receiverName = document.getElementById("searchInput").value;
//          console.log(receiverName); 
//          // Make create-conversation API call
//          fetch("http://localhost:3000/create-conversation", {
//              method: "POST",
//              headers: {
//                  "Content-Type": "application/json"
//              },
//              body: JSON.stringify({ username_sender: senderName, username_receiver: receiverName })
//          })
//          .then(response => {
//              if (!response.ok) {
//                  throw new Error("Network response was not ok");
//              }
//              return response.json();
//          })
//          .then(data => {
//              alert("Conversation created successfully!");
//              document.getElementById("searchInput").value = "";
//              // Refresh conversation list after creating a new conversation
//              refreshConversationList(); // Added refresh
//          })
//          .catch(error => {
//              console.error("Error creating conversation:", error);
//              alert("Error creating conversation: " + error.message);
//              document.getElementById("searchInput").value = "";
//          });
//         }; 
//         const enterButton = document.getElementById("enter");
//             if (enterButton) {
//                 enterButton.addEventListener("click", handleClick);
//             }
    
//             return () => {
//                 if (enterButton) {
//                     enterButton.removeEventListener("click", handleClick);
//                 }
//             }
       
//      }, []); 

//     return(
//         <form className='flex gap-2' id = "search">
//             <input type = 'text' placeholder ='..Search..' className = 'input input-bordered rounded-full' id ="searchInput"/>
//             <button type='submit' className='btn btn-circle bg-transparent text-white' id = "enter">
//             <IoSearch className="searchBar"/>
//             </button>

//         </form>
//     )
// }
// export default SearchInput