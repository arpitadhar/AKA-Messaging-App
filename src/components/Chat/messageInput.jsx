import { BsSend } from "react-icons/bs"; 
const MessageInput = () => {
  return(
    <form className='inputMessage'>
        <div className = 'messageInput'>
            <input type = "text" placeholder = 'Send a message'></input>
            <botton type = "submit"></botton>
            <BsSend className="sendButton"/> 
        </div>
    </form>
  )

}
export default MessageInput; 