import useConversation from "../../hooks/useConversation";

const Message = ({input_message}) => {
   const user1 = localStorage.getItem("email"); 
   const user = user1.replace(/^"(.*)"$/, '$1');
   const {selectedConversation} = useConversation(); 
   const fromMe = input_message.username === user; 
   const chatClassName = fromMe ? 'chat-end' : 'chat-start';
   console.log(input_message.username);
   return(
    <div>
    <div className={`chat ${chatClassName}`}>
        <div className='chat-image avatar'>
            <div className='w-10 rounded-full'>
            <span>{input_message.username}</span>
            </div>
        </div>  
        <div className={`chat-bubble text-white bg-blue-500`} id = 'messagesContainer'>{input_message.message}</div>
        <div className='chat-footer opacity-50 text-xs flex gap-1 items-center'>12:42</div>
    </div>
   </div>
   )
}
export default Message; 