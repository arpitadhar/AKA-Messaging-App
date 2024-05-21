import { BsSend } from "react-icons/bs"; 
import { useState } from "react";
import useSendMessage from "../../hooks/useSendMessage";

const MessageInput = ({socket}) => {
  const [message, setMessage] = useState(""); 
  const {loading, sendMessage} = useSendMessage(socket);
  const handleSubmit = async(e) => {
    e.preventDefault(); 
    if(!message) return; 
    await sendMessage(message); 
    setMessage(""); 
  };
  console.log("messageInput.jsx: ", message);

  return(
    <form className='inputMessage' onSubmit={handleSubmit}>
        <div className = 'messageInput'>
            <input type = "text" placeholder = 'Send a message' id = "message" value = {message} onChange={(e) => setMessage(e.target.value)}/>
            <button type = "submit" className = 'absolute inset-y-0 end-0 flex items-center pe-3'><BsSend className="sendButton"/></button>
        </div>
    </form>
  )

}
export default MessageInput; 