import Message from "./message"
import useGetMessages from "../../hooks/useGetMessages";
import MessageSkeleton from "./MessageSkeleton";
const Messages = () => {
    const {messages, loading} = useGetMessages(); 
    console.log("messages:", messages); 
    messages.map((mess) => console.log(mess)); 
    return(
        <div className='px-4 flex-1 overflow-auto'>
          {loading && <MessageSkeleton />}
          {!loading && messages.length > 0 && messages.map((message) => (
            <Message key={message.id} input_message ={message} />
          ))} 
          {!loading && messages.length === 0 && (
            <p className='text-center'>Send a message to start chatting</p>
          )}
            
        </div>
    )
}

export default Messages