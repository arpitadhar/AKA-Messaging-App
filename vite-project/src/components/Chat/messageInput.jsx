import { BsSend } from "react-icons/bs"; 
import { useEffect, useState } from "react";
import useSendMessage from "../../hooks/useSendMessage";
const MessageInput = () => {
  const [message, setMessage] = useState(""); 
  const {loading, sendMessage} = useSendMessage();
  const handleSubmit = async(e) => {
    e.preventDefault(); 
    if(!message) return; 
    await sendMessage(message); 
    setMessage(""); 
  };
  console.log(message);

  return(
    <form className='inputMessage' onSubmit={handleSubmit}>
        <div className = 'messageInput'>
            <input type = "text" placeholder = 'Send a message' id = "message" value = {message} onChange={(e) => setMessage(e.target.value)}/>
            <button type = "submit" className = 'absolute inset-y-0 end-0 flex items-center pe-3' id = "send-message"><BsSend className="sendButton"/></button>
        </div>
    </form>
  )

}
export default MessageInput; 