import Message from "./message"
import useGetMessages from "../../hooks/useGetMessages";
import MessageSkeleton from "./MessageSkeleton";

const Messages = () => {
    const {messages, loading, setMessages} = useGetMessages(); 
    
    console.log("messages:", messages); 
    
    messages.map((mess) => console.log("Messages(pt2):  ", mess)); 

    return(
        <div className='px-4 flex-1 overflow-auto'>
          {loading && <MessageSkeleton />}
          {!loading && messages.length > 0 && messages.map((message) => (
            <Message key={message.user} input_message ={message} />
          ))} 
          {!loading && messages.length === 0 && (
            <p className='text-center'>Send a message to start chatting</p>
          )}
            
        </div>
    )
}

export default Messages